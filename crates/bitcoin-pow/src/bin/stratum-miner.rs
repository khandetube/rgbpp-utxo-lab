use bitcoin::hashes::Hash;
use bitcoin_pow::stratum::{configure_version_rolling, connect_stratum, mine_stratum_job_cancelable_with_version, next_extranonce2, send_request, subscribe_and_authorize, MinerStats, StratumConfig, StratumJob};
use num_bigint::BigUint;
use serde_json::Value;
use std::collections::HashSet;
use std::env;
use std::io::{BufRead, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoints=env::var("STRATUM_URLS").ok().map(|v| v.split(',').map(str::trim).filter(|s|!s.is_empty()).map(str::to_owned).collect::<Vec<_>>()).filter(|v|!v.is_empty()).unwrap_or_else(|| vec![env::var("STRATUM_URL").unwrap_or_else(|_| "solo.ckpool.org:3333".into())]);
    let username=env::var("STRATUM_USERNAME").unwrap_or_else(|_| "bc1qrwhe5l4wvx86g6rs4n3tpyr85j0xr3cs6lex4d".into());
    let password=env::var("STRATUM_PASSWORD").unwrap_or_else(|_| "x".into());
    let threads=env::var("MINER_THREADS").ok().and_then(|v|v.parse().ok()).unwrap_or_else(||std::thread::available_parallelism().map(|n|n.get()).unwrap_or(1));
    let reconnect_secs=env::var("STRATUM_RECONNECT_SECS").ok().and_then(|v|v.parse().ok()).unwrap_or(5u64);

    println!("Bitcoin Stratum V1 miner");
    println!("endpoints={}",endpoints.join(","));
    println!("payout username={username}");
    println!("threads={threads}");
    println!("Live job rollover is enabled; stale work is cancelled.");
    println!("Protocol: Stratum V1 + optional BIP310 version rolling.");
    println!("Uses an external operator-controlled machine; it is not a GitHub-hosted miner.");

    let mut endpoint_index=0usize;
    loop {
        let endpoint=&endpoints[endpoint_index % endpoints.len()];
        match run_session(endpoint,&username,&password,threads) {
            Ok(()) => println!("server requested reconnect: {endpoint}"),
            Err(e) => eprintln!("session error on {endpoint}: {e}"),
        }
        endpoint_index=endpoint_index.wrapping_add(1);
        println!("reconnecting in {reconnect_secs}s...");
        thread::sleep(Duration::from_secs(reconnect_secs));
    }
}

