#!/usr/bin/env bash
set -e

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
CONTRACT_DIR="$ROOT_DIR/contracts/digital_identity"
WASM_TARGET="$CONTRACT_DIR/target/wasm32-unknown-unknown/release/digital_identity.wasm"

echo "Building digital_identity contract..."
cd "$CONTRACT_DIR"
cargo build --target wasm32-unknown-unknown --release

if ! command -v soroban &> /dev/null; then
  echo "soroban CLI not found. Install it: https://github.com/stellar/soroban-cli"
  exit 1
fi

echo "Deploying contract to testnet..."
# Try 'soroban contract deploy' (recent CLI) or fallback to 'soroban contract install'
if soroban contract deploy --wasm "$WASM_TARGET" --network testnet; then
  echo "Contract deploy attempted with 'soroban contract deploy'."
else
  echo "Fallback: trying 'soroban contract install'..."
  soroban contract install --wasm "$WASM_TARGET" --network testnet
fi

echo "Deployment finished. Note the contract ID printed above and set it in your client (config.contractId)."
