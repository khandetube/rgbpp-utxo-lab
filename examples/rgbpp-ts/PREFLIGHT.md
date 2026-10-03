# RGB++ Testnet Preflight

Run the example in three stages.

## 1. Build only

    pnpm install
    pnpm run typecheck
    pnpm run transfer

The default configuration does not broadcast.

## 2. Verify external configuration

Use only Bitcoin Testnet3 and CKB testnet accounts controlled by you. The UDT code hash, type arguments, cell dependency and RGB++ API access must belong to the real testnet asset you intend to move.

Never commit wallet credentials or API tokens.

## 3. Broadcast

After independently checking the receiver, asset, amount and fee:

    RGBPP_BROADCAST=true
    RGBPP_CONFIRM_TESTNET_BROADCAST=YES
    pnpm run transfer

The code rejects networks other than Bitcoin Testnet3 and requires the explicit confirmation before broadcast.

The final end-to-end transaction still depends on funded testnet accounts, a real RGB++ testnet xUDT and valid RGB++ testnet API access.

## xUDT configuration

The example now resolves the deployed CKB `KnownScript.XUdt` and its cell dependency through the CKB client. You only need the **unique xUDT type arguments** for the real RGB++ asset.

The RGB++ official UDT documentation describes the transfer flow as: build the CKB partial transaction, build/sign the BTC PSBT, inject the BTC transaction ID, construct the RGB++ witness, then submit the final CKB transaction.

For Meepo Testnet, the official RGB++ resources publish the deployed RGB++ script parameters and the Testnet3 BTC-assets API endpoints.


## Automated preflight

From this directory:

    pnpm install
    pnpm run typecheck
    pnpm run preflight

The preflight connects to CKB Testnet, reads the current tip, resolves the deployed xUDT known-script dependency, derives the controlled Bitcoin Testnet3 sender address, and validates required non-secret configuration. It does not sign or broadcast either transaction.
