# Funding Submission Package — RGB++ UTXO Lab

> Prepared for factual ecosystem outreach. This is a submission package, not a claim of approval, investment, sponsorship or partnership.

## Project

**RGB++ UTXO Lab**  
Repository: https://github.com/khandetube/rgbpp-utxo-lab

**One-line objective:** provide reproducible Rust + TypeScript infrastructure for Bitcoin UTXO, CKB Cell and RGB++ xUDT transaction construction, validation and testnet verification.

## Evidence already available

- Public Rust workspace covering UTXO models, deterministic selection, CKB Cell models, transaction builders and CKB RPC/indexer integration.
- TypeScript RGB++ xUDT transfer pipeline using the maintained CKB CCC/RGB++ stack.
- Fail-closed Bitcoin Testnet3 enforcement for the RGB++ path.
- Broadcast disabled by default and protected by an explicit confirmation gate.
- Public RGB++ Testnet3 preflight has been executed successfully against the live public testnet.
- Rust and TypeScript CI are maintained as separate reproducible checks.
- Public interactive demo: https://khandetube.github.io/rgbpp-utxo-lab/
- No fabricated production, customer, funding, transaction or adoption claims.

## Current technical gate

The remaining end-to-end milestone is a real, independently verifiable RGB++ Testnet3 transfer.

It requires only legitimate operator-controlled testnet resources:

1. BTC Testnet3 funding.
2. CKB Testnet/Pudge funding.
3. A real RGB++ xUDT type identifier.
4. A verified recipient BTC Testnet3 address.
5. Required RGB++ testnet API credentials/origin.
6. Execution through the repository's guarded workflow.

Private keys must stay in GitHub Actions Secrets or another operator-controlled secret manager. They must never be committed or pasted into chat.

## Proposed Spark scope

**Request:** up to USD 2,000 equivalent, subject to the program's current rules and committee decision.

### Milestone 1 — transaction hardening
- edge-case tests
- input/configuration validation
- failure-mode diagnostics
- deterministic build and verification tooling

### Milestone 2 — live testnet proof
- one real RGB++ Testnet3 transfer
- real Bitcoin TXID
- real CKB TXID
- independently verifiable records
- public verification report

### Milestone 3 — developer experience
- reproducible onboarding
- live verification runbook
- regression fixtures
- interactive technical demo

### Milestone 4 — ecosystem validation
- technical feedback from CKB/RGB++ builders
- integration opportunities with wallets/explorers/tooling
- next-phase scope based on evidence rather than speculative traction

## Proposed budget

| Workstream | Amount |
|---|---:|
| RGB++ transaction engineering and edge cases | $850 |
| Testnet verification, fixtures and QA | $400 |
| Documentation and developer onboarding | $350 |
| Security/failure-mode testing | $250 |
| Demo and ecosystem outreach | $150 |
| **Total** | **$2,000** |

The exact amount and currency remain subject to the specific funding program's decision. Recent Spark approvals have been paid in CKB rather than USD.

## Funding routes

### CKB Spark / ecosystem

Primary route because the project directly uses CKB Cells, CCC and RGB++ infrastructure.

Application should be made only after the live-transfer evidence gate is closed and the applicant identity/contact details are supplied.

### CKB Community Fund DAO

Follow-on route for a broader infrastructure scope after technical delivery and ecosystem demand are demonstrated.

### Tether Developer Grants / Bounties

Conditional route. Tether's current developer portal lists open-stack code, documentation/onboarding, applications, research, tooling and integrations and advertises rewards in USD₮ or Sats. This project should only apply when a concrete open task or integration maps to that stack.

### Bitcoin open-source funding

Conditional route. Any proposal must describe the actual Bitcoin/UTXO contribution accurately and must not present CKB/RGB++ work as Bitcoin Core work.

## Acceptance / evidence standard

Every claim in an application should be backed by one of:

- public source code and commit history;
- green CI run;
- public preflight output;
- real testnet transaction;
- independently verifiable transaction explorer/RPC record;
- written committee decision;
- independently verifiable on-chain funding transaction.

## What is not claimed

At the time of this document:

- no investor commitment;
- no grant approval;
- no customer contract;
- no production deployment;
- no revenue;
- no fabricated testnet transaction;
- no fabricated TXID;
- no mainnet broadcast.

## Submission checklist

- [ ] Add applicant Discord/email/contact details.
- [ ] Complete legitimate BTC Testnet3 and CKB Testnet funding.
- [ ] Complete the real RGB++ transfer.
- [ ] Independently verify both transaction records.
- [ ] Commit the verification report.
- [ ] Submit the Spark proposal through the official current process.
- [ ] Track written review/decision.
- [ ] Record funding only after written approval.
- [ ] Record received crypto funding only after independently verifying the on-chain payment.

