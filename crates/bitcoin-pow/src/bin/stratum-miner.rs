use bitcoin::hashes::Hash;
use bitcoin_pow::stratum::{connect_stratum, mine_stratum_job, next_extranonce2, send_request, subscribe_and_authorize, MinerStats, StratumConfig, StratumJob};
use num_bigint::BigUint;
use serde_json::Value;
use std::env;
use std::io::BufRead;
use std::sync::atomic::Ordering;
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
        std::thread::sleep(Duration::from_secs(reconnect_secs));
    }
}

fn run_session(endpoint:&str, username:&str, password:&str, threads:usize) -> Result<(),Box<dyn std::error::Error>> {
    let config=StratumConfig{endpoint:endpoint.to_string(),connect_timeout:Duration::from_secs(10)};
    let mut stream=connect_stratum(&config)?;
    stream.set_read_timeout(Some(Duration::from_secs(120)))?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))?;
    println!("connected to {endpoint}");
    let (mut session,mut reader)=subscribe_and_authorize(&mut stream,username,password)?;
    println!("authorized; extranonce1={} extranonce2_size={}",hex::encode(&session.extranonce1),session.extranonce2_size);
    let stats=MinerStats{hashes:std::sync::atomic::AtomicU64::new(0),accepted:std::sync::atomic::AtomicU64::new(0),rejected:std::sync::atomic::AtomicU64::new(0)};
    let started=Instant::now();
    let mut counter=0u128;
    let mut share_target: Option<BigUint> = None;
    loop {
        let mut line=String::new();
        if reader.read_line(&mut line)?==0 { return Err("server closed connection".into()); }
        let msg:Value=serde_json::from_str(line.trim())?;
        if msg["method"]=="mining.set_difficulty" {
            let difficulty = msg["params"].get(0).ok_or("set_difficulty missing value")?;
            let target = StratumJob::share_target_from_difficulty(difficulty)?;
            println!("pool share difficulty changed: {difficulty}; exact share target={target:x}");
            share_target = Some(target);
            continue;
        }
        if msg["method"]=="mining.set_extranonce" {
            let params = msg["params"].as_array().ok_or("set_extranonce params missing")?;
            let extranonce1 = params.get(0).and_then(Value::as_str).ok_or("set_extranonce missing extranonce1")?;
            let size = params.get(1).and_then(Value::as_u64).ok_or("set_extranonce missing size")? as usize;
            session.extranonce1 = hex::decode(extranonce1)?;
            session.extranonce2_size = size;
            counter = 0;
            println!("pool changed extranonce1={} extranonce2_size={size}", hex::encode(&session.extranonce1));
            continue;
        }
        if msg["method"]=="client.show_message" {
            println!("pool message: {}",msg["params"].get(0).unwrap_or(&Value::Null));
            continue;
        }
        if msg["method"]=="client.reconnect" { return Ok(()); }
        if msg["method"]!="mining.notify" { continue; }
        let params=msg["params"].as_array().ok_or("notify params missing")?;
        let job=StratumJob::from_notify(params)?;
        let extranonce2=next_extranonce2(counter,session.extranonce2_size)?;
        counter=counter.wrapping_add(1);
        println!("new {} job={} clean={}",if job.clean_jobs {"clean"} else {"non-clean"},job.job_id,job.clean_jobs);
        let Some(target) = share_target.as_ref() else {
            println!("job received before mining.set_difficulty; waiting for pool share target");
            continue;
        };
        let solved=mine_stratum_job(&job,&session,extranonce2.clone(),threads,&stats,target);
        let elapsed=started.elapsed().as_secs_f64().max(0.001);
        let hashes=stats.hashes.load(Ordering::Relaxed);
        println!("hashrate {:.2} H/s; session hashes={}",hashes/elapsed,hashes);
        if let Some((nonce,hash))=solved {
            let network_target = job.target()?;
            let is_block = network_target.is_met_by(hash);
            if is_block {
                println!("NETWORK BLOCK CANDIDATE: hash={} job={} extranonce2={} nonce={:08x} ntime={}",hash,job.job_id,hex::encode(&extranonce2),nonce,hex::encode(&job.ntime));
            } else {
                println!("SHARE FOUND: hash={} job={} extranonce2={} nonce={:08x} ntime={}",hash,job.job_id,hex::encode(&extranonce2),nonce,hex::encode(&job.ntime));
            }
            let id=1000+counter as u64;
            send_request(&mut stream,id,"mining.submit",serde_json::json!([username,job.job_id,hex::encode(&extranonce2),hex::encode(&job.ntime),format!("{nonce:08x}")]))?;
            loop {
                let mut response_line=String::new();
                if reader.read_line(&mut response_line)?==0 { return Err("server closed connection after submit".into()); }
                let response:Value=serde_json::from_str(response_line.trim())?;
                if response["id"].as_u64()==Some(id) {
                    if response["result"]==Value::Bool(true) { stats.accepted.fetch_add(1,Ordering::Relaxed); println!("SUBMISSION ACCEPTED"); }
                    else { stats.rejected.fetch_add(1,Ordering::Relaxed); println!("SUBMISSION REJECTED: {response}"); }
                    break;
                }
            }
        }
    }
}
