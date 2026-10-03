use crate::double_sha256;
use bitcoin::hashes::Hash;
use bitcoin::pow::{CompactTarget, Target};
use bitcoin::BlockHash;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct StratumJob {
    pub job_id: String,
    pub prevhash: Vec<u8>,
    pub coinb1: Vec<u8>,
    pub coinb2: Vec<u8>,
    pub merkle_branch: Vec<Vec<u8>>,
    pub version: Vec<u8>,
    pub nbits: Vec<u8>,
    pub ntime: Vec<u8>,
    pub clean_jobs: bool,
}

#[derive(Debug, Clone)]
pub struct StratumSession {
    pub extranonce1: Vec<u8>,
    pub extranonce2_size: usize,
}

#[derive(Debug, Clone)]
pub struct StratumConfig {
    pub endpoint: String,
    pub connect_timeout: Duration,
}

#[derive(Debug)]
pub enum StratumError {
    Io(std::io::Error),
    Protocol(String),
    Json(serde_json::Error),
    Hex(hex::FromHexError),
}

impl std::fmt::Display for StratumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Protocol(e) => write!(f, "Stratum protocol error: {e}"),
            Self::Json(e) => write!(f, "JSON error: {e}"),
            Self::Hex(e) => write!(f, "hex error: {e}"),
        }
    }
}
impl std::error::Error for StratumError {}
impl From<std::io::Error> for StratumError { fn from(e: std::io::Error) -> Self { Self::Io(e) } }
impl From<serde_json::Error> for StratumError { fn from(e: serde_json::Error) -> Self { Self::Json(e) } }
impl From<hex::FromHexError> for StratumError { fn from(e: hex::FromHexError) -> Self { Self::Hex(e) } }

impl StratumJob {
    pub fn from_notify(params: &[Value]) -> Result<Self, StratumError> {
        if params.len() < 9 { return Err(StratumError::Protocol("mining.notify has fewer than 9 parameters".into())); }
        let s = |i: usize| params[i].as_str().ok_or_else(|| StratumError::Protocol(format!("notify parameter {i} is not a string")));
        let hexv = |i: usize| -> Result<Vec<u8>, StratumError> { Ok(hex::decode(s(i)?)?) };
        let merkle = params[4].as_array().ok_or_else(|| StratumError::Protocol("merkle branch is not an array".into()))?
            .iter().map(|v| {
                let x=v.as_str().ok_or_else(|| StratumError::Protocol("merkle branch entry is not a string".into()))?;
                Ok(hex::decode(x)?)
            }).collect::<Result<Vec<_>,StratumError>>()?;
        Ok(Self {
            job_id: s(0)?.to_owned(),
            prevhash: hexv(1)?,
            coinb1: hexv(2)?,
            coinb2: hexv(3)?,
            merkle_branch: merkle,
            version: hexv(5)?,
            nbits: hexv(6)?,
            ntime: hexv(7)?,
            clean_jobs: params[8].as_bool().unwrap_or(true),
        })
    }

    pub fn target(&self) -> Result<Target, StratumError> {
        if self.nbits.len() != 4 { return Err(StratumError::Protocol("nBits must be 4 bytes".into())); }
        let bits = u32::from_be_bytes(self.nbits.as_slice().try_into().unwrap());
        Ok(Target::from_compact(CompactTarget::from_consensus(bits)))
    }

    pub fn share_target_from_difficulty(value: &Value) -> Result<BigUint, StratumError> {
        let text = value.as_str()
            .map(str::to_owned)
            .or_else(|| value.as_f64().map(|v| format!("{v:.18}")))
            .ok_or_else(|| StratumError::Protocol("mining.set_difficulty target is not numeric".into()))?;
        let (whole, frac) = text.split_once('.').unwrap_or((&text, ""));
        if whole.is_empty() || whole.starts_with('-') {
            return Err(StratumError::Protocol("difficulty must be positive".into()));
        }
        let digits = format!("{whole}{frac}");
        let numerator = BigUint::parse_bytes(digits.as_bytes(), 10)
            .ok_or_else(|| StratumError::Protocol("invalid difficulty".into()))?;
        let scale = BigUint::from(10u32).pow(frac.len() as u32);
        if numerator == BigUint::from(0u8) {
            return Err(StratumError::Protocol("difficulty must be greater than zero".into()));
        }
        let diff1 = BigUint::from_bytes_be(&hex::decode(
            "00000000ffff0000000000000000000000000000000000000000000000000000"
        )?);
        Ok((diff1 * scale) / numerator)
    }

    pub fn hash_meets_share_target(hash: &[u8; 32], target: &BigUint) -> bool {
        // SHA256d returns the digest in internal byte order. Stratum/Bitcoin
        // targets are compared against the displayed (big-endian) hash value.
        let mut displayed = *hash;
        displayed.reverse();
        BigUint::from_bytes_be(&displayed) <= *target
    }

