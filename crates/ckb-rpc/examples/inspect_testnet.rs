use ckb_rpc::inspect_node;

fn main() {
    let rpc_url = std::env::var("CKB_RPC_URL")
        .unwrap_or_else(|_| "https://testnet.ckb.dev".to_string());

    match inspect_node(&rpc_url) {
        Ok(info) => println!("CKB RPC: {}\nTip block: {}", info.rpc_url, info.tip_block_number),
        Err(error) => eprintln!("CKB RPC request failed: {:?}", error),
    }
}