fn run_session(endpoint:&str, username:&str, password:&str, threads:usize) -> Result<(),Box<dyn std::error::Error>> {
    let config=StratumConfig{endpoint:endpoint.to_string(),connect_timeout:Duration::from_secs(10)};
    let mut stream=connect_stratum(&config)?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))?;
    println!("connected to {endpoint}");
    let version_mask=env::var("VERSION_ROLLING_MASK").ok().and_then(|v| u32::from_str_radix(v.trim_start_matches("0x"),16).ok()).unwrap_or(0x1fffe000);
    let version_min_bits=env::var("VERSION_ROLLING_MIN_BITS").ok().and_then(|v| v.parse().ok()).unwrap_or(2u64);
    let (mut session,mut reader)=subscribe_and_authorize(&mut stream,username,password)?;
    let configure_id=10u64;
    let negotiated_mask=configure_version_rolling(&mut stream,&mut reader,configure_id,version_mask,version_min_bits).unwrap_or(None);
    println!("BIP310 version rolling mask={}",negotiated_mask.map(|m| format!("{m:08x}")).unwrap_or_else(|| "disabled".into()));
    session.version_mask=negotiated_mask.unwrap_or(0);
    println!("authorized; extranonce1={} extranonce2_size={} version_mask={:08x}",hex::encode(&session.extranonce1),session.extranonce2_size,session.version_mask);

    let (tx,rx)=mpsc::channel::<Result<Value,String>>();
    thread::spawn(move || {
        let mut reader=reader;
        loop {
            let mut line=String::new();
            match reader.read_line(&mut line) {
                Ok(0) => { let _=tx.send(Err("server closed connection".into())); break; }
                Ok(_) => match serde_json::from_str(line.trim()) {
                    Ok(value) => { if tx.send(Ok(value)).is_err() { break; } }
                    Err(e) => { let _=tx.send(Err(format!("invalid JSON from server: {e}"))); break; }
                },
                Err(e) => { let _=tx.send(Err(format!("reader error: {e}"))); break; }
            }
        }
    });

    let session=Arc::new(std::sync::RwLock::new(session));
    let stats=Arc::new(MinerStats{hashes:std::sync::atomic::AtomicU64::new(0),accepted:std::sync::atomic::AtomicU64::new(0),rejected:std::sync::atomic::AtomicU64::new(0)});
    let started=Instant::now();
    let mut counter=0u128;
    let mut share_target: Option<BigUint>=None;
    let mut pending_job: Option<(StratumJob,Vec<u8>)>=None;
    let mut pending_submissions=HashSet::<u64>::new();
    let mut active_version_bits=0u32;
    let mut request_id=1000u64;

    loop {
        let (job,extranonce2)=if let Some(item)=pending_job.take() {
            item
        } else {
            loop {
                let msg=rx.recv().map_err(|_| "reader channel closed")?;
                let msg=msg.map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
                match handle_control_message(&msg,&mut stream,&session,&mut share_target,&mut counter,&mut request_id,&mut pending_submissions)? {
                    ControlAction::NewJob(job_item) => break job_item,
                    ControlAction::RestartMining => continue,
                    ControlAction::None => account_submission(&msg,&mut pending_submissions,&stats),
                }
            }
        };

        let Some(target)=share_target.clone() else {
            println!("job received before mining.set_difficulty; waiting for pool share target");
            continue;
        };
        let cancel=Arc::new(AtomicBool::new(false));
        let mine_cancel=Arc::clone(&cancel);
        let mine_stats=Arc::clone(&stats);
        let mine_session=session.read().map_err(|_| "session lock poisoned")?.clone();
        let mine_job=job.clone();
        let mine_target=target.clone();
        let mine_extranonce2=extranonce2.clone();
        let live_mask=session.read().map_err(|_| "session lock poisoned")?.version_mask;
        let rolled_bits=if live_mask != 0 { active_version_bits=next_version_bits(active_version_bits,live_mask); Some(active_version_bits) } else { None };
        let handle=thread::spawn(move || mine_stratum_job_cancelable_with_version(&mine_job,&mine_session,mine_extranonce2,threads,&mine_stats,&mine_target,&mine_cancel,rolled_bits));

        loop {
            match rx.recv_timeout(Duration::from_millis(250)) {
                Ok(Err(e)) => return Err(e.into()),
                Ok(Ok(msg)) => {
                    match handle_control_message(&msg,&mut stream,&session,&mut share_target,&mut counter,&mut request_id,&mut pending_submissions)? {
                        ControlAction::NewJob((new_job,new_extranonce2)) => {
                            cancel.store(true,Ordering::Release);
                            pending_job=Some((new_job,new_extranonce2));
                            break;
                        }
                        ControlAction::RestartMining => {
                            cancel.store(true,Ordering::Release);
                            break;
                        }
                        ControlAction::None => account_submission(&msg,&pending_submissions,&stats),
                    }
                    if handle.is_finished() { break; }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    if handle.is_finished() { break; }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => return Err("reader channel disconnected".into()),
            }
        }
        if !handle.is_finished() { cancel.store(true,Ordering::Release); }
        let solved=handle.join().map_err(|_| "mining worker panicked")?;
        let elapsed=started.elapsed().as_secs_f64().max(0.001);
        let hashes=stats.hashes.load(Ordering::Relaxed);
        println!("hashrate {:.2} H/s; session hashes={}",hashes/elapsed,hashes);
        if let Some((nonce,hash))=solved {
            let network_target=job.target()?;
            let is_block=network_target.is_met_by(hash);
            if is_block {
                println!("NETWORK BLOCK CANDIDATE: hash={} job={} extranonce2={} nonce={:08x} ntime={}",hash,job.job_id,hex::encode(&extranonce2),nonce,hex::encode(&job.ntime));
            } else {
                println!("SHARE FOUND: hash={} job={} extranonce2={} nonce={:08x} ntime={}",hash,job.job_id,hex::encode(&extranonce2),nonce,hex::encode(&job.ntime));
            }
            let id=request_id;
            request_id=request_id.wrapping_add(1);
            send_request(&mut stream,id,"mining.submit",serde_json::json!(if let Some(bits)=rolled_bits { [serde_json::json!(username),serde_json::json!(job.job_id),serde_json::json!(hex::encode(&extranonce2)),serde_json::json!(hex::encode(&job.ntime)),serde_json::json!(format!("{nonce:08x}")),serde_json::json!(format!("{bits:08x}"))] } else { [serde_json::json!(username),serde_json::json!(job.job_id),serde_json::json!(hex::encode(&extranonce2)),serde_json::json!(hex::encode(&job.ntime)),serde_json::json!(format!("{nonce:08x}"))] }))?;
            pending_submissions.insert(id);
        }
    }
}

