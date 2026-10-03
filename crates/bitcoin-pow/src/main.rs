use bitcoin::consensus::deserialize;
use bitcoin::pow::Target;
use bitcoin::BlockHash;

const TESTNET_API: &str = "https://mempool.space/testnet/api";

fn get_text(url: &str) -> String {
    reqwest::blocking::get(url)
        .unwrap_or_else(|e| panic!("request failed for {url}: {e}"))
        .error_for_status()
        .unwrap_or_else(|e| panic!("HTTP error for {url}: {e}"))
        .text()
        .unwrap_or_else(|e| panic!("response body failed for {url}: {e}"))
}

fn main() {
    let height: u64 = get_text(&format!("{TESTNET_API}/blocks/tip/height"))
        .trim()
        .parse()
        .expect("invalid Testnet3 tip height");

    let block_hash_hex = get_text(&format!("{TESTNET_API}/block-height/{height}"))
        .trim()
        .to_owned();

    let header_hex = get_text(&format!("{TESTNET_API}/block/{block_hash_hex}/header"))
        .trim()
        .to_owned();

    let header_bytes = hex::decode(&header_hex).expect("invalid header hex");
    assert_eq!(
        header_bytes.len(),
        80,
        "Bitcoin block header must be 80 bytes"
    );

    let header: bitcoin::block::Header =
        deserialize(&header_bytes).expect("failed to decode Bitcoin block header");

    let calculated_hash = header.block_hash();
    let reported_hash: BlockHash = block_hash_hex.parse().expect("invalid reported block hash");

    let target = Target::from_compact(header.bits);
    let pow_valid = target.is_met_by(header.block_hash());

    println!("network=BitcoinTestnet3");
    println!("height={height}");
    println!("block_hash={calculated_hash}");
    println!("reported_hash={reported_hash}");
    println!("header_hex={header_hex}");
    println!("version={}", header.version.to_consensus());
    println!("prev_blockhash={}", header.prev_blockhash);
    println!("merkle_root={}", header.merkle_root);
    println!("time={}", header.time);
    println!("bits={:#x}", header.bits.to_consensus());
    println!("nonce={}", header.nonce);
    println!("pow_valid={pow_valid}");

    assert_eq!(calculated_hash, reported_hash, "header hash mismatch");
    assert!(
        pow_valid,
        "real Testnet3 header does not satisfy its target"
    );
}
