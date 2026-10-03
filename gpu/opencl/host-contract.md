# GPU host execution contract

## Work lifecycle

1. Receive a current 80-byte Bitcoin header template.
2. Reserve a bounded nonce interval.
3. Launch the OpenCL sha256d_header kernel.
4. Read back digests and nonces.
5. Apply the exact same target comparison used by the Rust verifier.
6. On a new job, cancel or discard the current batch before submission.
7. Only a result tied to the current job and session may be submitted.

## Accounting

Track separately:
- GPU hashes executed;
- pool shares accepted;
- pool shares rejected;
- network-valid block candidates;
- independently confirmed blocks;
- BTC actually credited to the configured payout destination.

A share is not BTC. A block candidate is not a confirmed reward.

Current network conditions are substantial: October 3, 2026 sources report roughly 922–950 EH/s and difficulty around 132.72 T. This makes local CPU/GPU proof-of-work useful for engineering validation, but not evidence of a $100 payout.
