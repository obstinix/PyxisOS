# PyxisOS Master Makefile
# Low-level x86_64 Operating System Build Orchestrator

ARCH = x86_64
CC = clang
AS = clang
LD = ld.lld
CARGO = cargo
PYTHON = python

CFLAGS = -target x86_64-unknown-none-elf -ffreestanding -mno-red-zone -mcmodel=kernel -nostdlib -Wall -Wextra -Iinclude -I.
ASFLAGS = -target x86_64-unknown-none-elf -c
LDFLAGS = -T linker.ld -nostdlib -static -z max-page-size=0x1000

BUILD_DIR = build
BIN_DIR = bin

C_SRCS = $(wildcard kernel/core/*.c) \
         $(wildcard mm/*.c) \
         $(wildcard drivers/char/*.c) \
         $(wildcard drivers/video/*.c) \
         $(wildcard drivers/timer/*.c) \
         $(wildcard drivers/input/*.c) \
         $(wildcard fs/*.c) \
         $(wildcard userspace/shell/*.c) \
         $(wildcard arch/x86_64/cpu/*.c) \
         $(wildcard arch/x86_64/interrupts/*.c)

ASM_SRCS = arch/x86_64/boot/header.S \
           arch/x86_64/boot/boot.S \
           arch/x86_64/cpu/gdt_flush.S \
           arch/x86_64/cpu/idt_flush.S \
           arch/x86_64/interrupts/isr.S \
           arch/x86_64/context/switch.S \
           arch/x86_64/syscall/syscall.S

C_OBJS = $(patsubst %.c, $(BUILD_DIR)/%.o, $(C_SRCS))
ASM_OBJS = $(patsubst %.S, $(BUILD_DIR)/%.o, $(ASM_SRCS))

RUST_LIB = kernel/target/x86_64-unknown-none/debug/libpyxis_kernel.a
KERNEL_ELF = $(BIN_DIR)/pyxis-kernel.elf

.PHONY: all build kernel rust check run debug test clean distclean dirs

all: build

dirs:
	@$(PYTHON) tools/python/build_dirs.py

$(BUILD_DIR)/%.o: %.c
	@echo "  CC   $<"
	@$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: %.S
	@echo "  AS   $<"
	@$(AS) $(ASFLAGS) $< -o $@

rust:
	@echo "  CARGO kernel"
	@cd kernel && $(CARGO) build --target x86_64-unknown-none

$(KERNEL_ELF): dirs $(ASM_OBJS) $(C_OBJS) rust
	@echo "  LD   $@"
	@$(LD) $(LDFLAGS) $(ASM_OBJS) $(C_OBJS) $(RUST_LIB) -o $@
	@echo "PyxisOS kernel successfully linked: $@"

build: $(KERNEL_ELF)

kernel: $(KERNEL_ELF)

check:
	@echo "=== PyxisOS Toolchain & Component Verification ==="
	@bash scripts/toolchain.sh
	@echo "=== Checking Rust Subsystems ==="
	@cd kernel && $(CARGO) check --target x86_64-unknown-none
	@echo "All pre-flight checks passed."

test: build
	@echo "=== Executing Automated Test Suite ==="
	@bash scripts/test.sh

run: build
	@bash scripts/run.sh

debug: build
	@bash scripts/debug.sh

clean:
	@rm -rf $(BUILD_DIR) $(BIN_DIR)
	@cd kernel && $(CARGO) clean
	@echo "Clean completed."

distclean: clean
	@rm -rf tools/python/__pycache__
