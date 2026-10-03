# CKB RPC Adapter

This crate is the live-network boundary of the RGB++ UTXO Lab.

It uses the official Rust CKB SDK to:

- read the current CKB tip block;
- query a specific cell by transaction hash and output index;
- keep network I/O isolated from the deterministic domain models.

The SDK currently used by this project is **ckb-sdk 5.1.0**. The official SDK exposes CKB RPC operations including tip queries and live-cell queries.

## Examples

Inspect the testnet tip:

```bash
cargo run -p ckb-rpc --example inspect_testnet
```

Inspect a real cell:

```bash
cargo run -p ckb-rpc --example inspect_live_cell -- <tx_hash> <output_index>
```

Override the endpoint when required:

```bash
CKB_RPC_URL=https://testnet.ckb.dev cargo run -p ckb-rpc --example inspect_live_cell -- <tx_hash> 0
```

The RGB++ SDK documentation also demonstrates the CKB testnet RPC endpoint and separates the Bitcoin/RGB++ workflow from the CKB transaction side.

No private keys, signing, or transaction broadcasting are performed by these examples.
