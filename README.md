# RGB++ UTXO Lab

> Production-oriented Rust + TypeScript laboratory for Bitcoin UTXO, CKB cells, xUDT and RGB++ transaction workflows.

[![Rust CI](https://github.com/khandetube/rgbpp-utxo-lab/actions/workflows/ci.yml/badge.svg)](https://github.com/khandetube/rgbpp-utxo-lab/actions/workflows/ci.yml)
[![RGB++ TypeScript CI](https://github.com/khandetube/rgbpp-utxo-lab/actions/workflows/rgbpp-ts.yml/badge.svg)](https://github.com/khandetube/rgbpp-utxo-lab/actions/workflows/rgbpp-ts.yml)

## Why this project

A transparent, reproducible engineering reference for developers building Bitcoin/UTXO and CKB/RGB++ infrastructure in Rust and TypeScript.

**Current verification:** deterministic Rust tests and TypeScript type-checking run in GitHub Actions. Live testnet verification is deliberately separated from push CI and must use legitimate operator-controlled testnet funds and credentials.

## Core capabilities

- deterministic Bitcoin-style UTXO selection and parsing
- deterministic CKB cell selection and capacity accounting
- Rust CKB RPC/indexer integration
- real CKB Testnet/Pudge transfer builder with guarded broadcast
- RGB++ xUDT transfer pipeline on Bitcoin Testnet3
- fail-closed network, secret and broadcast validation
- public and private testnet preflight paths
- reproducible CI and live-testnet runbook

## RGB++ transfer path

`xUDT -> CKB partial transaction -> Bitcoin PSBT -> Bitcoin broadcast -> inject real BTC TXID -> RGB++ witness -> final CKB transaction`

The implementation follows the RGB++ SDK model; protocol-specific behavior is kept behind integration boundaries.

## Live testnet verification

See [docs/live-testnet-verification.md](docs/live-testnet-verification.md).

The project never fabricates transaction IDs, balances, assets, credentials or successful broadcasts. A real end-to-end claim is made only after a real Testnet3 transaction is independently verifiable.

## Project structure

- `crates/utxo-model` — Bitcoin-style UTXO domain model
- `crates/ckb-cell-model` — CKB cell and capacity model
- `crates/tx-builder` — deterministic transfer builders
- `crates/ckb-rpc` — CKB RPC/indexer and guarded transfer integration
- `examples/rgbpp-ts` — RGB++ TypeScript integration and preflight tooling
- `docs/` — architecture, security, protocol and live-testnet documentation

## Run locally

```bash
cargo test --workspace
cargo check --workspace

cd examples/rgbpp-ts
pnpm install
pnpm run typecheck
pnpm run preflight
```

For real testnet execution, use only Bitcoin Testnet3 and CKB Testnet/Pudge assets controlled by the operator. Never commit private keys.

## Portfolio note

This repository is an independent engineering project and does not claim to be an official Nervos/RGB++ implementation or prior client production work.
