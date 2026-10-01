#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "Cleaning PyxisOS build artifacts..."
rm -rf "$ROOT_DIR/build" "$ROOT_DIR/bin"
(cd "$ROOT_DIR/kernel" && cargo clean 2>/dev/null || true)
echo "Cleanup completed."
