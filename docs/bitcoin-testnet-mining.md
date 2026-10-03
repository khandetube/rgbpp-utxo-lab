# Bitcoin Testnet3 block-template mining

The repository now contains a real block-construction path for an operator-controlled Bitcoin Testnet3 node:

1. Call Bitcoin Core `getblocktemplate` with the SegWit rule.
2. Parse the next-block height, previous block hash, version, target/bits, time bounds and transactions.
3. Construct a BIP34 coinbase transaction.
4. Include the template's witness commitment and the required 32-byte coinbase witness reservation when present.
5. Compute the transaction Merkle root.
6. Iterate the block nonce, then refresh the coinbase extra nonce/time when the nonce space is exhausted.
7. Verify the resulting header against the template target.
8. Optionally submit the complete serialized block through Bitcoin Core `submitblock`.

Bitcoin Core's documented mining flow is based on `getblocktemplate`, construction of the block/coinbase/Merkle root, proof-of-work search, and block submission. citeturn1search0turn1search1turn1search2

## Guardrails

The miner is **build-only by default**. Submission requires both:

- `BITCOIN_TESTNET_SUBMIT=YES`
- `BITCOIN_TESTNET_CONFIRM=YES`

The GitHub Actions workflow is manual-only and keeps the RPC credentials in repository Secrets.

Required secrets:

- `BITCOIN_TESTNET_RPC_URL`
- `BITCOIN_TESTNET_RPC_USER`
- `BITCOIN_TESTNET_RPC_PASSWORD`
- optional `BITCOIN_MINER_SCRIPT_PUBKEY`

For a real reward, use a scriptPubKey controlled by the operator. If no payout script is supplied, the implementation deliberately creates a zero-value demonstrator output rather than inventing a wallet.

## Current infrastructure constraint

Testnet3 support is deprecated in modern Bitcoin Core releases; Bitcoin Core explicitly recommends moving new testing to Testnet4. citeturn3search0turn3search1

Therefore this repository does **not** claim that a current public Testnet3 node is available. A real Testnet3 mining run requires access to an existing operator-controlled Testnet3-compatible node or mining service. No third-party credentials, leaked keys, mainnet funds, or fabricated block/TX identifiers are used.

For reproducible engineering tests, the same block-building path should also be exercised against regtest/Testnet4 infrastructure. Bitcoin Core documents regtest as the chain intended for regression testing and application development. citeturn3search0
