use bitcoin::block::Version;
use bitcoin::hashes::Hash;
use bitcoin::pow::{CompactTarget, Target};
use bitcoin::{BlockHash, TxMerkleNode};
use bitcoin_pow::{hash_rate, mine_header, verify_header};

fn main() {
    // Deliberately bounded test difficulty: this is a real Bitcoin-style
    // double-SHA256 PoW search, but it is not a mainnet mining target.
    let bits = CompactTarget::from_consensus(0x1f0f_ffff);
    let header = bitcoin::block::Header {
        version: Version::from_consensus(1),
        prev_blockhash: BlockHash::all_zeros(),
        merkle_root: TxMerkleNode::all_zeros(),
        time: 1_700_000_000,
        bits,
        nonce: 0,
    };
    let target = Target::from_compact(bits);

    let solution = mine_header(header, target);

    let mut solved = header;
    solved.nonce = solution.nonce;

    println!("pow=valid");
    println!("nonce={}", solution.nonce);
    println!("hash={}", solution.hash);
    println!("attempts={}", solution.attempts);
    println!("elapsed_ms={}", solution.elapsed.as_millis());
    println!("hashrate_hps={:.2}", hash_rate(solution.attempts, solution.elapsed));
    println!("verified={}", verify_header(&solved, target));
}
