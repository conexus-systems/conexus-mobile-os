#!/bin/bash
# Driver Package Signing Script

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Usage
if [ $# -lt 2 ]; then
    echo "Usage: $0 <package.cdrv> <signing-key.pem> [certificate.pem]"
    exit 1
fi

PACKAGE="$1"
SIGN_KEY="$2"
CERT="${3:-}"

if [ ! -f "$PACKAGE" ]; then
    echo -e "${RED}Error: Package not found: $PACKAGE${NC}"
    exit 1
fi

if [ ! -f "$SIGN_KEY" ]; then
    echo -e "${RED}Error: Signing key not found: $SIGN_KEY${NC}"
    exit 1
fi

echo -e "${BLUE}Signing package: $PACKAGE${NC}"

# Sign the package
openssl dgst -sha256 -sign "$SIGN_KEY" -out "${PACKAGE}.sig" "$PACKAGE"

echo -e "${GREEN}Package signed: ${PACKAGE}.sig${NC}"

# Verify signature
if [ -n "$3" ] && [ -f "$3" ]; then
    echo -e "${BLUE}Verifying signature...${NC}"
    openssl dgst -sha256 -verify "$3" -signature "${PACKAGE}.sig" "$PACKAGE"
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Signature verified successfully${NC}"
    else
        echo -e "${RED}Signature verification failed${NC}"
        exit 1
    fi
fi

# Verify package integrity
echo -e "${BLUE}Verifying package integrity...${NC}"
SHA256=$(sha256sum "$PACKAGE" | cut -d' ' -f1)
echo "SHA256: $SHA256"

# Verify package contents
echo -e "${BLUE}Verifying package contents...${NC}"
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

unzip -q "$1" -d "$TEMP_DIR"

# Verify manifest
if [ -f "$TEMP_DIR/manifest.json" ]; then
    echo -e "${GREEN}Manifest found${NC}"
    python3 -m json.tool "$TEMP_DIR/manifest.json" > /dev/null && echo -e "${GREEN}Manifest JSON valid${NC}" || echo -e "${RED}Manifest JSON invalid${NC}"
else
    echo -e "${RED}Error: manifest.json not found${NC}"
    exit 1
fi

# Verify SHA256SUMS
if [ -f "$TEMP_DIR/SHA256SUMS" ]; then
    echo -e "${BLUE}Verifying file hashes...${NC}"
    cd "$(dirname "$1")"
    sha256sum -c "$TEMP_DIR/SHA256SUMS" 2>/dev/null && echo -e "${GREEN}All file hashes verified${NC}" || echo -e "${RED}Hash verification failed${NC}"
fi

echo -e "${GREEN}Package verification complete${NC}"