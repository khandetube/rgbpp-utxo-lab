# Architecture

## Layers

1. Domain layer: utxo-model and ckb-cell-model.
2. Transaction layer: tx-builder composes domain primitives and accounts for fees.
3. Integration layer: CKB RPC client, live-cell collection, network-aware transaction assembly.
4. Protocol layer: RGB++ integration only through verified upstream SDK and protocol interfaces.

The separation is intentional: the core remains testable without a node, wallet, network credentials, or private key.
