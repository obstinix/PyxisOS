import os
import sys

DIRS = [
    "build/kernel/core",
    "build/mm",
    "build/drivers/char",
    "build/drivers/video",
    "build/drivers/timer",
    "build/drivers/input",
    "build/fs",
    "build/userspace/shell",
    "build/arch/x86_64/boot",
    "build/arch/x86_64/cpu",
    "build/arch/x86_64/interrupts",
    "build/arch/x86_64/context",
    "build/arch/x86_64/syscall",
    "bin",
]

def main():
    root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    for d in DIRS:
        p = os.path.join(root, d)
        os.makedirs(p, exist_ok=True)

if __name__ == "__main__":
    main()
