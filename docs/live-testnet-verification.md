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

## 3. Real RGB++ transfer

The transfer example is intentionally build-only by default. It requires:

- a user-controlled Bitcoin Testnet3 private key/address
- a user-controlled CKB testnet private key
- a real RGB++ xUDT unique ID/type args
- the RGB++ testnet API token/origin required by the service
- a funded testnet receiver
- independently verified amount and fee settings

Set `RGBPP_BROADCAST=true` only after independently checking all recipient, asset, amount, fee, and network values. Also set `RGBPP_CONFIRM_TESTNET_BROADCAST=YES`.

The code rejects non-Testnet3 networks and blocks broadcast unless the explicit confirmation guard is present.

## 4. What counts as a real end-to-end verification

A real verification should record the actual:

1. Bitcoin Testnet3 transaction ID
2. resulting CKB transaction ID
3. xUDT unique ID/type args
4. sender and receiver testnet addresses
5. network and fee settings

No transaction ID or asset ID should ever be invented for documentation.

## 5. Current external prerequisites

The repository can verify public infrastructure without secrets, but a complete live transfer cannot be truthfully executed until the external prerequisites above exist. Those values are chain state and credentials, not source-code placeholders.

The implementation follows the documented RGB++ sequence: construct the partial CKB transaction, build/sign the Bitcoin transaction, submit Bitcoin, inject the Bitcoin transaction ID into the RGB++ CKB transaction, then sign and submit the final CKB transaction.
