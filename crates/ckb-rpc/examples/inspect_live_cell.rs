use ckb_rpc::get_live_cell;

fn main() {
    let mut args = std::env::args().skip(1);

    let tx_hash = args.next().unwrap_or_else(|| {
        eprintln!(
            "usage: cargo run -p ckb-rpc --example inspect_live_cell -- <tx_hash> <output_index>"
        );
        std::process::exit(2);
    });

    let output_index = args
        .next()
        .unwrap_or_else(|| "0".to_string())
        .parse::<u32>()
        .unwrap_or_else(|_| {
            eprintln!("output_index must be a non-negative integer");
            std::process::exit(2);
        });

    let rpc_url = std::env::var("CKB_RPC_URL")
        .unwrap_or_else(|_| "https://testnet.ckb.dev".to_string());

    match get_live_cell(&rpc_url, &tx_hash, output_index, true) {
        Ok(response) => {
            println!("RPC: {rpc_url}");
            println!("{}", serde_json::to_string_pretty(&response).unwrap());
        }
        Err(error) => eprintln!("CKB live-cell query failed: {error:?}"),
    }
}
