# CKB RPC Adapter

This crate is the first live-network boundary in the project.

It uses the official `ckb-sdk` RPC client rather than implementing JSON-RPC manually. The adapter currently exposes a minimal, read-only operation: querying the current tip block number.

## Run

```bash
cargo run -p ckb-rpc --example inspect_testnet
```

Or select another endpoint:

```bash
CKB_RPC_URL="http://127.0.0.1:8114" cargo run -p ckb-rpc --example inspect_testnet
```

The default endpoint is the CKB testnet endpoint documented by the CKB SDK examples.

No transaction is signed or broadcast by this example.
