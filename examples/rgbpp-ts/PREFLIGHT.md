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