#!/usr/bin/env bash
#
# Build Script for Sentinel
# Built by Glincker (A GLINR Product)
#
# Usage:
#   ./scripts/build.sh [release|debug]
#   ./scripts/build.sh release    # Production build
#   ./scripts/build.sh debug      # Development build (default)

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BUILD_TYPE="${1:-debug}"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$PROJECT_ROOT/dist"

echo -e "${BLUE}🛡️  Sentinel Build Script${NC}"
echo -e "${BLUE}=========================${NC}\n"

# Validate build type
if [[ "$BUILD_TYPE" != "release" && "$BUILD_TYPE" != "debug" ]]; then
    echo -e "${RED}❌ Invalid build type: $BUILD_TYPE${NC}"
    echo "Usage: $0 [release|debug]"
    exit 1
fi

echo -e "${GREEN}Build Type:${NC} $BUILD_TYPE"
echo -e "${GREEN}Project Root:${NC} $PROJECT_ROOT\n"

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo not found. Install from https://rustup.rs/${NC}"
    exit 1
fi
echo -e "${GREEN}✓${NC} Rust $(rustc --version)"

if ! command -v npm &> /dev/null; then
    echo -e "${RED}❌ npm not found. Install Node.js from https://nodejs.org/${NC}"
    exit 1
fi
echo -e "${GREEN}✓${NC} npm $(npm --version)"

# Clean previous builds
echo -e "\n${YELLOW}Cleaning previous builds...${NC}"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"
cargo clean
echo -e "${GREEN}✓${NC} Cleaned"

# Install frontend dependencies
echo -e "\n${YELLOW}Installing frontend dependencies...${NC}"
cd "$PROJECT_ROOT"
npm install
echo -e "${GREEN}✓${NC} Frontend dependencies installed"

# Build frontend
echo -e "\n${YELLOW}Building frontend...${NC}"
npm run build
echo -e "${GREEN}✓${NC} Frontend built"

# Build Rust backend
echo -e "\n${YELLOW}Building Rust backend ($BUILD_TYPE)...${NC}"
cd "$PROJECT_ROOT/src-tauri"

if [[ "$BUILD_TYPE" == "release" ]]; then
    cargo build --release
    BINARY_PATH="$PROJECT_ROOT/src-tauri/target/release/sentinel"
else
    cargo build
    BINARY_PATH="$PROJECT_ROOT/src-tauri/target/debug/sentinel"
fi

echo -e "${GREEN}✓${NC} Backend built"

# Copy binary to dist
echo -e "\n${YELLOW}Packaging...${NC}"
cp "$BINARY_PATH" "$DIST_DIR/"
echo -e "${GREEN}✓${NC} Binary copied to $DIST_DIR/sentinel"

# Print binary info
if [[ -f "$DIST_DIR/sentinel" ]]; then
    BINARY_SIZE=$(du -h "$DIST_DIR/sentinel" | cut -f1)
    echo -e "\n${GREEN}✅ Build successful!${NC}"
    echo -e "${BLUE}Binary:${NC} $DIST_DIR/sentinel"
    echo -e "${BLUE}Size:${NC} $BINARY_SIZE"
    echo -e "\n${YELLOW}Run with:${NC} $DIST_DIR/sentinel --version"
else
    echo -e "\n${RED}❌ Build failed: Binary not found${NC}"
    exit 1
fi
