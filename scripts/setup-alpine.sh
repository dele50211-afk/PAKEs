#!/usr/bin/env sh
set -e

if [ "$(id -u)" -ne 0 ]; then
  echo "This script requires root. Re-run as: sudo $0"
  exit 1
fi

apk update
apk add --no-cache build-base pkgconfig openssl-dev bubblewrap socat
echo "Alpine prerequisites installed."
