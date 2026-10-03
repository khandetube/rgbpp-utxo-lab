# Real CKB transfer

This example uses the official ckb-sdk transaction-building and unlocking interfaces.

## Safety

The private key is read only from the environment. It is never written to the repository, logs, or source code.

The default mode is **build only**. Broadcasting requires `CKB_BROADCAST=true`.

Required environment variables:

- `CKB_SENDER_PRIVATE_KEY`: 32-byte hex private key for the sender.
- `CKB_RECEIVER_ADDRESS`: CKB testnet address.
- `CKB_TRANSFER_AMOUNT`: amount in CKB, default `61`.
- `CKB_FEE_RATE`: fee-rate parameter, default `1000`.
- `CKB_RPC_URL`: RPC endpoint, default `https://testnet.ckb.dev`.
- `CKB_BROADCAST`: `false` by default.

Build without broadcasting:

    CKB_SENDER_PRIVATE_KEY=... CKB_RECEIVER_ADDRESS=ckt1... cargo run -p ckb-rpc --example transfer_testnet

Broadcast only after independently verifying sender balance, receiver address, network, amount and fee:

    CKB_SENDER_PRIVATE_KEY=... CKB_RECEIVER_ADDRESS=ckt1... CKB_BROADCAST=true cargo run -p ckb-rpc --example transfer_testnet

**Broadcast safety:** when `CKB_BROADCAST=true`, the implementation accepts only the official CKB Testnet RPC `https://testnet.ckb.dev`. Build-only mode may use a custom/local RPC. This prevents an accidental mainnet broadcast through an overridden RPC endpoint.

The implementation follows the official SDK pattern using CapacityTransferBuilder, CapacityBalancer, DefaultCellCollector, dependency resolvers, and SecpSighashUnlocker.
