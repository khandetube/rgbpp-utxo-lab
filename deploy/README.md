# 24/7 Bitcoin Stratum Miner Host

Deployment files for running the project's Stratum V1 miner continuously on an operator-controlled Linux machine.

Use only hardware and network resources you own or are explicitly authorized to use. The miner does not guarantee a block or payout. Keep credentials outside Git.

## Install

On Ubuntu/Debian:

    sudo apt-get update
    sudo apt-get install -y git build-essential pkg-config libssl-dev curl

Clone the repository at /opt/rgbpp-utxo-lab, then:

    sudo ./deploy/install-systemd.sh /opt/rgbpp-utxo-lab

Review `/etc/rgbpp-utxo-lab/miner.env`, then:

    sudo systemctl enable --now rgbpp-stratum-miner
    systemctl status rgbpp-stratum-miner
    journalctl -u rgbpp-stratum-miner -f

The service restarts after unexpected exits and starts after host reboot.


## VM bootstrap

For a cloud VM that supports cloud-init, use `deploy/cloud-init.yaml` as the user-data configuration. It installs the required build toolchain, clones this repository, installs the systemd service and enables the miner/watchdog.

After boot:

    sudo systemctl status rgbpp-stratum-miner
    sudo systemctl status rgbpp-stratum-watchdog.timer
    sudo journalctl -u rgbpp-stratum-miner --since "10 minutes ago"

The bootstrap does not create cloud accounts, allocate paid resources, or bypass provider authorization.

## Operational checks

The miner must show an active Stratum connection and share/hashrate telemetry in its journal before it is considered operational. A running systemd process alone is not proof of hashing or payout.

For a real Mainnet payout, independently verify any resulting block on the Bitcoin network. Pool shares are not BTC and a submitted share is not proof that a block was found.
