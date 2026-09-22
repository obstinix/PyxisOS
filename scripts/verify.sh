#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
RESET='\033[0m'

failures=0

check_file() {
    local file="$1"
    if [ -f "$ROOT_DIR/$file" ]; then
        echo -e "  ${GREEN}[ OK ]${RESET} File exists: $file"
    else
        echo -e "  ${RED}[FAIL]${RESET} Missing file: $file"
        failures=$((failures + 1))
    fi
}

check_syntax() {
    local script="$1"
    if [ -f "$ROOT_DIR/$script" ]; then
        if bash -n "$ROOT_DIR/$script"; then
            echo -e "  ${GREEN}[ OK ]${RESET} Syntax valid: $script"
        else
            echo -e "  ${RED}[FAIL]${RESET} Syntax error: $script"
            failures=$((failures + 1))
        fi
    fi
}

echo "=== PyxisOS Repository Verification ==="
echo

echo "1. Checking Core Files..."
check_file "README.md"
check_file "LICENSE"
check_file "CONTRIBUTING.md"
check_file "SECURITY.md"
check_file "VERSION"
check_file "cli/pyxis"
check_file "scripts/cli.sh"
check_file "system/boot/pyxisos_boot_commands.sh"
check_file "system/branding/os-release"
check_file "system/branding/pyxis-banner"
echo

echo "2. Validating Shell Script Syntax..."
check_syntax "cli/pyxis"
check_syntax "scripts/cli.sh"
check_syntax "scripts/verify.sh"
check_syntax "system/branding/pyxis-banner"
check_syntax "system/boot/pyxisos_boot_commands.sh"
check_syntax "tests/cli_test.sh"
echo

echo "3. Testing Pyxis CLI Execution..."
if [ -f "$ROOT_DIR/cli/pyxis" ]; then
    if bash "$ROOT_DIR/cli/pyxis" version >/dev/null 2>&1; then
        echo -e "  ${GREEN}[ OK ]${RESET} 'pyxis version' responded"
    else
        echo -e "  ${RED}[FAIL]${RESET} 'pyxis version' execution failed"
        failures=$((failures + 1))
    fi

    if bash "$ROOT_DIR/cli/pyxis" doctor >/dev/null 2>&1; then
        echo -e "  ${GREEN}[ OK ]${RESET} 'pyxis doctor' responded"
    else
        echo -e "  ${RED}[FAIL]${RESET} 'pyxis doctor' execution failed"
        failures=$((failures + 1))
    fi
fi
echo

echo "4. Scanning Repository Security Hygiene..."
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    leaked_keys=$(git grep -i -E '(PRIVATE KEY|BEGIN OPENSSH PRIVATE KEY|github_pat_[0-9a-zA-Z_]+)' 2>/dev/null || true)
    if [ -n "$leaked_keys" ]; then
        echo -e "  ${RED}[FAIL]${RESET} Potential credentials or private keys detected in tracked files:"
        echo "$leaked_keys"
        failures=$((failures + 1))
    else
        echo -e "  ${GREEN}[ OK ]${RESET} No credentials or private keys found in tracked files"
    fi
fi
echo

echo "------------------------------------------------------------"
if [ "$failures" -eq 0 ]; then
    echo -e "${GREEN}ALL VERIFICATION CHECKS PASSED${RESET}"
    exit 0
else
    echo -e "${RED}VERIFICATION FAILED: $failures issue(s) detected${RESET}"
    exit 1
fi
