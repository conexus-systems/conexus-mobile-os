#!/bin/bash
# Driver Package Build Script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Building Conexus Driver Package...${NC}"

# Usage
if [ $# -lt 1 ]; then
    echo "Usage: $0 <source-directory> [output-file]"
    echo "Example: $0 ./my-driver-source ./my-driver.cdrv"
    exit 1
fi

SOURCE_DIR="$1"
OUTPUT_FILE="${2:-$(basename "$SOURCE_DIR").cdrv}"

if [ ! -d "$SOURCE_DIR" ]; then
    echo -e "${RED}Error: Source directory not found: $SOURCE_DIR${NC}"
    exit 1
fi

# Check for manifest
if [ ! -f "$SOURCE_DIR/manifest.json" ]; then
    echo -e "${RED}Error: manifest.json not found in source directory${NC}"
    exit 1
fi

echo -e "${BLUE}Building package from $SOURCE_DIR...${NC}"

# Create temporary directory
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Copy source to temp
cp -r "$SOURCE_DIR"/* "$TEMP_DIR/"

# Validate manifest
if [ ! -f "$TEMP_DIR/manifest.json" ]; then
    echo -e "${RED}Error: manifest.json not found${NC}"
    exit 1
fi

# Validate manifest JSON
if ! python3 -m json.tool "$TEMP_DIR/manifest.json" > /dev/null 2>&1; then
    echo -e "${RED}Error: Invalid JSON in manifest.json${NC}"
    exit 1
fi

# Calculate SHA256 for all files
echo "Calculating SHA256 hashes..."
cd "$TEMP_DIR"
find . -type f ! -name "*.sig" ! -name "*.pem" ! -name "manifest.json" | while read file; do
    hash=$(sha256sum "$file" | cut -d' ' -f1)
    echo "$hash  $file" >> SHA256SUMS
done

# Create package
cd "$TEMP_DIR"
cdrv_file=$(basename "$OUTPUT_FILE")
if [[ "$OUTPUT_FILE" != *.cdrv ]]; then
    cdrv_file="${cdrv_file}.cdrv"
fi

zip -r "$OUTPUT_FILE" . -x "*.sig" "*.pem"

# Calculate final SHA256
SHA256=$(sha256sum "$OUTPUT_FILE" | cut -d' ' -f1)
echo "$SHA256  $(basename "$OUTPUT_FILE")" > "${OUTPUT_FILE}.sha256"

# Sign if key provided
if [ -n "$SIGN_KEY" ] && [ -f "$SIGN_KEY" ]; then
    echo "Signing package..."
    openssl dgst -sha256 -sign "$SIGN_KEY" -out "${OUTPUT_FILE}.sig" "$OUTPUT_FILE"
fi

echo -e "${GREEN}Package created: $OUTPUT_FILE${NC}"
echo -e "${BLUE}SHA256: $SHA256${NC}"