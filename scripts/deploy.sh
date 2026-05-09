#!/usr/bin/env bash
# deploy.sh — Build and deploy WaveHub to a Stellar network.
#
# Usage:
#   ./scripts/deploy.sh [testnet|mainnet|standalone]
#
# Prerequisites:
#   - stellar-cli installed (cargo install --locked stellar-cli --features opt)
#   - A funded identity named "default" (stellar keys generate default)

set -euo pipefail

NETWORK="${1:-testnet}"
WASM="contracts/wavehub/target/wasm32-unknown-unknown/release/wavehub.wasm"

echo "==> Building WaveHub for wasm32..."
cargo build \
  --manifest-path contracts/wavehub/Cargo.toml \
  --target wasm32-unknown-unknown \
  --release

echo "==> Deploying to ${NETWORK}..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM" \
  --network "$NETWORK" \
  --source default)

echo ""
echo "✓ Deployed WaveHub"
echo "  Contract ID : ${CONTRACT_ID}"
echo "  Network     : ${NETWORK}"
echo ""
echo "Next step — initialize the contract:"
echo "  stellar contract invoke --id ${CONTRACT_ID} --network ${NETWORK} --source default \\"
echo "    -- initialize --owner \$(stellar keys address default)"
