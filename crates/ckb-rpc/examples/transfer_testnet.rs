fn main() {
    if let Err(error) = ckb_rpc::transfer::run_env_transfer() {
        eprintln!("transfer failed: {error:#}");
        std::process::exit(1);
    }
}
