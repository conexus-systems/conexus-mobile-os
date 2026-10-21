#!/bin/bash
# Conexus Mobile OS - Quick Installer Wrapper
# Usage: ./install.sh [options]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYTHON_SCRIPT="$SCRIPT_DIR/install_conexus.py"

# Colors
RED='\033[91m'
GREEN='\033[92m'
YELLOW='\033[93m'
BLUE='\033[94m'
CYAN='\033[96m'
BOLD='\033[1m'
RESET='\033[0m'

# Banner
echo -e "${CYAN}╔═══════════════════════════════════════════════════╗${RESET}"
echo -e "${CYAN}║${RESET}  ${BOLD}Conexus Mobile OS - Automatic Installer${RESET}       ${CYAN}║${RESET}"
echo -e "${CYAN}║${RESET}  ${WHITE}LineageOS 23.2 + Conexus Launcher + Metro UI${RESET}  ${CYAN}║${RESET}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════╝${RESET}"
echo ""

# Check Python
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}Error: python3 not found. Please install Python 3.8+${RESET}"
    exit 1
fi

# Check ADB
if ! command -v adb &> /dev/null; then
    echo -e "${RED}Error: adb not found. Install Android Platform Tools.${RESET}"
    exit 1
fi

# Check fastboot
if ! command -v fastboot &> /dev/null; then
    echo -e "${RED}Error: fastboot not found. Install Android Platform Tools.${RESET}"
    exit 1
fi

# Check Python script exists
if [[ ! -f "$PYTHON_SCRIPT" ]]; then
    echo -e "${RED}Error: Python installer not found at $PYTHON_SCRIPT${RESET}"
    exit 1
fi

# Run Python installer with all arguments
echo -e "${BLUE}Starting Conexus Mobile OS installer...${RESET}"
echo ""

exec python3 "$PYTHON_SCRIPT" "$@"