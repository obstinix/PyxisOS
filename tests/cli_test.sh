#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLI="$SCRIPT_DIR/../cli/pyxis"

GREEN='\033[0;32m'
RED='\033[0;31m'
RESET='\033[0m'

test_command() {
    local cmd="$1"
    local expected="$2"
    echo -n "Testing 'pyxis $cmd'... "
    local output
    output=$("$CLI" "$cmd" 2>&1)
    if echo "$output" | grep -q "$expected"; then
        echo -e "${GREEN}PASS${RESET}"
    else
        echo -e "${RED}FAIL${RESET}"
        echo "Expected substring: $expected"
        echo "Got output: $output"
        exit 1
    fi
}

echo "=== Pyxis CLI Test Suite ==="
test_command "version" "0.1.0"
test_command "help" "Usage:"
test_command "info" "PYXISOS IDENTITY"
test_command "system" "PYXISOS SYSTEM METRICS"
test_command "status" "PYXISOS SERVICE STATUS"
test_command "doctor" "PYXISOS DOCTOR"
echo "=== All CLI tests passed successfully ==="
