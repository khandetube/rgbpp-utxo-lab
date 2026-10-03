use bitcoin_pow::double_sha256;
use std::time::{Duration, Instant};

fn main() {
    let seconds: u64 = std::env::var("BENCH_SECONDS").ok().and_then(|v| v.parse().ok()).unwrap_or(5);
    let threads: usize = std::env::var("BENCH_THREADS").ok().and_then(|v| v.parse().ok()).unwrap_or_else(|| std::thread::available_parallelism().map(usize::from).unwrap_or(1));
    let deadline = Instant::now() + Duration::from_secs(seconds.max(1));
    let total = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    std::thread::scope(|scope| {
        for worker in 0..threads {
            let total=std::sync::Arc::clone(&total);
            scope.spawn(move || {
                let mut n=worker as u64;
                let mut header=[0u8;80];
                while Instant::now() < deadline {
                    header[76..80].copy_from_slice(&(n as u32).to_le_bytes());
                    let _=double_sha256(&header);
                    total.fetch_add(1,std::sync::atomic::Ordering::Relaxed);
                    n=n.wrapping_add(threads as u64);
                }
            });
        }
    });
    let hashes=total.load(std::sync::atomic::Ordering::Relaxed);
    println!("sha256d_hashes={hashes}");
    println!("threads={threads}");
    println!("elapsed_seconds={seconds}");
    println!("hashrate_hps={:.2}", hashes as f64 / seconds.max(1) as f64);
}
