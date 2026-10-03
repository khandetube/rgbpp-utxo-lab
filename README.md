# RGB++ UTXO Lab

A runnable, test-driven blockchain engineering laboratory focused on Bitcoin-style UTXOs, CKB Cells, transaction construction, and RGB++ workflow modeling.

> Portfolio project — independent implementation.
> This repository is not an official Nervos/RGB++ implementation and does not claim client production experience.

## What this project demonstrates

- Rust workspace architecture
- Deterministic Bitcoin-style UTXO selection and change calculation
- CKB Cell modeling based on the documented Cell model
- Transaction validation invariants with unit tests
- Explicit separation between protocol-independent domain logic and network adapters
- A path for later integration with real CKB RPC/testnet tooling

## Why these primitives

CKB uses a Cell model in which state is represented by transaction outputs, with lock and type scripts governing authorization and state validity. Its official ecosystem provides Rust SDKs, RPC tooling, testnet infrastructure, and development examples.

This project deliberately starts with a fully local, deterministic core so that every example can run without private keys, funded wallets, or a live node.

## Project status

### Implemented in this first milestone

- UTXO domain types
- deterministic UTXO selection
- fee/change accounting
- CKB Cell domain types
- capacity conservation checks
- transaction-level validation
- executable examples
- automated tests

### Planned integration milestones

1. CKB JSON-RPC adapter
2. real testnet cell collection
3. signed transaction construction
4. integration tests against a local/dev CKB node
5. carefully scoped RGB++ workflow adapter using verified upstream protocol APIs

No protocol-specific API is fabricated in this repository.

## Verification

Run: cargo test --workspace
Run: cargo run -p tx-builder --example transfer

The core examples are deterministic and do not require network access.

## References

- CKB official documentation: https://docs.nervos.org/
- CKB source: https://github.com/nervosnetwork/ckb
- CKB RFC 0002: https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0002-ckb/0002-ckb.md
- CKB developer resources: https://github.com/ckb-devrel/CKB-Developer-Resource

## Engineering principles

- No fabricated protocol claims
- No fake transaction IDs
- No fake client work
- Reproducible local tests
- Small, reviewable modules
- Explicit security boundaries
## Operational milestones

- Rust UTXO and deterministic selection
- CKB Cell modeling and capacity invariants
- CKB RPC/indexer integration
- Official CKB SDK transaction construction and guarded testnet broadcast path
- rust-bitcoin outpoint primitives
- RGB++ SDK 0.7.3 TypeScript integration for real xUDT transfer construction, PSBT generation, guarded BTC broadcast, and RGB++ queue submission

The final end-to-end RGB++ execution requires user-supplied, funded testnet BTC/CKB accounts, an actual RGB++ xUDT asset, live RGB++ lock args/UTXOs, and a valid service token. Those are external chain state and secrets, so the repository never invents or stores them.
