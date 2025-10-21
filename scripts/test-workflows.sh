#!/usr/bin/env bash
#
# Test GitHub Actions Workflows Locally
# Built by Glincker (A GLINR Product)
#
# Usage:
#   ./test-workflows.sh [all|ci|pr|coverage|quick]
#   ./test-workflows.sh all        # Test all workflows
#   ./test-workflows.sh ci         # Test CI workflow only
#   ./test-workflows.sh pr         # Test PR checks workflow
#   ./test-workflows.sh coverage   # Test coverage workflow
#   ./test-workflows.sh quick      # Quick test (lint + unit tests)

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
TEST_MODE="${1:-quick}"
CONTAINER_ARCH="linux/amd64"  # For M1/M2 Macs

echo -e "${BLUE}🧪 Testing GitHub Actions Workflows Locally${NC}"
echo -e "${BLUE}==========================================${NC}\n"

# Check if act is installed
if ! command -v act &> /dev/null; then
    echo -e "${RED}❌ 'act' is not installed${NC}"
    echo -e "${YELLOW}Install with:${NC}"
    echo "  macOS:    brew install act"
    echo "  Linux:    curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash"
    echo "  Windows:  choco install act-cli"
    exit 1
fi

# Check if Docker is running
if ! docker info &> /dev/null; then
    echo -e "${RED}❌ Docker is not running${NC}"
    echo -e "${YELLOW}Please start Docker Desktop and try again${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} act installed: $(act --version)"
echo -e "${GREEN}✓${NC} Docker running\n"

# Function to run workflow test
run_test() {
    local test_name="$1"
    local event="$2"
    local job="$3"
    local workflow="${4:-}"

    echo -e "\n${BLUE}▶ Testing: ${test_name}${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    local cmd="act ${event} -j ${job} --container-architecture ${CONTAINER_ARCH} --pull=false"

    if [[ -n "${workflow}" ]]; then
        cmd="${cmd} -W ${workflow}"
    fi

    # Add dry-run for release workflow to avoid creating actual releases
    if [[ "${job}" == "create-release" ]] || [[ "${job}" == "build-release" ]]; then
        cmd="${cmd} --dry-run"
    fi

    echo -e "${BLUE}Command:${NC} ${cmd}"
    echo ""

    if eval "${cmd}"; then
        echo -e "\n${GREEN}✅ ${test_name} passed${NC}"
        return 0
    else
        echo -e "\n${RED}❌ ${test_name} failed${NC}"
        return 1
    fi
}

# Track results
PASSED=0
FAILED=0

case "${TEST_MODE}" in
    quick)
        echo -e "${YELLOW}Running quick tests (lint + unit tests)...${NC}\n"

        if run_test "Rust Linting" "push" "test-rust" ".github/workflows/ci.yml"; then
            ((PASSED++))
        else
            ((FAILED++))
        fi

        if run_test "Frontend Linting" "push" "test-frontend" ".github/workflows/ci.yml"; then
            ((PASSED++))
        else
            ((FAILED++))
        fi
        ;;

    ci)
        echo -e "${YELLOW}Running full CI workflow...${NC}\n"

        if run_test "Rust Tests" "push" "test-rust" ".github/workflows/ci.yml"; then
            ((PASSED++))
        else
            ((FAILED++))
        fi

        if run_test "Frontend Tests" "push" "test-frontend" ".github/workflows/ci.yml"; then
            ((PASSED++))
        else
            ((FAILED++))
        fi

        if run_test "Security Audit" "push" "security" ".github/workflows/ci.yml"; then
            ((PASSED++))
        else
            ((FAILED++))
        fi

        if run_test "Build" "push" "build" ".github/workflows/ci.yml"; then
            ((PASSED++))
        else
            ((FAILED++))
        fi
        ;;

    pr)
        echo -e "${YELLOW}Running PR checks workflow...${NC}\n"

        if run_test "PR Validation" "pull_request" "validate-pr" ".github/workflows/pr-checks.yml"; then
            ((PASSED++))
        else
            ((FAILED++))
        fi

        if run_test "PR Labeling" "pull_request" "label-pr" ".github/workflows/pr-checks.yml"; then
            ((PASSED++))
        else
            ((FAILED++))
        fi
        ;;

    coverage)
        echo -e "${YELLOW}Running coverage workflow...${NC}\n"
        echo -e "${YELLOW}Note: This may take several minutes...${NC}\n"

        if run_test "Code Coverage" "push" "coverage" ".github/workflows/coverage.yml"; then
            ((PASSED++))
        else
            ((FAILED++))
        fi
        ;;

    all)
        echo -e "${YELLOW}Running all workflow tests...${NC}\n"
        echo -e "${YELLOW}Note: This will take 10-20 minutes...${NC}\n"

        # CI Workflow
        for job in "test-rust" "test-frontend" "security" "build"; do
            if run_test "CI: ${job}" "push" "${job}" ".github/workflows/ci.yml"; then
                ((PASSED++))
            else
                ((FAILED++))
            fi
        done

        # PR Checks Workflow
        for job in "validate-pr" "label-pr"; do
            if run_test "PR: ${job}" "pull_request" "${job}" ".github/workflows/pr-checks.yml"; then
                ((PASSED++))
            else
                ((FAILED++))
            fi
        done

        # Coverage Workflow
        if run_test "Coverage" "push" "coverage" ".github/workflows/coverage.yml"; then
            ((PASSED++))
        else
            ((FAILED++))
        fi

        # Release Workflow (dry-run only)
        echo -e "\n${YELLOW}Note: Skipping release workflow (requires tag trigger)${NC}"
        ;;

    *)
        echo -e "${RED}❌ Invalid test mode: ${TEST_MODE}${NC}"
        echo "Usage: $0 [all|ci|pr|coverage|quick]"
        exit 1
        ;;
esac

# Summary
echo -e "\n${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Test Summary${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Passed:${NC} ${PASSED}"
echo -e "${RED}Failed:${NC} ${FAILED}"

if [[ ${FAILED} -eq 0 ]]; then
    echo -e "\n${GREEN}✅ All tests passed!${NC}"
    echo -e "${GREEN}Your workflows are ready to push to GitHub.${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Some tests failed${NC}"
    echo -e "${YELLOW}Please fix the issues before pushing to GitHub.${NC}"
    exit 1
fi
