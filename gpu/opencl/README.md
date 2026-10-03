# OpenCL SHA256d backend

This directory contains a standalone reference kernel for Bitcoin-style SHA256d header hashing.

## Contract

Input:
- exactly 80 bytes of a Bitcoin block header;
- a nonce base supplied by the host.

Each work item hashes one nonce: nonce_base + global_id.

Output:
- raw 32-byte SHA256d digest;
- the nonce used.

## Host-side validation

The eventual host adapter must:
1. compare GPU digests against the Rust double_sha256 implementation;
2. interpret the digest using the same target and byte-order rules as the Rust verifier;
3. cancel work immediately when a new Stratum job replaces the current job;
4. never submit a nonce from a stale job;
5. keep GPU execution separate from wallet/private-key handling.

Version rolling is additional header work space and must respect the negotiated mask; it does not replace target validation.

A GPU hash benchmark or a pool share is not a confirmed BTC balance. A balance change requires independently verified on-chain receipt.
