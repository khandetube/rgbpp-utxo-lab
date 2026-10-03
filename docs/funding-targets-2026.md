# 2026 Funding & Ecosystem Targets

This document tracks funding routes that are technically relevant to RGB++ UTXO Lab. It is a research and outreach plan, not a claim that any program has accepted, funded, endorsed, or contacted the project.

## 1. CKB Eco Fund — Spark Program

**Fit:** highest-priority ecosystem route.

CKB's Spark Program explicitly supports digital-sovereign infrastructure and primitives, including developer tooling and RGB++ innovations. The public program describes a 1–2 month validation-oriented cycle with milestone funding and public progress tracking.

Current 2026 operating guidance:
- Recent Spark applications show a small, milestone-defined scope is expected; do not assume a generic investment round.
- The August 2026 CKB ecosystem update states that new Spark applicants should create the project repository under the GitHub account used for identity verification (for multi-applicant teams, the primary applicant creates it and others fork it).
- Applications and progress are publicly discussed on Nervos Talk with the `Spark-Program` tag.
- A reproducible "How to Verify" section remains part of the evidence standard for this project.
- Recent 2026 Spark examples have been paid in CKB; the exact amount and payment terms must be confirmed in the specific approval thread before treating them as committed funding.
- Current ecosystem activity includes new/pending Spark projects, so this route is active but competitive and not guaranteed.

**Our application evidence:**
- public Rust + TypeScript repository
- deterministic CI
- CKB RPC/indexer integration
- RGB++ xUDT transfer path
- public testnet preflight
- guarded private testnet transfer workflow
- explicit security and no-fabrication policy

**Next evidence gate:** one independently verifiable RGB++ Testnet3 transfer.

## 2. CKB Community Fund DAO / CKB ecosystem

**Fit:** strategic follow-on route after a validated Spark milestone.

The CKB ecosystem uses the Community Fund DAO for broader ecosystem funding. A proposal should be based on demonstrated technical delivery, ecosystem demand and a concrete scale-up scope rather than a generic investment request.

Potential follow-on scope:
- reusable transaction/verification infrastructure
- wallet/explorer integrations
- hosted verification or orchestration services
- ecosystem developer onboarding
- maintenance and security hardening

## 3. Tether Developer Grants

**Fit:** conditional.

Tether announced a developer grants program in May 2026 for open technology work, including wallet infrastructure, browser extensions and e-commerce/payment integrations. The announcement says active tasks can pay approximately $1,500–$4,000 in USD₮ or Bitcoin.

The current RGB++ UTXO Lab is not automatically eligible merely because it is a crypto project. Before applying, identify a concrete Tether-stack task or integration that is genuinely compatible with this repository.

**Potential angle:** payment/transaction infrastructure only if a real Tether task maps cleanly to the project's UTXO/transaction tooling.

## 4. OpenSats

**Fit:** conditional Bitcoin open-source route.

OpenSats states that developers and contributors working on Bitcoin, Nostr and adjacent freedom technology can apply on a rolling basis. Its spring 2026 focus was Bitcoin base-layer privacy.

This repository is Bitcoin/UTXO infrastructure, but the proposal should only be submitted if the scope is clearly aligned with an OpenSats funding objective. Do not present RGB++/CKB work as Bitcoin Core work.

## 5. Evidence package required for every application

Before requesting funds, keep the evidence package factual:

1. public repository URL
2. green Rust and TypeScript CI
3. public preflight result
4. real Testnet3 RGB++ transaction, once executed
5. BTC TXID and CKB TXID, independently verifiable
6. documented asset/type identifier
7. reproducible commands
8. explicit security model
9. milestone budget
10. post-grant maintenance plan

## 6. Funding integrity

No funding application should claim:
- investors who have not committed;
- customers who have not signed;
- users who do not exist;
- revenue that has not been received;
- partnerships that have not been agreed;
- transaction IDs that have not occurred;
- production deployments that have not been independently demonstrated.

Real crypto funding will be recorded only after an actual on-chain payment is received and independently verifiable.


## 7. Outreach status — October 2026

### Priority A — Spark / CKB ecosystem
**Action:** prepare the submission package around one independently verifiable RGB++ Testnet3 transfer and a narrowly scoped 4-week milestone plan.

**Public evidence to present:**
- repository: https://github.com/khandetube/rgbpp-utxo-lab
- architecture and security docs
- green CI runs
- public RGB++ preflight
- real Testnet3 transfer evidence once completed

**Do not submit yet as a funded/approved project.** The repository currently has no verified grant commitment.

### Priority B — CKB Community Fund DAO
Use this route for a broader follow-on proposal after technical evidence and ecosystem demand are demonstrated. The official CKB developer resources point applicants to the Community Fund DAO rules and process.

### Priority C — Tether Developer Grants / Bounties
Tether's current developer portal explicitly advertises grants and bounties paid in USD₮ or Sats. The portal's listed focus areas are QVAC, MOS, WDK, Pears, documentation/onboarding, applications on Tether's stack, research, and tooling/integrations/open standards. RGB++ UTXO Lab should only apply when a concrete task or integration genuinely matches that scope; a generic request for crypto funding is not sufficient.

### Outreach evidence ledger
Record only:
- date contacted
- official application URL/thread
- exact requested amount and currency
- requested deliverables
- reply/status
- committed amount only after written confirmation
- on-chain payment only after independently verifying the transaction

Never record an investor, sponsor, grant, partnership, user, or payment as real before there is evidence.
