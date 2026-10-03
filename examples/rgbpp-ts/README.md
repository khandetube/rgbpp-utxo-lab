# Real RGB++ xUDT transfer integration

This directory is a TypeScript integration boundary around the maintained RGB++ SDK. The current upstream organization publishes the SDK as version 0.7.3 packages, and its own examples use the same `Collector`, `DataSource`, `BtcAssetsApi`, `buildRgbppTransferTx`, PSBT signing, and RGB++ queue flow used here.

## Setup

Install Node 21+ and pnpm 9+, then run:

    cd examples/rgbpp-ts
    pnpm install

Copy `.env.example` to `.env` and provide your own testnet values.

Important inputs are real protocol data: funded BTC UTXOs, an existing RGB++ xUDT type args, RGB++ lock args, and a valid RGB++ service token. No fake transaction IDs or asset identifiers are included.

## Build only

The default execution builds the RGB++ CKB virtual transaction and the Bitcoin PSBT, then prints the PSBT and commitment. It does not sign or broadcast.

    pnpm transfer

## Broadcast

Only after independently checking the PSBT, recipient, amount, network and fee, set:

    BROADCAST=true

The guarded broadcast path currently supports P2WPKH. It signs the PSBT, broadcasts the Bitcoin transaction through the configured RGB++ service, submits the resulting BTC txid plus CKB virtual transaction to the RGB++ queue, and polls for completion.

The upstream RGB++ examples document the same lifecycle: construct the CKB virtual transaction, construct/sign the BTC transaction, submit the BTC txid and virtual result to the RGB++ CKB queue, and wait for processing.

This example intentionally does not contain or persist private keys. `.env` must never be committed.
