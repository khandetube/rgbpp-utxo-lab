pub mod route_orchestrator;
pub mod sha256d_backend;
pub mod pool_supervisor;
pub mod stratum;
pub mod template;

use bitcoin::blockdata::block::Header;
use bitcoin::pow::Target;
use sha2::{Digest, Sha256};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowSolution {
    pub nonce: u32,
    pub hash: bitcoin::BlockHash,
    pub attempts: u64,
    pub elapsed: Duration,
}

pub fn double_sha256(data: &[u8]) -> [u8; 32] {
    let first = Sha256::digest(data);
    let second = Sha256::digest(first);
    second.into()
}

pub fn mine_header(mut header: Header, target: Target) -> PowSolution {
    let started = Instant::now();
    let mut attempts = 0u64;
    for nonce in 0..=u32::MAX {
        header.nonce = nonce;
        let hash = header.block_hash();
        attempts += 1;
        if target.is_met_by(hash) {
            return PowSolution { nonce, hash, attempts, elapsed: started.elapsed() };
        }
    }
    panic!("nonce space exhausted without finding a valid proof");
}

pub fn hash_rate(attempts: u64, elapsed: Duration) -> f64 {
    let seconds = elapsed.as_secs_f64();
    if seconds == 0.0 { return 0.0; }
    attempts as f64 / seconds
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::block::Version;
    use bitcoin::hashes::sha256d;
    use bitcoin::hashes::Hash;
    use bitcoin::{BlockHash, TxMerkleNode};

    #[test]
    fn double_sha256_matches_known_vector() {
        let digest = double_sha256(b"abc");
        let expected = sha256d::Hash::hash(b"abc").to_byte_array();
        assert_eq!(digest, expected);
    }

    #[test]
    fn mines_and_verifies_real_double_sha256_pow() {
        let header = Header {
            version: Version::from_consensus(1),
            prev_blockhash: BlockHash::all_zeros(),
            merkle_root: TxMerkleNode::all_zeros(),
            time: 1_700_000_000,
            bits: bitcoin::pow::CompactTarget::from_consensus(0x207f_ffff),
            nonce: 0,
        };
        let target = Target::from_compact(header.bits);
        let solution = mine_header(header, target);
        let mut solved = header;
        solved.nonce = solution.nonce;
        assert!(target.is_met_by(solved.block_hash()));
        assert_eq!(solved.block_hash(), solution.hash);
        assert!(solution.attempts > 0);
    }

    #[test]
    fn hash_rate_is_zero_for_zero_duration() {
        assert_eq!(hash_rate(100, Duration::ZERO), 0.0);
    }
}
