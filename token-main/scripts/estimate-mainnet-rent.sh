#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -eq 0 ]; then
  set -- 200000 500000 1000000 2000000
fi

for bytes in "$@"; do
  printf "%s bytes: " "$bytes"
  solana rent "$bytes" --url mainnet-beta
done
