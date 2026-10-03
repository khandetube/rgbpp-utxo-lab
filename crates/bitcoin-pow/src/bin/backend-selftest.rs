use bitcoin_pow::sha256d_backend::{CpuSha256dBackend, HeaderWork, Sha256dBackend};

fn main() {
    let backend = CpuSha256dBackend;
    let mut header = [0u8; 80];
    header[0] = 1;
    header[76..80].copy_from_slice(&42u32.to_le_bytes());

    let target = [0xffu8; 32];
    let result = backend
        .search(HeaderWork::with_nonce_range(header, 42, 1), &target)
        .expect("easy target must produce a result");

    assert_eq!(result.nonce, 42);
    println!("backend=cpu");
    println!("nonce={}", result.nonce);
    println!("hash={}", hex::encode(result.hash));
    println!("target_check=PASS");
}
