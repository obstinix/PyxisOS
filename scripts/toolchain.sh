#!/usr/bin/env bash

set -euo pipefail

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
RESET='\033[0m'

check_tool() {
    local name="$1"
    local cmd="$2"
    if command -v "$cmd" >/dev/null 2>&1; then
        echo -e "  ${GREEN}[ OK ]${RESET} $name: $($cmd --version 2>&1 | head -n 1)"
        return 0
    else
        echo -e "  ${YELLOW}[WARN]${RESET} $name ($cmd) not found in PATH"
        return 1
    fi
}

echo "=== PyxisOS Toolchain Detection ==="
check_tool "C Compiler" "clang" || check_tool "C Compiler" "gcc" || true
check_tool "Linker" "ld.lld" || check_tool "Linker" "ld" || true
check_tool "Rust Compiler" "rustc"
check_tool "Cargo Package Manager" "cargo"
check_tool "Python 3" "python" || check_tool "Python 3" "python3" || true
check_tool "Make Build System" "make" || check_tool "Make Build System" "mingw32-make" || true
check_tool "QEMU x86_64 Emulator" "qemu-system-x86_64" || true
echo "==================================="
