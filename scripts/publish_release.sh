#!/bin/bash
#
# Publish SwiftMTMessage release to GitHub
#
# This script:
# 1. Generates JSON schemas for all message types
# 2. Generates the plugin manifest
# 3. Creates a GitHub release
# 4. Uploads schemas and manifest as release assets
#
# Usage:
#   ./scripts/publish_release.sh v3.1.4        # Publish specific version
#   ./scripts/publish_release.sh               # Use version from Cargo.toml
#   ./scripts/publish_release.sh --dry-run     # Generate files without publishing
#
# Requirements:
#   - gh CLI (GitHub CLI) installed and authenticated
#   - cargo installed
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
SCHEMAS_DIR="$PROJECT_DIR/schemas"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Parse arguments
DRY_RUN=false
VERSION=""

for arg in "$@"; do
    case $arg in
        --dry-run)
            DRY_RUN=true
            ;;
        v*)
            VERSION="$arg"
            ;;
        *)
            if [[ -z "$VERSION" ]]; then
                VERSION="$arg"
            fi
            ;;
    esac
done

# Get version from Cargo.toml if not provided
if [[ -z "$VERSION" ]]; then
    VERSION=$(grep '^version = ' "$PROJECT_DIR/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/')
    VERSION="v$VERSION"
fi

# Ensure version starts with 'v'
if [[ ! "$VERSION" =~ ^v ]]; then
    VERSION="v$VERSION"
fi

# Extract version number without 'v' prefix
VERSION_NUM="${VERSION#v}"

log_info "SwiftMTMessage Release Publisher"
log_info "================================="
log_info "Version: $VERSION"
log_info "Dry run: $DRY_RUN"
echo ""

# Change to project directory
cd "$PROJECT_DIR"

# Step 1: Build the project
log_info "Building project..."
cargo build --release --features jsonschema
log_success "Build complete"

# Step 2: Generate JSON schemas and manifest
log_info "Generating JSON schemas and plugin manifest..."
cargo run --example generate_manifest --features jsonschema -- "$VERSION_NUM"
log_success "JSON schemas and manifest generated"

# List generated files
echo ""
log_info "Generated files in $SCHEMAS_DIR:"
ls -la "$SCHEMAS_DIR"
echo ""

# Count files
SCHEMA_COUNT=$(ls -1 "$SCHEMAS_DIR"/*.schema.json 2>/dev/null | wc -l | tr -d ' ')
log_info "Generated $SCHEMA_COUNT schema files + manifest.json"

if $DRY_RUN; then
    log_warn "Dry run mode - skipping GitHub release creation"
    echo ""
    log_info "To publish this release, run:"
    echo "  ./scripts/publish_release.sh $VERSION"
    exit 0
fi

# Step 3: Check if gh CLI is available
if ! command -v gh &> /dev/null; then
    log_error "GitHub CLI (gh) is not installed"
    log_info "Install it with: brew install gh"
    log_info "Then authenticate with: gh auth login"
    exit 1
fi

# Check if authenticated
if ! gh auth status &> /dev/null; then
    log_error "GitHub CLI is not authenticated"
    log_info "Run: gh auth login"
    exit 1
fi

log_success "GitHub CLI authenticated"

# Step 4: Check if release already exists
if gh release view "$VERSION" &> /dev/null; then
    log_warn "Release $VERSION already exists"
    read -p "Do you want to delete and recreate it? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        log_info "Deleting existing release..."
        gh release delete "$VERSION" --yes
        log_success "Existing release deleted"
    else
        log_info "Aborting"
        exit 1
    fi
fi

# Step 5: Create GitHub release
log_info "Creating GitHub release $VERSION..."

# Build release notes
RELEASE_NOTES="## SwiftMTMessage $VERSION

### Plugin Manifest & JSON Schemas

This release includes:
- \`manifest.json\` - Plugin manifest with all supported message types
- JSON Schema files for all ${SCHEMA_COUNT} supported SWIFT MT message types

### Supported Message Types

#### Customer Payments & Cheques (Category 1)
MT101, MT103, MT104, MT107, MT110, MT111, MT112, MT190, MT191, MT192, MT196, MT199

#### Financial Institution Transfers (Category 2)
MT200, MT202, MT204, MT205, MT210, MT290, MT291, MT292, MT296, MT299

#### Cash Management & Customer Status (Category 9)
MT900, MT910, MT920, MT935, MT940, MT941, MT942, MT950

### Usage

Download the manifest to discover available schemas:
\`\`\`bash
curl -sL https://github.com/GoPlasmatic/SwiftMTMessage/releases/download/$VERSION/manifest.json | jq .
\`\`\`

Download a specific schema:
\`\`\`bash
curl -sLO https://github.com/GoPlasmatic/SwiftMTMessage/releases/download/$VERSION/MT103.schema.json
\`\`\`
"

# Create the release
gh release create "$VERSION" \
    --title "SwiftMTMessage $VERSION" \
    --notes "$RELEASE_NOTES" \
    "$SCHEMAS_DIR"/*.json

log_success "Release $VERSION created successfully!"

# Show release URL
RELEASE_URL=$(gh release view "$VERSION" --json url -q .url)
echo ""
log_info "Release URL: $RELEASE_URL"
log_info "Manifest URL: https://github.com/GoPlasmatic/SwiftMTMessage/releases/download/$VERSION/manifest.json"

echo ""
log_success "Release publishing complete!"
