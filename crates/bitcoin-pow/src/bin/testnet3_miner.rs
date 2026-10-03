use bitcoin_pow::template::{serialize_block, verify_candidate, BlockTemplate};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::env;
use std::time::{Duration, Instant};

fn env_required(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("missing required environment variable {name}"))
}

fn rpc(client: &Client, url: &str, user: &str, password: &str, method: &str, params: Value) -> Value {
    let response: Value = client
        .post(url)
        .basic_auth(user, Some(password))
        .json(&json!({"jsonrpc":"1.0","id":"rgbpp-utxo-lab","method":method,"params":params}))
        .send().unwrap_or_else(|e| panic!("RPC request failed: {e}"))
        .error_for_status().unwrap_or_else(|e| panic!("RPC HTTP error: {e}"))
        .json().unwrap_or_else(|e| panic!("RPC JSON decode failed: {e}"));
    if !response["error"].is_null() {
        panic!("Bitcoin RPC error: {}", response["error"]);
    }
    response["result"].clone()
}

fn main() {
    let rpc_url = env_required("BITCOIN_TESTNET_RPC_URL");
    let user = env_required("BITCOIN_TESTNET_RPC_USER");
    let password = env_required("BITCOIN_TESTNET_RPC_PASSWORD");
    let payout_hex = env::var("BITCOIN_MINER_SCRIPT_PUBKEY").ok();
    let payout = payout_hex.as_deref().map(|s| bitcoin::ScriptBuf::from_bytes(hex::decode(s).expect("invalid payout script hex")));
    let max_attempts: u64 = env::var("MAX_ATTEMPTS").unwrap_or_else(|_| "1000000".into()).parse().expect("invalid MAX_ATTEMPTS");
    let submit = env::var("BITCOIN_TESTNET_SUBMIT").as_deref() == Ok("YES");

    if submit && env::var("BITCOIN_TESTNET_CONFIRM").as_deref() != Ok("YES") {
        panic!("refusing submit: set BITCOIN_TESTNET_CONFIRM=YES");
    }

    let client = Client::builder().timeout(Duration::from_secs(30)).build().expect("client");
    let template_json = rpc(&client, &rpc_url, &user, &password, "getblocktemplate", json!([{"rules":["segwit"]}]));
    let template = BlockTemplate::from_json(&template_json).expect("invalid getblocktemplate response");

    println!("network=BitcoinTestnet3");
    println!("height={}", template.height);
    println!("previousblockhash={}", template.previous_blockhash);
    println!("tx_count={}", template.transactions.len());
    println!("bits={:#x}", template.bits.to_consensus());
    println!("target={:?}", template.target);
    println!("mintime={}", template.mintime);
    println!("curtime={}", template.curtime);

    let started = Instant::now();
    let mut attempts = 0u64;
    let mut extra_nonce = 0u64;
    let mut time = template.curtime.max(template.mintime);

    loop {
        let mut block = template.build_block(extra_nonce, time, 0, payout.as_ref());
        for nonce in 0..=u32::MAX {
            block.header.nonce = nonce;
            attempts += 1;
            if verify_candidate(&block, template.target) {
                let block_hex = serialize_block(&block);
                println!("pow_solution=ok");
                println!("nonce={nonce}");
                println!("extra_nonce={extra_nonce}");
                println!("time={time}");
                println!("block_hash={}", block.header.block_hash());
                println!("attempts={attempts}");
                println!("hashrate_hps={:.2}", attempts as f64 / started.elapsed().as_secs_f64().max(f64::MIN_POSITIVE));
                println!("block_hex={block_hex}");

                if submit {
                    let result = rpc(&client, &rpc_url, &user, &password, "submitblock", json!([block_hex]));
                    if result.is_null() {
                        println!("submitblock=accepted");
                    } else {
                        println!("submitblock=result={result}");
                    }
                } else {
                    println!("submitblock=skipped");
                }
                return;
            }
            if max_attempts != 0 && attempts >= max_attempts {
                println!("pow_solution=not_found");
                println!("attempts={attempts}");
                println!("submitblock=skipped");
                return;
            }
        }
        extra_nonce = extra_nonce.wrapping_add(1);
        time = time.saturating_add(1).max(template.mintime);
        if time > template.curtime.saturating_add(2 * 60 * 60) {
            time = template.curtime.max(template.mintime);
        }
    }
}
