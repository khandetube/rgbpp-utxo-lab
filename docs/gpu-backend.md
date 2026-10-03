# GPU backend boundary

The mining protocol and hashing backend are intentionally separated.

A future CUDA/OpenCL implementation should expose the same logical operation as the CPU miner:

1. receive an 80-byte header prefix;
2. apply a nonce range and stride;
3. perform SHA256d;
4. compare the displayed hash against the share or network target;
5. return the first matching nonce and hash;
6. stop immediately when the current job is cancelled.

The backend must never receive wallet private keys or seed phrases.

GPU availability is detected from the host running the miner. ChatGPT/OpenAI inference GPUs are not exposed as user-programmable mining devices, so this document does not pretend that those GPUs are contributing Bitcoin hashrate.

## Acceptance criteria

- deterministic SHA256d test vector;
- identical results between CPU and GPU for the same header and nonce;
- bounded nonce ranges with no overlap;
- cancellation latency measurement;
- benchmarked hashes per second;
- Stratum share submission only from the authorized worker process.
