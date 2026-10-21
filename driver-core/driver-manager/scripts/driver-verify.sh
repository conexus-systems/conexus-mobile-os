#!/bin/bash
# Driver Package Verification Script

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Usage
if [ $# -lt 1 ]; then
    echo "Usage: $0 <package.cdrv> [public-key.pem]"
    exit 1
fi

PACKAGE="$1"
PUBLIC_KEY="${2:-}"

if [ ! -f "$PACKAGE" ]; then
    echo -e "${RED}Error: Package not found: $PACKAGE${NC}"
    exit 1
fi

echo -e "${BLUE}Verifying package: $PACKAGE${NC}"

# Verify file exists and is readable
if [ ! -r "$PACKAGE" ]; then
    echo -e "${RED}Error: Package not readable: $PACKAGE${NC}"
    exit 1
fi

# Calculate SHA256
SHA256=$(sha256sum "$PACKAGE" | cut -d' ' -f1)
echo -e "${BLUE}Package SHA256: $SHA256${NC}"

# Check for signature file
SIG_FILE="${PACKAGE}.sig"
if [ -f "${PACKAGE}.sig" ]; then
    echo -e "${BLUE}Signature file found: ${PACKAGE}.sig${NC}"
    
    # Check for certificate
    CERT_FILE="${PACKAGE}.pem"
    if [ -f "$CERT_FILE" ]; then
        echo -e "${BLUE}Certificate found: $CERT_FILE${NC}"
        
        # Verify certificate
        echo -e "${BLUE}Verifying certificate...${NC}"
        openssl x509 -in "$CERT_FILE" -text -noout | head -20
    fi
    
    # If public key provided, verify signature
    if [ -n "$2" ] && [ -f "$2" ]; then
        echo -e "${BLUE}Verifying signature with public key...${NC}"
        if openssl dgst -sha256 -verify "$2" -signature "${PACKAGE}.sig" "$PACKAGE"; then
            echo -e "${GREEN}Signature verified successfully${NC}"
        else
            echo -e "${RED}Signature verification failed${NC}"
            exit 1
        fi
    fi
else
    echo -e "${YELLOW}Warning: No signature file found (${PACKAGE}.sig)${NC}"
fi

# Extract and verify package contents
echo -e "${BLUE}Extracting and verifying package contents...${NC}"
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

cd "$TEMP_DIR"
unzip -q "$1" 2>/dev/null || {
    echo -e "${RED}Error: Failed to extract package (not a valid zip/cdrv file)${NC}"
    exit 1
)

# Verify manifest
if [ -f "manifest.json" ]; then
    echo -e "${GREEN}Manifest found${NC}"
    
    # Validate JSON
    if python3 -m json.tool manifest.json > /dev/null 2>&1; then
        echo -e "${GREEN}Manifest JSON valid${NC}"
        
        # Show package info
        echo -e "${BLUE}Package Information:${NC}"
        cat manifest.json | python3 -c "
import sys, json
data = json.load(sys.stdin)
print(f'  Name: {data.get(\"package\", {}).get(\"name\", \"N/A\")}')
print(f'  Version: {data.get(\"package\", {}).get(\"version\", \"N/A\")}')
print(f'  Vendor: {data.get(\"package\", {}).get(\"vendor\", \"N/A\")}')
print(f'  Devices: {\", \".join(data.get(\"package\", {}).get(\"devices\", []))}')
print(f'  Description: {data.get(\"package\", {}).get(\"description\", \"N/A\")[:80]}')
"
    else
        echo -e "${RED}Manifest JSON invalid${NC}"
        exit 1
    fi
else
    echo -e "${RED}Error: manifest.json not found${NC}"
    exit 1
fi

# Verify SHA256SUMS
if [ -f "SHA256SUMS" ]; then
    echo -e "${BLUE}Verifying file hashes...${NC}"
    if sha256sum -c SHA256SUMS 2>/dev/null; then
        echo -e "${GREEN}All file hashes verified${NC}"
    else
        echo -e "${RED}Hash verification failed${NC}"
        echo -e "${YELLOW}Details:${NC}"
        sha256sum -c SHA256SUMS 2>&1 | grep -v "OK"
        exit 1
    fi
else
    echo -e "${YELLOW}Warning: No SHA256SUMS file found${NC}"
fi

# Check for signature
if [ -f "signature.sig" ]; then
    echo -e "${BLUE}Signature file found: signature.sig${NC}"
else
    echo -e "${YELLOW}Warning: No signature file found${NC}"
fi

# Check for certificate
if [ -f "certificate.pem" ]; then
    echo -e "${BLUE}Certificate found: certificate.pem${NC}"
    openssl x509 -in certificate.pem -text -noout | grep -E "(Subject:|Issuer:|Not Before:|Not After:|Public Key Algorithm:)" | sed 's/^/  /'
fi

# List package contents
echo -e "${BLUE}Package Contents:${NC}"
find . -type f | grep -v "^\.$" | sort | sed 's/^/  /'

echo -e "${GREEN}Verification complete${NC}"