# Bitcoin UTXO layer

The UTXO package now also uses the official rust-bitcoin primitives for parsing Bitcoin outpoints. This keeps Bitcoin transaction identifiers and outpoint structure type-safe instead of representing every value as an unvalidated string.

The current lab deliberately stops short of broadcasting Bitcoin transactions because a real Bitcoin signer and node/API backend must be selected and configured for the target network. The protocol types are nevertheless provided by rust-bitcoin.
