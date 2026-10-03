# Bitcoin Testnet3 mining workflow

This manual workflow validates a real Bitcoin Core `getblocktemplate` response before any mining attempt. It never broadcasts a block.

Required repository secrets:
- BITCOIN_TESTNET_RPC_URL
- BITCOIN_TESTNET_RPC_USER
- BITCOIN_TESTNET_RPC_PASSWORD

The RPC endpoint must be an operator-controlled Bitcoin Core Testnet3 node. Keep RPC credentials in GitHub Actions Secrets.

The workflow verifies that the returned template contains the next-block height, previous block hash, compact target (`bits`), network target, nonce range, timestamp bounds, and transaction list. A future miner can consume the validated template and submit a solved block through `submitblock` on the same controlled node.

Do not expose Bitcoin Core RPC publicly. Keep it bound to localhost or a private network and place any remote access behind authenticated infrastructure.