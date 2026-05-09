#!/usr/bin/env bash
# invoke.sh — Helper to call WaveHub contract functions.
#
# Usage:
#   CONTRACT_ID=<id> ./scripts/invoke.sh <function> [args...]
#
# Examples:
#   CONTRACT_ID=C... ./scripts/invoke.sh wave_count
#   CONTRACT_ID=C... ./scripts/invoke.sh create_wave --duration 3600 --total_pool 1000000
#   CONTRACT_ID=C... ./scripts/invoke.sh get_wave --wave_id 1
#   CONTRACT_ID=C... ./scripts/invoke.sh finalize_wave --wave_id 1

set -euo pipefail

NETWORK="${NETWORK:-testnet}"
SOURCE="${SOURCE:-default}"

if [[ -z "${CONTRACT_ID:-}" ]]; then
  echo "Error: CONTRACT_ID environment variable is required." >&2
  echo "  export CONTRACT_ID=<your-contract-id>" >&2
  exit 1
fi

if [[ $# -lt 1 ]]; then
  echo "Usage: CONTRACT_ID=<id> $0 <function> [args...]" >&2
  exit 1
fi

FUNCTION="$1"
shift

stellar contract invoke \
  --id "$CONTRACT_ID" \
  --network "$NETWORK" \
  --source "$SOURCE" \
  -- "$FUNCTION" "$@"
