use tx_builder::build_transfer;
use utxo_model::{OutPoint, Utxo};

fn main() {
    let utxos = vec![
        Utxo { outpoint: OutPoint { txid: "a1".into(), vout: 0 }, value: 70_000 },
        Utxo { outpoint: OutPoint { txid: "b2".into(), vout: 1 }, value: 45_000 },
        Utxo { outpoint: OutPoint { txid: "c3".into(), vout: 0 }, value: 25_000 },
    ];
    let selection = build_transfer(utxos, 100_000, 2_000).expect("funds available");
    println!("selected_inputs={}");
    for input in &selection.inputs { println!("  {}:{} -> {}", input.outpoint.txid, input.outpoint.vout, input.value); }
    println!("total={}", selection.total);
    println!("change={}", selection.change);
}
