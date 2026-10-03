use bitcoin::hashes::Hash;
use bitcoin_pow::stratum::{connect_stratum, mine_stratum_job_cancelable, next_extranonce2, send_request, subscribe_and_authorize, MinerStats, StratumConfig, StratumJob};
use num_bigint::BigUint;
use serde_json::Value;
use std::collections::HashSet;
use std::env;
use std::io::BufRead;
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
    println!("Live job rollover is enabled; this miner cancels stale clean jobs instead of waiting for the nonce space to finish.");
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
    let (session,reader)=subscribe_and_authorize(&mut stream,username,password)?;
    println!("authorized; extranonce1={} extranonce2_size={}",hex::encode(&session.extranonce1),session.extranonce2_size);

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
    let mut request_id=1000u64;

    loop {
        let (job,extranonce2)=if let Some(item)=pending_job.take() {
            item
        } else {
            loop {
                let msg=rx.recv().map_err(|_| "reader channel closed")??;
                if let Some(job_item)=handle_control_message(&msg,&mut stream,&session,&mut share_target,&mut counter,&mut request_id,&mut pending_submissions)? {
                    break job_item;
                }
                if let Some(id)=msg["id"].as_u64() {
                    if pending_submissions.remove(&id) {
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
        let handle=thread::spawn(move || mine_stratum_job_cancelable(&mine_job,&mine_session,mine_extranonce2,threads,&mine_stats,&mine_target,&mine_cancel));

        loop {
            match rx.recv_timeout(Duration::from_millis(250)) {
                Ok(Err(e)) => return Err(e.into()),
                Ok(Ok(msg)) => {
                    if let Some((new_job,new_extranonce2))=handle_control_message(&msg,&mut stream,&session,&mut share_target,&mut counter,&mut request_id,&mut pending_submissions)? {
                        cancel.store(true,Ordering::Release);
                        pending_job=Some((new_job,new_extranonce2));
                        break;
                    }
                    if let Some(id)=msg["id"].as_u64() {
                        if pending_submissions.remove(&id) {
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
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => return Err("reader channel disconnected".into()),
            }
            if handle.is_finished() { break; }
        }
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
            send_request(&mut stream,id,"mining.submit",serde_json::json!([username,job.job_id,hex::encode(&extranonce2),hex::encode(&job.ntime),format!("{nonce:08x}")]))?;
            pending_submissions.insert(id);
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
) -> Result<Option<(StratumJob,Vec<u8>)>,Box<dyn std::error::Error>> {
    match msg["method"].as_str() {
        Some("mining.set_difficulty") => {
            let difficulty=msg["params"].get(0).ok_or("set_difficulty missing value")?;
            *share_target=Some(StratumJob::share_target_from_difficulty(difficulty)?);
            println!("pool share difficulty changed: {difficulty}; new jobs will use the new target");
        }
        Some("mining.set_extranonce") => {
            let params=msg["params"].as_array().ok_or("set_extranonce params missing")?;
            let extranonce1=params.get(0).and_then(Value::as_str).ok_or("set_extranonce missing extranonce1")?;
            let size=params.get(1).and_then(Value::as_u64).ok_or("set_extranonce missing size")? as usize;
            let mut guard=session.write().map_err(|_| "session lock poisoned")?;
            guard.extranonce1=hex::decode(extranonce1)?;
            guard.extranonce2_size=size;
            *counter=0;
            println!("pool changed extranonce1={} extranonce2_size={size}; current work will be discarded",hex::encode(&guard.extranonce1));
        }
        Some("mining.ping") => {
            let id=msg["id"].as_u64().ok_or("mining.ping id missing")?;
            send_request(stream,id,"mining.pong",Value::Array(vec![]))?;
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
            return Ok(Some((job,extranonce2)));
        }
        _ => {
            if let Some(id)=msg["id"].as_u64() {
                if pending_submissions.contains(&id) {
                    // Submission responses are accounted for by the caller.
                }
            }
        }
    }
    Ok(None)
}
