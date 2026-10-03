# Deterministic CKB Cell Selection

The lab now includes a network-independent cell-selection layer for CKB-style capacity transfers.

The selector:

1. Computes amount plus fee using checked arithmetic.
2. Orders candidate live cells deterministically by descending capacity, then transaction hash, then output index.
3. Selects cells until the required capacity is covered.
4. Returns selected inputs, total capacity, target capacity, and change.
5. Fails explicitly on arithmetic overflow or insufficient capacity.

This layer is intentionally separate from RPC discovery. The RPC adapter supplies chain data; the transaction layer decides which cells to consume.

It is not yet a complete CKB transaction builder. It does not calculate occupied capacity for arbitrary outputs, resolve cell dependencies, construct witnesses, sign, or broadcast. The next milestone will use the official CKB SDK transaction-building interfaces for those responsibilities.

All tests use synthetic data.
