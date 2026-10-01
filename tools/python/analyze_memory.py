import os
import subprocess
import sys

def main():
    root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    bin_path = os.path.join(root, "bin", "pyxis-kernel.elf")

    if not os.path.exists(bin_path):
        print(f"Error: {bin_path} not found. Run make build first.")
        sys.exit(1)

    print(f"Analyzing memory layout for: {bin_path}")
    size = os.path.getsize(bin_path)
    print(f"Kernel binary file size: {size} bytes ({size / 1024:.2f} KB)")

    try:
        res = subprocess.run(["llvm-objdump", "-h", bin_path], capture_output=True, text=True)
        if res.returncode == 0:
            print("\nELF Sections:")
            print(res.stdout)
    except Exception as e:
        print(f"Note: llvm-objdump not available: {e}")

if __name__ == "__main__":
    main()
