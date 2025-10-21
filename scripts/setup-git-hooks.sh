#!/usr/bin/env bash
#
# Setup Git Hooks for Sentinel using Lefthook
# Built by Glincker (A GLINR Product)
#
# This script sets up:
# - Lefthook for fast, parallel Git hooks
# - Commitlint for conventional commits
# - Commitizen for interactive commits
#
# Lefthook is 2-3x faster than Husky and runs hooks in parallel!
#
# Usage: ./scripts/setup-git-hooks.sh

set -euo pipefail

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${BLUE}🚀 Setting up Lefthook Git Hooks for Sentinel${NC}"
echo -e "${BLUE}===============================================${NC}\n"

# Get project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# Check if Lefthook is installed
if ! command -v lefthook &> /dev/null; then
    echo -e "${YELLOW}Installing Lefthook...${NC}"

    # Install via npm (will be added to node_modules)
    npm install

    echo -e "${GREEN}✓${NC} Lefthook installed"
else
    echo -e "${GREEN}✓${NC} Lefthook already installed: $(lefthook version)"
fi

echo -e "\n${YELLOW}Installing Node.js dependencies...${NC}"
npm install
echo -e "${GREEN}✓${NC} Dependencies installed"

echo -e "\n${YELLOW}Installing Lefthook hooks...${NC}"
npx lefthook install
echo -e "${GREEN}✓${NC} Lefthook hooks installed"

# Summary
echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Lefthook setup complete!${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

echo "Hooks configured (see lefthook.yml):"
echo "  • ${GREEN}pre-commit${NC}   - Formats & lints (runs in parallel!)"
echo "    ├─ Frontend: Prettier + ESLint"
echo "    ├─ Rust Backend: cargo fmt"
echo "    └─ Rust CLI: cargo fmt"
echo ""
echo "  • ${GREEN}commit-msg${NC}   - Validates commit message format"
echo "    └─ Enforces Conventional Commits"
echo ""
echo "  • ${YELLOW}pre-push${NC}     - Optional: runs tests (disabled)"
echo ""

echo "Configuration:"
echo "  lefthook.yml  - Hook configuration (parallel execution enabled)"
echo "  commitlint.config.js - Commit message rules"
echo ""

echo "Usage:"
echo "  ${CYAN}Standard commit:${NC}"
echo "    git add ."
echo "    git commit -m \"feat: your message\""
echo ""

echo "  ${CYAN}Interactive commit (Commitizen):${NC}"
echo "    git add ."
echo "    npm run commit"
echo ""

echo "  ${CYAN}Bypass hooks (NOT recommended):${NC}"
echo "    git commit --no-verify"
echo ""

echo "  ${CYAN}Uninstall hooks:${NC}"
echo "    npm run hooks:uninstall"
echo ""

echo "Commit types allowed:"
echo "  ${GREEN}feat${NC}, ${GREEN}fix${NC}, ${GREEN}docs${NC}, ${GREEN}style${NC}, ${GREEN}refactor${NC}, ${GREEN}perf${NC}, ${GREEN}test${NC}, ${GREEN}build${NC}, ${GREEN}ci${NC}, ${GREEN}chore${NC}, ${GREEN}revert${NC}"
echo ""

echo -e "${YELLOW}What happens automatically on commit:${NC}"
echo "  ✓ Prettier formats your code (parallel)"
echo "  ✓ ESLint lints your code (parallel)"
echo "  ✓ cargo fmt formats Rust code (parallel)"
echo "  ✓ Commit message validated"
echo "  ✓ 2-3x faster than Husky!"
echo ""

echo -e "${CYAN}Why Lefthook?${NC}"
echo "  • Written in Go (compiled binary, fast)"
echo "  • Parallel execution (Rust + JS lints run simultaneously)"
echo "  • Better for polyglot projects (Rust + Node.js)"
echo "  • Works in CI without Node.js"
echo ""

echo "Test it:"
echo "  git add ."
echo "  npm run commit  # Interactive commit"
echo ""

echo -e "${GREEN}Happy committing! 🎉${NC}"
