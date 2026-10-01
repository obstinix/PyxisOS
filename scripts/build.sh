#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== Building PyxisOS ==="
cd "$ROOT_DIR"

if command -v make >/dev/null 2>&1; then
    make build
elif command -v mingw32-make >/dev/null 2>&1; then
    mingw32-make build
else
    echo "Make not found, invoking direct toolchain pipeline..."
    mkdir -p build/kernel/core build/mm build/drivers/char build/drivers/video build/drivers/timer build/drivers/input build/fs build/userspace/shell build/arch/x86_64/boot build/arch/x86_64/cpu build/arch/x86_64/interrupts build/arch/x86_64/context build/arch/x86_64/syscall bin

    clang -target x86_64-unknown-none-elf -c arch/x86_64/boot/header.S -o build/arch/x86_64/boot/header.o
    clang -target x86_64-unknown-none-elf -c arch/x86_64/boot/boot.S -o build/arch/x86_64/boot/boot.o
    clang -target x86_64-unknown-none-elf -c arch/x86_64/cpu/gdt_flush.S -o build/arch/x86_64/cpu/gdt_flush.o
    clang -target x86_64-unknown-none-elf -c arch/x86_64/cpu/idt_flush.S -o build/arch/x86_64/cpu/idt_flush.o
    clang -target x86_64-unknown-none-elf -c arch/x86_64/interrupts/isr.S -o build/arch/x86_64/interrupts/isr.o
    clang -target x86_64-unknown-none-elf -c arch/x86_64/context/switch.S -o build/arch/x86_64/context/switch.o
    clang -target x86_64-unknown-none-elf -c arch/x86_64/syscall/syscall.S -o build/arch/x86_64/syscall/syscall.o

    for cfile in kernel/core/*.c mm/*.c drivers/char/*.c drivers/video/*.c drivers/timer/*.c drivers/input/*.c fs/*.c userspace/shell/*.c arch/x86_64/cpu/*.c arch/x86_64/interrupts/*.c; do
        obj="build/${cfile%.c}.o"
        clang -target x86_64-unknown-none-elf -ffreestanding -mno-red-zone -mcmodel=kernel -nostdlib -Iinclude -I. -c "$cfile" -o "$obj"
    done

    (cd kernel && cargo build --target x86_64-unknown-none)

    ld.lld -T linker.ld -nostdlib -static -z max-page-size=0x1000 build/arch/x86_64/boot/*.o build/arch/x86_64/cpu/*.o build/arch/x86_64/interrupts/*.o build/arch/x86_64/context/*.o build/arch/x86_64/syscall/*.o build/kernel/core/*.o build/mm/*.o build/drivers/char/*.o build/drivers/video/*.o build/drivers/timer/*.o build/drivers/input/*.o build/fs/*.o build/userspace/shell/*.o kernel/target/x86_64-unknown-none/debug/libpyxis_kernel.a -o bin/pyxis-kernel.elf
fi

echo "Build successful: bin/pyxis-kernel.elf"
