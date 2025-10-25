#!/bin/bash

# Docker Integration Test Script
# Tests Docker daemon detection and control features

set -e

echo "=== Docker Integration Test Suite ==="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

test_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ PASS${NC}: $2"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC}: $2"
        ((TESTS_FAILED++))
    fi
}

# Test 1: Check Docker context
echo "Test 1: Docker Context Configuration"
docker context ls > /dev/null 2>&1
test_result $? "Docker context command works"

CONTEXT=$(docker context show 2>/dev/null || echo "unknown")
echo "   Current context: $CONTEXT"

# Test 2: Check socket file existence
echo ""
echo "Test 2: Docker Socket File"
SOCKET_PATH="/Users/$USER/.docker/run/docker.sock"
if [ -S "$SOCKET_PATH" ]; then
    test_result 0 "Docker Desktop socket exists at $SOCKET_PATH"
    ls -lh "$SOCKET_PATH"
else
    test_result 1 "Docker Desktop socket NOT found at $SOCKET_PATH"
fi

# Test 3: Test socket connectivity with curl
echo ""
echo "Test 3: Socket Connectivity Test"
RESPONSE=$(curl -s --unix-socket "$SOCKET_PATH" http://localhost/_ping 2>/dev/null || echo "FAIL")
if [ "$RESPONSE" = "OK" ]; then
    test_result 0 "Docker socket responds to ping"
else
    test_result 1 "Docker socket does not respond (got: $RESPONSE)"
fi

# Test 4: Docker CLI connectivity
echo ""
echo "Test 4: Docker CLI Commands"
docker info > /dev/null 2>&1
test_result $? "docker info command works"

docker ps > /dev/null 2>&1
test_result $? "docker ps command works"

# Test 5: Check Docker version
echo ""
echo "Test 5: Docker Version Info"
DOCKER_VERSION=$(docker version --format '{{.Server.Version}}' 2>/dev/null || echo "unknown")
if [ "$DOCKER_VERSION" != "unknown" ]; then
    test_result 0 "Docker server version detected: $DOCKER_VERSION"
else
    test_result 1 "Could not detect Docker server version"
fi

# Test 6: Check if Docker Desktop app is running
echo ""
echo "Test 6: Docker Desktop Process"
if pgrep -f "Docker.app" > /dev/null 2>&1; then
    test_result 0 "Docker Desktop application is running"
    echo "   Processes:"
    ps aux | grep -i "Docker.app" | grep -v grep | head -3 | awk '{print "   ", $2, $11}'
else
    test_result 1 "Docker Desktop application is NOT running"
fi

# Test 7: Test Rust backend detection (if app is running)
echo ""
echo "Test 7: Backend API Test (if Tauri app is accessible)"
# This would require the app to be running and expose the API
echo -e "${YELLOW}⊘ SKIP${NC}: Requires manual testing in running app"

# Test 8: Alternative Docker runtimes
echo ""
echo "Test 8: Alternative Docker Runtimes"

# Check for Colima
if command -v colima &> /dev/null; then
    echo -e "${GREEN}✓ Found${NC}: Colima is installed"
    colima status 2>&1 | head -1
else
    echo "  Colima: not installed"
fi

# Check for Podman
if command -v podman &> /dev/null; then
    echo -e "${GREEN}✓ Found${NC}: Podman is installed"
    podman --version
else
    echo "  Podman: not installed"
fi

# Summary
echo ""
echo "=== Test Summary ==="
echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Failed: ${RED}$TESTS_FAILED${NC}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed. Check output above.${NC}"
    exit 1
fi
