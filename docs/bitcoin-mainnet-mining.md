# Bitcoin Mainnet mining path

The repository now contains a parallel, explicitly guarded Bitcoin Mainnet path alongside the existing Regtest/Testnet3 tooling.

## Flow

1. Connect to an operator-controlled Bitcoin Core Mainnet node.
2. Request a current SegWit block template with `getblocktemplate`.
3. Validate the template target and network parameters.
4. Construct a BIP34 coinbase paying the supplied Mainnet address.
5. Build the transaction Merkle root.
6. Search the nonce/extra-nonce/time space with Bitcoin double-SHA256 proof of work.
7. Verify the candidate against the network target.
8. Only when both Mainnet submission guards are explicitly enabled, submit the complete block with `submitblock`.

## Payout target

The manual GitHub Actions workflow defaults to the public Mainnet address supplied for this project:

`bc1qrwhe5l4wvx86g6rs4n3tpyr85j0xr3cs6lex4d`

The address is only a payout destination. No private key is stored in the repository or workflow.

## Mainnet safety

- Mainnet submission is **OFF by default**.
- Submission requires both `submit=YES` and `BITCOIN_MAINNET_CONFIRM=YES`.
- RPC credentials must be stored as GitHub Actions Secrets.
- No private key or seed phrase is required by the miner.
- A solved block is the event that creates a coinbase reward.
- A successful mining run is **not guaranteed**; Bitcoin Mainnet proof of work requires sufficient hashing power and a valid current template.
- The GitHub-hosted runner is ordinary CPU compute and is not competitive with dedicated Bitcoin ASIC mining. This workflow therefore provides the real Mainnet construction/submission path, but it does not imply that a runner will find a Mainnet block.

Bitcoin's solo-mining flow uses a current block template, constructs the coinbase/Merkle root, performs proof-of-work, and submits the completed block.
