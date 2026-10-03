# Live Testnet Verification Report

> Fill this document only after a real, independently verifiable Testnet3 transfer. Never enter placeholder or fabricated transaction IDs.

## Execution

- Date/time (UTC):
- Git commit:
- Network: Bitcoin Testnet3 + CKB Testnet/Pudge
- Operator:
- Workflow run URL:
- Transfer amount:
- Fee rate:

## Asset

- RGB++ xUDT type args:
- Asset/unique ID (if separately exposed by the service):
- Source used to verify the asset:

## Addresses

- Sender BTC Testnet3 address:
- Receiver BTC Testnet3 address:
- Sender CKB Testnet address (if applicable):

## Bitcoin transaction

- BTC TXID:
- Explorer URL:
- Independently verified: [ ] yes
- Verification notes:

## CKB transaction

- CKB TXID:
- Explorer URL:
- Independently verified: [ ] yes
- Verification notes:

## Result

- BTC transaction confirmed: [ ] yes [ ] no
- CKB transaction confirmed: [ ] yes [ ] no
- RGB++ asset state independently verified: [ ] yes [ ] no
- Receiver state independently verified: [ ] yes [ ] no

## Reproduction

Commands used:

```text
cd examples/rgbpp-ts
pnpm install
pnpm run typecheck
pnpm run preflight
pnpm run transfer
```

For a broadcast execution, record the manual workflow inputs and the corresponding GitHub Actions run.

## Evidence standard

A report is publishable only when the BTC TXID and CKB TXID are real, publicly queryable, correspond to the stated testnet network, and independently verify the claimed transfer. If any prerequisite fails, document the failure instead of converting it into a success claim.
