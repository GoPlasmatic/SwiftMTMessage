#!/bin/bash

set -e

echo "🚀 Starting Backward Compatibility Test"
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Navigate to the backward compatibility test directory
cd "$(dirname "$0")"

echo
echo -e "${BLUE}📦 Step 1: Building with old version (published)${NC}"
echo "Building binaries with published version..."
cargo build --release --features published
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Failed to build with published version${NC}"
    exit 1
fi

echo
echo -e "${BLUE}📋 Step 2: Generating JSON with old version${NC}"
echo "Processing test data with published version..."
./target/release/generate_old_json
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Failed to generate JSON with old version${NC}"
    exit 1
fi

echo
echo -e "${BLUE}🔄 Step 3: Switching to local version${NC}"
echo "Cleaning and rebuilding with local version..."
cargo clean
cargo build --release --features local --no-default-features
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Failed to build with local version${NC}"
    exit 1
fi

echo
echo -e "${BLUE}📋 Step 4: Generating JSON with new version${NC}"
echo "Processing test data with local version..."
./target/release/generate_new_json
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Failed to generate JSON with new version${NC}"
    exit 1
fi

echo
echo -e "${BLUE}🔍 Step 5: Comparing versions${NC}"
echo "Analyzing compatibility..."
./target/release/compare_compatibility --detailed --output compatibility_report.md
COMPARISON_RESULT=$?

echo
echo -e "${BLUE}📊 Step 6: Results${NC}"
if [ $COMPARISON_RESULT -eq 0 ]; then
    echo -e "${GREEN}✅ Backward compatibility test PASSED!${NC}"
    echo "No breaking changes detected."
else
    echo -e "${RED}❌ Backward compatibility test FAILED!${NC}"
    echo "Breaking changes detected. Check the output above and compatibility_report.md for details."
fi

echo
echo "📁 Output files:"
echo "  - Old version JSON: output/old_version/"
echo "  - New version JSON: output/new_version/"
echo "  - Compatibility report: compatibility_report.md"

echo
echo "🔧 Manual Commands:"
echo "  Generate old JSON: cargo run --release --features published --bin generate_old_json"
echo "  Generate new JSON: cargo run --release --features local --no-default-features --bin generate_new_json"
echo "  Compare versions: cargo run --release --bin compare_compatibility -- --detailed"

exit $COMPARISON_RESULT 