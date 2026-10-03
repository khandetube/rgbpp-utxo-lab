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
