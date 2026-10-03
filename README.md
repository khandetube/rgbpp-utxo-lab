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


## Funding & ecosystem development

The project is also being prepared for transparent ecosystem funding and technical partnerships. The evidence-based funding brief is available at [docs/investor-ecosystem-brief.md](docs/investor-ecosystem-brief.md), with the current 2026 funding routes in [docs/funding-targets-2026.md](docs/funding-targets-2026.md) and the submission-ready package in [docs/funding-submission-package.md](docs/funding-submission-package.md).

The funding plan is milestone-based: real testnet verification, reproducible evidence, developer tooling and security hardening. No adoption, revenue, investor commitment or production deployment is claimed before it is independently demonstrated.

## Project structure

- `crates/utxo-model` — Bitcoin-style UTXO domain model
- `crates/ckb-cell-model` — CKB cell and capacity model
- `crates/tx-builder` — deterministic transfer builders
- `crates/ckb-rpc` — CKB RPC/indexer and guarded transfer integration
- `examples/rgbpp-ts` — RGB++ TypeScript integration and preflight tooling
- `docs/` — architecture, security, protocol and live-testnet documentation

## Real Bitcoin Stratum miner

The Rust crate also contains a standalone Stratum V1 miner for a machine or mining controller you are authorized to operate. It connects with `mining.subscribe` / `mining.authorize`, consumes `mining.notify` jobs, builds Bitcoin block headers and performs SHA256d proof-of-work, then submits a solved nonce with `mining.submit`. Stratum V1 defines this job/submit flow. citeturn1search0turn1search2

For a solo pool such as ckpool, the payout Bitcoin address is supplied as the Stratum username. citeturn0search0turn0search2

```bash
export STRATUM_URLS=solo.ckpool.org:3333
export STRATUM_USERNAME=bc1qrwhe5l4wvx86g6rs4n3tpyr85j0xr3cs6lex4d
export STRATUM_PASSWORD=x
export MINER_THREADS=$(nproc)
cargo run -p bitcoin-pow --bin stratum-miner --release
```

This is intentionally not a GitHub-hosted continuous mining workflow. Multiple explicitly authorized machines can use the same payout username to aggregate idle SHA256 capacity; see `docs/bitcoin-stratum-mining.md` for the real-machine runbook. No third-party or unconsented compute is used.

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
