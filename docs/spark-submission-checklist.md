# Spark Program Submission Checklist

Use this checklist immediately before posting the Spark application.

## Evidence gate

- [ ] Rust CI is green on the latest commit.
- [ ] TypeScript CI is green on the latest commit.
- [ ] Public RGB++ preflight succeeds.
- [ ] Private preflight succeeds with operator-controlled testnet credentials.
- [ ] Bitcoin Testnet3 wallet is funded through a legitimate faucet/source.
- [ ] CKB Testnet/Pudge wallet is funded through a legitimate faucet/source.
- [ ] The exact RGB++ xUDT type args are recorded.
- [ ] Receiver address is independently checked.
- [ ] Amount and fee are independently checked.
- [ ] First live transfer completes.
- [ ] Real BTC TXID is recorded.
- [ ] Real CKB TXID is recorded.
- [ ] Both records are independently verifiable.
- [ ] Verification report is committed to the repository.

## Application package

- [ ] Use the proposal in `docs/spark-proposal-draft.md`.
- [ ] Replace contact placeholders with the applicant's real contact details.
- [ ] Link the public repository and verification report.
- [ ] State only completed work as completed.
- [ ] Keep the requested amount and milestones consistent with the current Spark rules.
- [ ] Explain the scope complexity if requesting more than $1,000.
- [ ] Do not claim approval before the committee approves it.

## Post-submission

- [ ] Track committee questions in the public application thread.
- [ ] Reply with reproducible evidence, not marketing claims.
- [ ] Keep weekly progress synchronized with actual commits and testnet records.
- [ ] Record any approved funding only after the official approval post.
- [ ] Record received CKB only after the actual on-chain payment is independently verified.
