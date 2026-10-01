#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
KERNEL_BIN="$ROOT_DIR/bin/pyxis-kernel.elf"

if [ ! -f "$KERNEL_BIN" ]; then
    bash "$SCRIPT_DIR/build.sh"
fi

if command -v qemu-system-x86_64 >/dev/null 2>&1; then
    echo "Starting QEMU in debug mode (GDB stub listening on tcp::1234)..."
    qemu-system-x86_64 \
        -kernel "$KERNEL_BIN" \
        -serial stdio \
        -m 128M \
        -s -S \
        -no-reboot
else
    echo "qemu-system-x86_64 not found."
    echo "To debug with GDB: qemu-system-x86_64 -kernel $KERNEL_BIN -s -S -serial stdio"
fi
