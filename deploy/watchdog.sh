#!/usr/bin/env bash
set -euo pipefail

SERVICE="${SERVICE:-rgbpp-stratum-miner}"

if ! systemctl is-active --quiet "$SERVICE"; then
  echo "watchdog: $SERVICE is not active; restarting"
  systemctl restart "$SERVICE"
  exit 0
fi

echo "watchdog: $SERVICE active"
