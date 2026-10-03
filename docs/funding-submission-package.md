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


## Verified live-chain evidence update — 2026-10-03

The Bitcoin PoW verification milestone is now reproducibly passing in public GitHub Actions.

- Rust CI run: https://github.com/khandetube/rgbpp-utxo-lab/actions/runs/37105866071
- Final commit: e73e79fea59b46df2a533fe465e76a820a8ee8f7
- Rust formatting, workspace tests and workspace check: passed.
- Live Bitcoin Testnet3 header verification: passed.
- Verified Testnet3 height during the run: 5,155,024.
- Calculated block hash matched the reported block hash: 0000000000076ae0db0656ada8b11a23673d8e3f5e5c43be993a6f5dba6ccc49.
- The decoded header satisfied its compact-target PoW check.

This evidence proves live Testnet3 header retrieval and independent PoW verification. It does **not** constitute a mined block, a funded wallet, an RGB++ asset issuance, an RGB++ transfer, funding approval, or investor commitment.

The next technical gate remains a real, independently verifiable RGB++ Testnet3 issuance/transfer using legitimate operator-controlled BTC Testnet3 and CKB Testnet resources.

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



## Current funding-environment update — 2026-10-03

The current Spark ecosystem shows active developer-tooling and infrastructure proposals. Recent committee decisions also demonstrate that delivery quality, real testnet verification and actual ecosystem integration are material acceptance criteria.

For this project, the funding request remains milestone-based and evidence-first. The repository identity should match the GitHub account used for any identity verification, and each milestone should point to reproducible public evidence.

### Current technical evidence gate

1. Green Rust and TypeScript CI.
2. Public live Testnet3 header/PoW verification.
3. Isolated Regtest block-construction and submission coverage.
4. Legitimate BTC Testnet3 and CKB Testnet resources.
5. One real RGB++ Testnet3 transfer.
6. Independently verifiable BTC and CKB transaction records.
7. Public verification report.

No funding, approval, investor commitment or payment is recorded until independently verified.

### Route priority

**CKB Spark / ecosystem funding:** directly aligned with the project's CKB/RGB++ developer-infrastructure scope. Submit only through the current official Spark process and use the verified GitHub identity associated with the applicant.

**Tether Developer Grants:** conditional secondary route. Apply only when a concrete current task genuinely matches the repository's Bitcoin/UTXO tooling.

**Bitcoin open-source funding:** conditional. Any application must describe the actual Bitcoin contribution accurately and must not present CKB/RGB++ work as Bitcoin Core development.

**CKB Community Fund DAO:** follow-on route after technical delivery and demonstrated ecosystem demand.

### Evidence discipline

Public source code, reproducible CI, live-chain records and independently verifiable transaction identifiers are the only acceptable evidence for completion claims. Simulated balances, hard-coded transaction identifiers and artificial adoption metrics are excluded.