#[derive(Debug)]
enum ControlAction {
    None,
    NewJob((StratumJob, Vec<u8>)),
    RestartMining,
}

fn account_submission(msg:&Value,pending:&mut HashSet<u64>,stats:&MinerStats) {
    if let Some(id)=msg["id"].as_u64() {
        if pending.remove(&id) {
            if msg["result"]==Value::Bool(true) {
                stats.accepted.fetch_add(1,Ordering::Relaxed);
                println!("SUBMISSION ACCEPTED id={id}");
            } else {
                stats.rejected.fetch_add(1,Ordering::Relaxed);
                println!("SUBMISSION REJECTED id={id}: {msg}");
            }
        }
    }
}

fn handle_control_message(
    msg:&Value,
    stream:&mut std::net::TcpStream,
    session:&Arc<std::sync::RwLock<bitcoin_pow::stratum::StratumSession>>,
    share_target:&mut Option<BigUint>,
    counter:&mut u128,
    request_id:&mut u64,
    pending_submissions:&mut HashSet<u64>,
) -> Result<ControlAction,Box<dyn std::error::Error>> {
    match msg["method"].as_str() {
        Some("mining.set_difficulty") => {
            let difficulty=msg["params"].get(0).ok_or("set_difficulty missing value")?;
            *share_target=Some(StratumJob::share_target_from_difficulty(difficulty)?);
            println!("pool share difficulty changed: {difficulty}; waiting for next job before applying it");
        }
        Some("mining.set_version_mask") => {
            let mask_text=msg["params"].get(0).and_then(Value::as_str).ok_or("set_version_mask missing mask")?;
            let mask=u32::from_str_radix(mask_text,16)?;
            let mut guard=session.write().map_err(|_| "session lock poisoned")?;
            guard.version_mask=mask;
            println!("pool changed version rolling mask={mask:08x}; applying to subsequent work");
            Ok(ControlAction::RestartMining)
        }
        Some("mining.set_extranonce") => {
            let params=msg["params"].as_array().ok_or("set_extranonce params missing")?;
            let extranonce1=params.get(0).and_then(Value::as_str).ok_or("set_extranonce missing extranonce1")?;
            let size=params.get(1).and_then(Value::as_u64).ok_or("set_extranonce missing size")? as usize;
            let mut guard=session.write().map_err(|_| "session lock poisoned")?;
            guard.extranonce1=hex::decode(extranonce1)?;
            guard.extranonce2_size=size;
            *counter=0;
            println!("pool changed extranonce1={} extranonce2_size={size}; waiting for next job",hex::encode(&guard.extranonce1));
        }
        Some("mining.ping") => {
            let id=msg["id"].as_u64().ok_or("mining.ping id missing")?;
            let response=serde_json::to_string(&serde_json::json!({"id":id,"result":true,"error":null}))?;stream.write_all(response.as_bytes())?;stream.write_all(b"\n")?;stream.flush()?;
        }
        Some("client.show_message") => println!("pool message: {}",msg["params"].get(0).unwrap_or(&Value::Null)),
        Some("client.reconnect") => return Err("pool requested reconnect".into()),
        Some("mining.notify") => {
            let params=msg["params"].as_array().ok_or("notify params missing")?;
            let job=StratumJob::from_notify(params)?;
            let size=session.read().map_err(|_| "session lock poisoned")?.extranonce2_size;
            let extranonce2=next_extranonce2(*counter,size)?;
            *counter=counter.wrapping_add(1);
            println!("new {} job={} clean={}",if job.clean_jobs {"clean"} else {"non-clean"},job.job_id,job.clean_jobs);
            return Ok(ControlAction::NewJob((job,extranonce2)));
        }
        _ => {}
    }
    Ok(ControlAction::None)
}

fn next_version_bits(current:u32,mask:u32)->u32 { current.wrapping_add(1) & mask }
