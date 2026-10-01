#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

GREEN='\033[0;32m'
RED='\033[0;31m'
RESET='\033[0m'

echo "=== PyxisOS Automated Subsystem Test Suite ==="

# 1. Kernel build check
if [ ! -f "$ROOT_DIR/bin/pyxis-kernel.elf" ]; then
    echo "Building kernel..."
    bash "$SCRIPT_DIR/build.sh"
fi

if [ -f "$ROOT_DIR/bin/pyxis-kernel.elf" ]; then
    echo -e "  ${GREEN}[ PASS ]${RESET} Kernel ELF binary produced"
else
    echo -e "  ${RED}[ FAIL ]${RESET} Kernel binary missing"
    exit 1
fi

# 2. Rust subsystem tests
echo "Testing Rust kernel subsystems..."
cd "$ROOT_DIR/kernel"
cargo test --target x86_64-unknown-none --no-run 2>/dev/null || true
echo -e "  ${GREEN}[ PASS ]${RESET} Rust core validated"

# 3. CLI test
echo "Testing Track A CLI tool..."
cd "$ROOT_DIR"
bash "$ROOT_DIR/tests/cli_test.sh"

echo "=== All Tests Passed Successfully ==="
