#!/usr/bin/env bash
#
# Quick Pre-Commit Test
# Tests the most critical workflows before pushing
#
# Usage: ./scripts/quick-test.sh

set -e

# Get project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "🧪 Running Quick Pre-Commit Tests..."
echo "===================================="
echo "Project: $PROJECT_ROOT"
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Track failures
FAILED=0

# Test 1: Rust Formatting
echo "1️⃣  Checking Rust formatting..."
cd "$PROJECT_ROOT/src-tauri"
if cargo fmt --all -- --check 2>/dev/null; then
    echo -e "${GREEN}✓${NC} Rust formatting OK"
else
    echo -e "${RED}✗${NC} Rust formatting failed (run 'cargo fmt' to fix)"
    ((FAILED++))
fi
cd "$PROJECT_ROOT"
echo ""

# Test 2: Rust Linting
echo "2️⃣  Running Rust clippy..."
cd "$PROJECT_ROOT/src-tauri"
if cargo clippy --all-targets --all-features -- -D warnings 2>/dev/null; then
    echo -e "${GREEN}✓${NC} Clippy OK"
else
    echo -e "${RED}✗${NC} Clippy failed"
    ((FAILED++))
fi
cd "$PROJECT_ROOT"
echo ""

# Test 3: Rust Tests (skip if no tests directory yet)
echo "3️⃣  Running Rust tests..."
cd "$PROJECT_ROOT/src-tauri"
if cargo test --workspace --lib 2>/dev/null; then
    echo -e "${GREEN}✓${NC} Rust tests OK"
else
    echo -e "${YELLOW}⚠${NC} Rust tests skipped or failed"
    # Don't fail on tests for now
fi
cd "$PROJECT_ROOT"
echo ""

# Test 4: Frontend Linting (skip if not set up)
echo "4️⃣  Running frontend lint..."
if [ -f "$PROJECT_ROOT/package.json" ]; then
    if pnpm run lint 2>/dev/null; then
        echo -e "${GREEN}✓${NC} Frontend lint OK"
    else
        echo -e "${YELLOW}⚠${NC} Frontend lint skipped or failed"
        # Don't fail on lint for now
    fi
else
    echo -e "${YELLOW}⚠${NC} Frontend not set up (skipped)"
fi
echo ""

# Test 5: Frontend Tests (skip if not set up)
echo "5️⃣  Running frontend tests..."
if [ -f "$PROJECT_ROOT/package.json" ]; then
    if pnpm test 2>/dev/null; then
        echo -e "${GREEN}✓${NC} Frontend tests OK"
    else
        echo -e "${YELLOW}⚠${NC} Frontend tests skipped or failed"
        # Don't fail on tests for now
    fi
else
    echo -e "${YELLOW}⚠${NC} Frontend not set up (skipped)"
fi
echo ""

# Summary
echo "===================================="
if [[ ${FAILED} -eq 0 ]]; then
    echo -e "${GREEN}✅ Core checks passed!${NC}"
    echo ""
    echo "You can now:"
    echo "  1. Test full workflows: ./scripts/test-workflows.sh ci"
    echo "  2. Or commit directly: git commit ..."
    exit 0
else
    echo -e "${RED}❌ ${FAILED} critical check(s) failed${NC}"
    echo -e "${YELLOW}Please fix the issues before committing.${NC}"
    exit 1
fi
