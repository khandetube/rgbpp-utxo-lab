# Live Testnet Verification

This repository separates deterministic CI from live network verification.

## 1. Public RGB++/CKB connectivity

From GitHub Actions, open **Actions → RGB++ Live Testnet Preflight → Run workflow**.

The workflow runs:

`pnpm run public-preflight`

It checks, without private keys and without broadcasting anything:

- Bitcoin Testnet3 RGB++ network configuration
- public CKB testnet connectivity
- current CKB testnet tip
- the deployed xUDT cell dependency
- RGB++ script information resolution

The public preflight has a 45-second timeout per network operation and the workflow itself has a 5-minute timeout.

## 2. Private-key preflight

The `preflight` command validates a funded testnet execution configuration without signing or broadcasting:

`pnpm run preflight`

Required environment variables are documented in `.env.example`.

**Never commit private keys or API tokens.** For CI, use GitHub Actions Secrets.

## 3. Guarded GitHub Actions transfer

The repository now includes **RGB++ Testnet Transfer** as a manual `workflow_dispatch` workflow.

It performs this sequence:

1. install the pinned TypeScript dependencies
2. run TypeScript typecheck
3. run private preflight
4. run the transfer example

The workflow requires these Actions Secrets:

- `UTXO_BASED_CHAIN_PRIVATE_KEY`
- `UTXO_BASED_CHAIN_ADDRESS_TYPE`
- `CKB_SECP256K1_PRIVATE_KEY`
- `BTC_ASSETS_API_TOKEN`
- `BTC_ASSETS_API_ORIGIN`
- `UDT_TYPE_ARGS`
- `RGBPP_RECEIVER_BTC_ADDRESS`

Optional repository Variables:

- `RGBPP_TRANSFER_AMOUNT`
- `RGBPP_FEE_RATE`

The workflow defaults to **no broadcast**. Bitcoin broadcast is enabled only when the manual `broadcast` input is true **and** the separate `confirm_broadcast` input is exactly `YES`. The application itself has the same fail-closed guard.

## 4. Real RGB++ transfer prerequisites

A truthful end-to-end transfer requires:

- a user-controlled Bitcoin Testnet3 private key/address with sufficient testnet BTC
- a user-controlled CKB testnet private key with sufficient CKB capacity
- a real RGB++ xUDT unique ID/type args
- the RGB++ testnet API token/origin required by the service
- a funded and independently verified testnet receiver
- independently verified amount and fee settings

The implementation rejects non-Testnet3 networks and blocks broadcast unless the explicit confirmation guard is present.

## 5. What counts as a real end-to-end verification

A real verification should record the actual:

1. Bitcoin Testnet3 transaction ID
2. resulting CKB transaction ID
3. xUDT unique ID/type args
4. sender and receiver testnet addresses
5. network and fee settings

No transaction ID, asset ID, balance, or successful broadcast should ever be invented for documentation.

## 6. Current external prerequisites

The repository can verify public infrastructure without secrets, but a complete live transfer cannot be truthfully executed until the external prerequisites above exist. Those values are chain state and credentials, not source-code placeholders.

The implementation follows the documented RGB++ sequence: construct the partial CKB transaction, build/sign the Bitcoin transaction, submit Bitcoin, inject the Bitcoin transaction ID into the RGB++ CKB transaction, then sign and submit the final CKB transaction.

This matches the current RGB++ documentation for xUDT transfer on Bitcoin Testnet3.