    fn coinbase(&self, session: &StratumSession, extranonce2: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.coinb1.len()+session.extranonce1.len()+extranonce2.len()+self.coinb2.len());
        out.extend_from_slice(&self.coinb1);
        out.extend_from_slice(&session.extranonce1);
        out.extend_from_slice(extranonce2);
        out.extend_from_slice(&self.coinb2);
        out
    }

    fn merkle_root(&self, coinbase_hash: [u8;32]) -> [u8;32] {
        let mut root = coinbase_hash;
        for branch in &self.merkle_branch {
            let mut data = Vec::with_capacity(64);
            data.extend_from_slice(&root);
            data.extend_from_slice(branch);
            root = double_sha256(&data);
        }
        root
    }

    pub fn header_prefix(&self, session: &StratumSession, extranonce2: &[u8]) -> Result<[u8;80], StratumError> {
        if self.prevhash.len()!=32 || self.version.len()!=4 || self.nbits.len()!=4 || self.ntime.len()!=4 {
            return Err(StratumError::Protocol("invalid header field length in mining.notify".into()));
        }
        let coinbase_hash = double_sha256(&self.coinbase(session,&extranonce2));
        let root = self.merkle_root(coinbase_hash);
        let mut header=[0u8;80];
        for i in 0..4 {
            header[i]=self.version[3-i];
            header[68+i]=self.ntime[3-i];
            header[72+i]=self.nbits[3-i];
        }
        for i in 0..32 {
            header[4+i]=self.prevhash[31-i];
            header[36+i]=root[31-i];
        }
        Ok(header)
    }

    pub fn nonce_header(prefix: &[u8;80], nonce: u32) -> [u8;80] {
        let mut header=*prefix;
        header[76..80].copy_from_slice(&nonce.to_le_bytes());
        header
    }
}

pub fn connect_stratum(config: &StratumConfig) -> Result<TcpStream, StratumError> {
    let mut addrs=config.endpoint.to_socket_addrs().map_err(StratumError::Io)?;
    let addr=addrs.next().ok_or_else(|| StratumError::Protocol("endpoint resolved to no addresses".into()))?;
    Ok(TcpStream::connect_timeout(&addr, config.connect_timeout)?)
}

