# PyxisOS Development Environment Setup

Official development environment setup for PyxisOS Native (Track B).

This guide configures a cross-platform operating system development environment on Windows, macOS, or Linux using Rust, Cargo, Alacritty, VS Code, QEMU, and GDB.

## System Requirements

| Component | Requirement |
| --------- | ----------- |
| OS | Windows 11 / macOS (12+) / Linux (modern Ubuntu/Arch/etc.) |
| RAM | 8 GB minimum (16 GB recommended) |
| Storage | 10 GB free space |
| CPU | x86_64 or ARM processor with hardware virtualization support |

## Recommended Toolchain

| Tool | Purpose |
| ---- | ------- |
| Alacritty | Cross-platform GPU-accelerated terminal emulator |
| Git | Version control |
| Rustup & Cargo | Rust toolchain installer and package manager |
| LLVM Tools | LLVM binary utilities (`llvm-tools-preview` component) |
| QEMU | Processor emulator to run the OS binary |
| GDB / LLDB | Debuggers to connect to QEMU |
| VS Code | Recommended integrated development environment |

---

## Setup Instructions

### 1. Install Rust
Install the Rust toolchain manager via `rustup`:
- **Windows / macOS / Linux**: Follow instructions at [https://rustup.rs/](https://rustup.rs/).
- PyxisOS requires the **nightly** channel for bare-metal `#![no_std]` features. This is configured automatically in `native/rust-toolchain.toml`.

To manually configure nightly:
```bash
rustup toolchain install nightly
```

### 2. Install Rust Components
The kernel requires standard library source code for cross-compilation and LLVM tools for building:
```bash
rustup component add rust-src --toolchain nightly
rustup component add llvm-tools-preview --toolchain nightly
```

### 3. Install QEMU
Install the standard emulator for running the compiled OS image:
- **Windows (via MSYS2)**:
  ```bash
  pacman -S mingw-w64-ucrt-x86_64-qemu
  ```
- **macOS (via Homebrew)**:
  ```bash
  brew install qemu
  ```
- **Linux (Debian/Ubuntu)**:
  ```bash
  sudo apt-get install qemu-system-x86
  ```
- **Linux (Arch)**:
  ```bash
  sudo pacman -S qemu-desktop
  ```

### 4. Install GDB / LLDB
- **Windows**: Install GDB through MSYS2:
  ```bash
  pacman -S mingw-w64-ucrt-x86_64-gdb
  ```
- **macOS / Linux**: Typically pre-installed or available via default package managers.

### 5. Configure Visual Studio Code Extensions
For systems development in Rust, install these extensions:
- **rust-analyzer** (essential for code completion and syntax analysis)
- **CodeLLDB** (recommended for debugger integration with QEMU)
- **Better TOML**
- **Hex Editor**

---

## Development Workflow

Unlike native applications, the `lunar-core` kernel is compiled for a bare-metal architecture with no operating system assumptions.

```
Rust Source (lunar-core)
      │
      ▼
rustc (Nightly) + Custom target spec (x86_64-pyxis.json)
      │
      ▼
ELF Binary (no_std, no_main)
      │
      ▼
Bootloader / ISO Image
      │
      ▼
QEMU Emulator (runs the image)
      │
      ▼
GDB / LLDB (debugging interface)
```

### Building the Workspace
Under the `native/` folder, run:
```bash
cargo build --target lunar-core/x86_64-pyxis.json
```

---

## Troubleshooting

### QEMU command not found
Ensure the directory containing the QEMU binaries is added to your system's `PATH` environment variable. On Windows, this is typically `C:\msys64\ucrt64\bin` or `C:\Program Files\qemu`.

### Target spec not found
Make sure you are executing the cargo commands from the correct directory relative to the target description file (`native/lunar-core/x86_64-pyxis.json`).
