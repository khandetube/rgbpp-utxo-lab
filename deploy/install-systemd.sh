#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="${1:-/opt/rgbpp-utxo-lab}"
REPO_DIR="$INSTALL_DIR"
SERVICE_NAME=rgbpp-stratum-miner

if [[ $EUID -ne 0 ]]; then
  echo "Run as root: sudo $0 [install-dir]" >&2
  exit 1
fi

command -v cargo >/dev/null || { echo "Rust/Cargo is required; install the stable Rust toolchain first." >&2; exit 1; }
[[ -d "$REPO_DIR/.git" ]] || { echo "Expected a Git checkout at $REPO_DIR" >&2; exit 1; }

cargo build -p bitcoin-pow --bin stratum-miner --release --manifest-path "$REPO_DIR/Cargo.toml"

install -d -m 0755 /etc/rgbpp-utxo-lab
if [[ ! -f /etc/rgbpp-utxo-lab/miner.env ]]; then
  install -m 0600 "$REPO_DIR/deploy/miner.env.example" /etc/rgbpp-utxo-lab/miner.env
  echo "Created /etc/rgbpp-utxo-lab/miner.env; review it before starting the service."
fi

install -m 0644 "$REPO_DIR/deploy/rgbpp-stratum-miner.service" "/etc/systemd/system/$SERVICE_NAME.service"
systemctl daemon-reload
systemctl enable "$SERVICE_NAME"

echo "Installation complete."
echo "Edit /etc/rgbpp-utxo-lab/miner.env, then run:"
echo "  systemctl start $SERVICE_NAME"
