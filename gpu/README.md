# GPU PoW track

This directory is an independent GPU execution path. It does not alter the existing Stratum, GBT, Regtest, or route-supervisor paths.

## OpenCL reference kernel

The OpenCL kernel accepts an 80-byte Bitcoin header template and assigns one nonce per work item. It performs Bitcoin-style double SHA256 and returns the raw 32-byte digest.

The host integration should validate the header, allocate bounded nonce batches, launch the kernel, compare digests with the current target, and cancel a batch when a new Stratum job arrives.

## Verification

Before public-pool integration, compare GPU digests against the Rust double_sha256 implementation for identical headers and nonces.

No private key is required. Real BTC requires an authorized machine and independently verified accepted work.
