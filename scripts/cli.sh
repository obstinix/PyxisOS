#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLI_SOURCE="$SCRIPT_DIR/../cli/pyxis"
CLI_TARGET="/usr/local/bin/pyxis"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RESET='\033[0m'

echo
echo "============================================================"
echo "                  PYXISOS CLI INSTALLER"
echo "============================================================"
echo

if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}[ERROR]${RESET} This script must be run with sudo."
    echo
    echo "Usage:"
    echo "  sudo ./scripts/cli.sh"
    exit 1
fi

if [ ! -f "$CLI_SOURCE" ]; then
    echo -e "${RED}[ERROR]${RESET} CLI source not found:"
    echo "        $CLI_SOURCE"
    exit 1
fi

echo "[ 1/4 ] Installing PyxisOS CLI..."

install -m 755 "$CLI_SOURCE" "$CLI_TARGET"

echo -e "${GREEN}[ OK ]${RESET} CLI installed"
echo "      $CLI_TARGET"

echo
echo "[ 2/4 ] Verifying executable..."

if [ -x "$CLI_TARGET" ]; then
    echo -e "${GREEN}[ OK ]${RESET} Pyxis CLI is executable"
else
    echo -e "${RED}[FAIL]${RESET} Pyxis CLI is not executable"
    exit 1
fi

echo
echo "[ 3/4 ] Verifying command..."

if command -v pyxis >/dev/null 2>&1; then
    echo -e "${GREEN}[ OK ]${RESET} pyxis command available"
    echo "      $(command -v pyxis)"
else
    echo -e "${RED}[FAIL]${RESET} pyxis command not found"
    exit 1
fi

echo
echo "[ 4/4 ] Checking PyxisOS CLI..."

if pyxis version >/dev/null 2>&1; then
    echo -e "${GREEN}[ OK ]${RESET} PyxisOS CLI responding"
else
    echo -e "${YELLOW}[WARN]${RESET} CLI installed but version check failed"
fi

echo
echo "------------------------------------------------------------"
echo "              PYXISOS CLI INSTALL COMPLETE"
echo "------------------------------------------------------------"
echo

echo "Run:"
echo
echo "    pyxis"
echo "    pyxis info"
echo "    pyxis system"
echo "    pyxis status"
echo "    pyxis doctor"
echo "    pyxis help"
echo

echo "============================================================"
echo
