# Bitcoin UTXO Model

A UTXO is identified by an outpoint (txid, vout) and carries a value.

This project models the accounting problem independently of Bitcoin serialization and signing. Selection is deterministic: candidates are sorted by value and a stable identifier tie-breaker, then accumulated until the target amount plus fee is covered.

The later Bitcoin adapter will convert real chain data into these domain types.
