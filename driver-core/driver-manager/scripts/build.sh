#!/bin/bash
# Driver Manager Build Script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Building Conexus Driver Manager...${NC}"

cd "$PROJECT_ROOT"

# Check if gradlew exists
if [ ! -f "./gradlew" ]; then
    echo -e "${RED}Error: gradlew not found in project root${NC}"
    exit 1
fi

# Build the driver-manager module
echo -e "${BLUE}Building driver-manager module...${NC}"
./gradlew :driver-manager:build -x test

# Build the driver-manager module tests
echo -e "${BLUE}Running tests...${NC}"
./gradlew :driver-manager:test

# Build the Kotlin SDK
echo -e "${BLUE}Building Kotlin SDK...${NC}"
./gradlew :conexus-sdk:kotlin:build -x test

echo -e "${GREEN}Build completed successfully!${NC}"

# Show artifacts
echo -e "\n${BLUE}Built artifacts:${NC}"
find . -name "*.aar" -o -name "*.jar" | grep -E "(driver-manager|conexus-sdk)" | head -20