use ckb_rpc::collector::collect_cells_by_lock;
use serde_json::json;

fn main() {
    let mut args = std::env::args().skip(1);

    let code_hash = args.next().expect("missing code_hash");
    let hash_type = args.next().unwrap_or_else(|| "type".to_owned());
    let lock_args = args.next().expect("missing lock_args");

    let rpc_url = std::env::var("CKB_RPC_URL")
        .unwrap_or_else(|_| "https://testnet.ckb.dev".to_owned());

    let lock_script = json!({
        "code_hash": code_hash,
        "hash_type": hash_type,
        "args": lock_args
    });

    match collect_cells_by_lock(&rpc_url, lock_script, 100, None, false) {
        Ok(page) => {
            println!("cells={}", page.cells.len());
            for cell in page.cells {
                println!(
                    "{}:{} capacity={} shannons",
                    cell.tx_hash, cell.output_index, cell.capacity_shannons
                );
            }
            if let Some(cursor) = page.next_cursor {
                println!("next_cursor={cursor}");
            }
        }
        Err(error) => eprintln!("collection failed: {error:?}"),
    }
}
