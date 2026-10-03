# Real Bitcoin Stratum mining\n\nThe repository includes a standalone Stratum V1 client for an operator-controlled workstation, VPS, mining controller, or ASIC host. It implements the standard subscribe/authorize/notify/submit flow. Stratum supplies the coinbase fragments, extranonces, merkle branches, version, nBits and nTime; the miner constructs the 80-byte header and performs SHA256d proof-of-work.\n\nFor ckpool solo mode, the Bitcoin address is supplied as the Stratum username and a solved block is assigned to that miner address, subject to any configured donation. citeturn0search0turn0search2\n\n## Run on your own machine\n\nDo not run this as a continuous GitHub-hosted miner. Use hardware or a machine you are authorized to operate.\n\n    export STRATUM_URL=solo.ckpool.org:3333\n    export STRATUM_USERNAME=bc1qrwhe5l4wvx86g6rs4n3tpyr85j0xr3cs6lex4d\n    export STRATUM_PASSWORD=x\n    export MINER_THREADS=$(nproc)\n    cargo run -p bitcoin-pow --bin stratum-miner --release\n\nAn accepted share is not a Bitcoin block; only a network-accepted block produces the block reward. Mainnet mining remains probabilistic and requires substantial real hash rate for meaningful expected results.\n\nNever place private keys or seed phrases in the repository or chat.\n
## Opt-in idle-hash worker pool

The miner can be deployed on multiple independently controlled machines and point all workers at the same payout username. This is the practical way to aggregate otherwise-unused SHA256 ASIC capacity: each participant explicitly opts in, runs a worker, and contributes only the compute they control. The pool receives the shares and, for solo mining, the payout remains tied to the configured Bitcoin address.

Multiple Stratum endpoints can be configured for resilience:

    export STRATUM_URLS=solo.ckpool.org:3333,solo.example.net:3333
    export STRATUM_USERNAME=bc1qrwhe5l4wvx86g6rs4n3tpyr85j0xr3cs6lex4d
    export STRATUM_PASSWORD=x
    export MINER_THREADS=$(nproc)
    cargo run -p bitcoin-pow --bin stratum-miner --release

This project does **not** scan, hijack, install on, or consume third-party machines. Only explicitly authorized workers should participate. A worker's hash rate can be measured from accepted shares, but no block or BTC balance should be reported until a real network block is independently verified.
