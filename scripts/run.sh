#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
KERNEL_BIN="$ROOT_DIR/bin/pyxis-kernel.elf"

if [ ! -f "$KERNEL_BIN" ]; then
    bash "$SCRIPT_DIR/build.sh"
fi

if command -v qemu-system-x86_64 >/dev/null 2>&1; then
    echo "Launching PyxisOS in QEMU..."
    qemu-system-x86_64 \
        -kernel "$KERNEL_BIN" \
        -serial stdio \
        -m 128M \
        -no-reboot \
        -no-shutdown
else
    echo "Note: qemu-system-x86_64 is not currently in PATH."
    echo "Kernel ELF binary is ready at: $KERNEL_BIN"
    echo "To launch when QEMU is installed, run:"
    echo "  qemu-system-x86_64 -kernel $KERNEL_BIN -serial stdio -m 128M"
fi
