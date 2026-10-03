# RGB++ UTXO Lab

> Production-oriented Rust + TypeScript laboratory for Bitcoin UTXO, CKB cells, xUDT and RGB++ transaction workflows.

[![Rust CI](https://github.com/khandetube/rgbpp-utxo-lab/actions/workflows/ci.yml/badge.svg)](https://github.com/khandetube/rgbpp-utxo-lab/actions/workflows/ci.yml)
[![RGB++ TypeScript CI](https://github.com/khandetube/rgbpp-utxo-lab/actions/workflows/rgbpp-ts.yml/badge.svg)](https://github.com/khandetube/rgbpp-utxo-lab/actions/workflows/rgbpp-ts.yml)

## Why this project

A transparent, reproducible engineering reference for developers building Bitcoin/UTXO and CKB/RGB++ infrastructure in Rust and TypeScript.

**Current verification:** deterministic Rust tests and TypeScript type-checking run in GitHub Actions. Live testnet verification is deliberately separated from push CI and must use legitimate operator-controlled testnet funds and credentials.

## Infrastructure Engineering Profile

For infrastructure-focused review, see:

[Infrastructure Engineering Profile](docs/infrastructure-engineering-profile.md)

## Core capabilities

- deterministic Bitcoin-style UTXO selection and parsing
- deterministic CKB cell selection and capacity accounting
- Rust CKB RPC/indexer integration
- real CKB Testnet/Pudge transfer builder with guarded broadcast
- RGB++ xUDT transfer pipeline on Bitcoin Testnet3
- fail-closed network, secret and broadcast validation
- public and private testnet preflight paths
- guarded GitHub Actions workflow for reproducible Testnet3 execution
- reproducible CI and live-testnet runbook
- publication-ready live-testnet verification report template

## RGB++ transfer path

`xUDT -> CKB partial transaction -> Bitcoin PSBT -> Bitcoin broadcast -> inject real BTC TXID -> RGB++ witness -> final CKB transaction`

The implementation follows the RGB++ SDK model; protocol-specific behavior is kept behind integration boundaries.

## Live testnet verification

See [docs/live-testnet-verification.md](docs/live-testnet-verification.md).

The project never fabricates transaction IDs, balances, assets, credentials or successful broadcasts. A real end-to-end claim is made only after a real Testnet3 transaction is independently verifiable. The evidence format is documented in [docs/live-testnet-verification-report-template.md](docs/live-testnet-verification-report-template.md).

### GitHub Actions execution

Two manual workflows are provided:

1. **RGB++ Live Testnet Preflight** — public connectivity and protocol-script checks; no private keys and no broadcast.
2. **RGB++ Testnet Transfer** — private, operator-controlled execution. It runs typecheck and private preflight first. Broadcast is disabled unless the workflow input explicitly enables it **and** the operator types `YES` in the separate confirmation field.

The transfer workflow expects these GitHub Actions Secrets:

- `UTXO_BASED_CHAIN_PRIVATE_KEY`
- `UTXO_BASED_CHAIN_ADDRESS_TYPE`
- `CKB_SECP256K1_PRIVATE_KEY`
- `BTC_ASSETS_API_TOKEN`
- `BTC_ASSETS_API_ORIGIN`
- `UDT_TYPE_ARGS`
- `RGBPP_RECEIVER_BTC_ADDRESS`

Optional repository Variables:

- `RGBPP_TRANSFER_AMOUNT` (default `1`)
- `RGBPP_FEE_RATE` (default `28`)

Secrets must contain only operator-controlled **Bitcoin Testnet3 / CKB Testnet** credentials. Never use mainnet keys, leaked credentials, or third-party wallets.

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