pub fn send_request(stream: &mut TcpStream, id: u64, method: &str, params: Value) -> Result<(), StratumError> {
    let line=serde_json::to_string(&json!({"id":id,"method":method,"params":params}))?;
    stream.write_all(line.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;
    Ok(())
}

pub fn subscribe_and_authorize(
    stream: &mut TcpStream,
    username: &str,
    password: &str,
) -> Result<(StratumSession, BufReader<TcpStream>), StratumError> {
    send_request(stream,1,"mining.subscribe",json!(["rgbpp-utxo-lab/stratum-v1"]))?;
    let mut reader=BufReader::new(stream.try_clone()?);
    let response=next_matching_response(&mut reader,1)?;
    let result=response["result"].as_array().ok_or_else(|| StratumError::Protocol(format!("subscribe rejected: {response}")))?;
    let extranonce1=result.get(1).and_then(Value::as_str).ok_or_else(|| StratumError::Protocol("subscribe missing extranonce1".into()))?;
    let extranonce2_size=result.get(2).and_then(Value::as_u64).ok_or_else(|| StratumError::Protocol("subscribe missing extranonce2 size".into()))? as usize;
    let session=StratumSession { extranonce1:hex::decode(extranonce1)?, extranonce2_size };
    send_request(stream,2,"mining.authorize",json!([username,password]))?;
    let auth=next_matching_response(&mut reader,2)?;
    if auth["result"] != Value::Bool(true) { return Err(StratumError::Protocol(format!("authorization rejected: {auth}"))); }
    Ok((session,reader))
}

fn next_matching_response(reader: &mut BufReader<TcpStream>, id: u64) -> Result<Value, StratumError> {
    loop {
        let mut line=String::new();
        if reader.read_line(&mut line)? == 0 { return Err(StratumError::Protocol("server closed connection".into())); }
        let value:Value=serde_json::from_str(line.trim())?;
        if value["id"].as_u64()==Some(id) {
            if !value["error"].is_null() { return Err(StratumError::Protocol(format!("request rejected: {value}"))); }
            return Ok(value);
        }
    }
}

pub struct MinerStats {
    pub hashes: AtomicU64,
    pub accepted: AtomicU64,
    pub rejected: AtomicU64,
}

pub fn mine_stratum_job(
    job: &StratumJob,
    session: &StratumSession,
    extranonce2: Vec<u8>,
    threads: usize,
    stats: &MinerStats,
    share_target: &BigUint,
) -> Option<(u32, BlockHash)> {
    let cancel = AtomicBool::new(false);
    mine_stratum_job_cancelable(job, session, extranonce2, threads, stats, share_target, &cancel)
}

pub fn mine_stratum_job_cancelable(
    job: &StratumJob,
    session: &StratumSession,
    extranonce2: Vec<u8>,
    threads: usize,
    stats: &MinerStats,
    share_target: &BigUint,
    cancel: &AtomicBool,
) -> Option<(u32, BlockHash)> {
    let prefix=job.header_prefix(session,&extranonce2).ok()?;
    let target=job.target().ok()?;
    let found=Arc::new(AtomicBool::new(false));
    let result=Arc::new(std::sync::Mutex::new(None));
    let threads=threads.max(1);
    let stats_ref=&stats;
    thread::scope(|scope| {
        for worker in 0..threads {
            let found=Arc::clone(&found);
            let result=Arc::clone(&result);
            let prefix=prefix;
            scope.spawn(move || {
                let mut nonce=worker as u32;
                let stride=threads as u32;
                loop {
                    if found.load(Ordering::Relaxed) || cancel.load(Ordering::Relaxed) { break; }
                    let header=StratumJob::nonce_header(&prefix,nonce);
                    let digest=double_sha256(&header);
                    stats_ref.hashes.fetch_add(1, Ordering::Relaxed);
                    let hash=BlockHash::from_byte_array(digest);
                    if target.is_met_by(hash) || StratumJob::hash_meets_share_target(&digest, share_target) {
                        if !found.swap(true,Ordering::AcqRel) {
                            *result.lock().unwrap()=Some((nonce,hash));
                        }
                        break;
                    }
                    let next=nonce.wrapping_add(stride);
                    if next < nonce { break; }
                    nonce=next;
                }
            });
        }
    });
    if cancel.load(Ordering::Relaxed) { return None; }
    result.lock().unwrap().take()
}

pub fn next_extranonce2(counter: u128, size: usize) -> Result<Vec<u8>, StratumError> {
    if size==0 || size>16 { return Err(StratumError::Protocol("unsupported extranonce2 size; expected 1..=16 bytes".into())); }
    let max_bits=(size*8) as u32;
    if max_bits<128 && counter >= (1u128<<max_bits) { return Err(StratumError::Protocol("extranonce2 counter exhausted".into())); }
    let bytes=counter.to_be_bytes();
    Ok(bytes[16-size..].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extranonce2_is_fixed_width_big_endian() {
        assert_eq!(hex::encode(next_extranonce2(0, 4).unwrap()), "00000000");
        assert_eq!(hex::encode(next_extranonce2(1, 4).unwrap()), "00000001");
        assert_eq!(hex::encode(next_extranonce2(0x1234, 4).unwrap()), "00001234");
    }

    #[test]
    fn extranonce2_rejects_invalid_width() {
        assert!(next_extranonce2(0, 0).is_err());
        assert!(next_extranonce2(0, 17).is_err());
    }

    #[test]
    fn nonce_is_little_endian_in_header_tail() {
        let prefix = [0u8; 80];
        let header = StratumJob::nonce_header(&prefix, 0x12345678);
        assert_eq!(&header[76..80], &[0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn share_target_uses_displayed_hash_byte_order() {
        let target = BigUint::from_bytes_be(&[0xff; 32]);
        let mut digest = [0u8; 32];
        digest[0] = 1;
        assert!(StratumJob::hash_meets_share_target(&digest, &target));

        let target = BigUint::from(1u8);
        let mut digest = [0u8; 32];
        digest[31] = 1;
        assert!(StratumJob::hash_meets_share_target(&digest, &target));
        digest[31] = 2;
        assert!(!StratumJob::hash_meets_share_target(&digest, &target));
    }

    #[test]
    fn notify_parsing_and_target_conversion_are_consistent() {
        let params = vec![
            Value::String("job-1".into()),
            Value::String("00".repeat(32)),
            Value::String("aa".into()),
            Value::String("bb".into()),
            Value::Array(vec![]),
            Value::String("01000000".into()),
            Value::String("207fffff".into()),
            Value::String("65000000".into()),
            Value::Bool(true),
        ];
        let job = StratumJob::from_notify(&params).unwrap();
        assert_eq!(job.job_id, "job-1");
        assert_eq!(job.prevhash.len(), 32);
        assert_eq!(job.target(), Target::from_compact(CompactTarget::from_consensus(0x207fffff)));
        let share = StratumJob::share_target_from_difficulty(&Value::String("1".into())).unwrap();
        assert_eq!(share, BigUint::from_bytes_be(&hex::decode("00000000ffff0000000000000000000000000000000000000000000000000000").unwrap()));
    }
}
