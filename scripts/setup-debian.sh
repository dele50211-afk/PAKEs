#!/usr/bin/env bash
set -euo pipefail

if [ "$EUID" -ne 0 ]; then
  echo "This script requires sudo. Re-run as: sudo $0"
  exit 1
fi

apt-get update
apt-get install -y build-essential pkg-config libssl-dev bubblewrap socat
echo "Debian/Ubuntu prerequisites installed."
