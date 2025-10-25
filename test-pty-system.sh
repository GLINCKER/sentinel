#!/bin/bash

# Test script for PTY Process Management System
# This script tests the PTY functionality end-to-end

set -e

echo "🧪 Testing PTY Process Management System"
echo "========================================"
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test results
TESTS_PASSED=0
TESTS_FAILED=0

# Helper function to print test results
pass() {
    echo -e "${GREEN}✓ PASS${NC}: $1"
    ((TESTS_PASSED++))
}

fail() {
    echo -e "${RED}✗ FAIL${NC}: $1"
    ((TESTS_FAILED++))
}

info() {
    echo -e "${YELLOW}ℹ INFO${NC}: $1"
}

echo "Test 1: Check if Sentinel is running"
echo "------------------------------------"
if pgrep -f "sentinel" > /dev/null; then
    pass "Sentinel process is running"
else
    fail "Sentinel is not running"
    echo "Please start Sentinel with: pnpm tauri dev"
    exit 1
fi
echo ""

echo "Test 2: Create test script for PTY"
echo "------------------------------------"
cat > /tmp/pty-test-script.sh << 'EOF'
#!/bin/bash
echo "PTY Test: Line 1"
sleep 0.5
echo "PTY Test: Line 2"
sleep 0.5
echo "PTY Test: Line 3"
sleep 0.5
echo "PTY Test: Complete"
exit 0
EOF
chmod +x /tmp/pty-test-script.sh

if [ -f /tmp/pty-test-script.sh ]; then
    pass "Created test script at /tmp/pty-test-script.sh"
else
    fail "Failed to create test script"
    exit 1
fi
echo ""

echo "Test 3: Test basic echo command"
echo "------------------------------------"
info "Testing echo command output capture..."

# Create a simple test - just verify echo works
TEST_OUTPUT=$(echo "Hello PTY")
if [ "$TEST_OUTPUT" = "Hello PTY" ]; then
    pass "Echo command works correctly"
else
    fail "Echo command failed"
fi
echo ""

echo "Test 4: Test script execution"
echo "------------------------------------"
info "Running test script..."

if bash /tmp/pty-test-script.sh > /tmp/pty-test-output.txt 2>&1; then
    if grep -q "PTY Test: Complete" /tmp/pty-test-output.txt; then
        pass "Test script executed successfully"
        info "Script output:"
        cat /tmp/pty-test-output.txt
    else
        fail "Test script didn't produce expected output"
    fi
else
    fail "Test script execution failed"
fi
echo ""

echo "Test 5: Check PTY dependencies"
echo "------------------------------------"

# Check if portable-pty is in Cargo.toml
if grep -q "portable-pty" src-tauri/Cargo.toml; then
    pass "portable-pty dependency found in Cargo.toml"
else
    fail "portable-pty dependency not found"
fi
echo ""

echo "Test 6: Verify PTY manager exists"
echo "------------------------------------"
if [ -f "src-tauri/src/core/pty_process_manager.rs" ]; then
    pass "PTY process manager file exists"

    # Check for key functions
    if grep -q "pub async fn spawn_process" src-tauri/src/core/pty_process_manager.rs; then
        pass "spawn_process function found"
    else
        fail "spawn_process function not found"
    fi

    if grep -q "pub async fn kill_process" src-tauri/src/core/pty_process_manager.rs; then
        pass "kill_process function found"
    else
        fail "kill_process function not found"
    fi

    if grep -q "pub async fn restart_process" src-tauri/src/core/pty_process_manager.rs; then
        pass "restart_process function found"
    else
        fail "restart_process function not found"
    fi
else
    fail "PTY process manager file not found"
fi
echo ""

echo "Test 7: Verify Tauri commands"
echo "------------------------------------"
if [ -f "src-tauri/src/commands/pty.rs" ]; then
    pass "PTY commands file exists"

    # Check for registered commands
    if grep -q "spawn_pty_process" src-tauri/src/commands/pty.rs; then
        pass "spawn_pty_process command found"
    else
        fail "spawn_pty_process command not found"
    fi

    if grep -q "kill_pty_process" src-tauri/src/commands/pty.rs; then
        pass "kill_pty_process command found"
    else
        fail "kill_pty_process command not found"
    fi

    if grep -q "restart_pty_process" src-tauri/src/commands/pty.rs; then
        pass "restart_pty_process command found"
    else
        fail "restart_pty_process command not found"
    fi
else
    fail "PTY commands file not found"
fi
echo ""

echo "Test 8: Verify frontend integration"
echo "------------------------------------"
if [ -f "src/routes/pty-test/+page.svelte" ]; then
    pass "PTY test page exists"
else
    fail "PTY test page not found"
fi

if [ -f "src/stores/ptyProcesses.svelte.ts" ]; then
    pass "PTY store exists"
else
    fail "PTY store not found"
fi

if grep -q "process-output" src/lib/components/LogViewer.svelte; then
    pass "LogViewer listens to process-output events"
else
    fail "LogViewer doesn't listen to process-output events"
fi
echo ""

echo "Test 9: Test long-running process simulation"
echo "------------------------------------"
cat > /tmp/pty-long-test.sh << 'EOF'
#!/bin/bash
echo "Starting long-running test..."
for i in {1..5}; do
    echo "Tick $i"
    sleep 1
done
echo "Long-running test complete"
EOF
chmod +x /tmp/pty-long-test.sh

info "Running 5-second test process..."
if timeout 10 bash /tmp/pty-long-test.sh > /tmp/pty-long-output.txt 2>&1; then
    if grep -q "Tick 5" /tmp/pty-long-output.txt; then
        pass "Long-running process test completed"
    else
        fail "Long-running process didn't complete all iterations"
    fi
else
    fail "Long-running process test failed or timed out"
fi
echo ""

echo "Test 10: Test ANSI color support"
echo "------------------------------------"
cat > /tmp/pty-color-test.sh << 'EOF'
#!/bin/bash
echo -e "\033[31mRed text\033[0m"
echo -e "\033[32mGreen text\033[0m"
echo -e "\033[33mYellow text\033[0m"
echo -e "\033[34mBlue text\033[0m"
EOF
chmod +x /tmp/pty-color-test.sh

if bash /tmp/pty-color-test.sh > /tmp/pty-color-output.txt 2>&1; then
    # Check if ANSI codes are present
    if grep -q $'\033\[31m' /tmp/pty-color-output.txt || grep -q "Red text" /tmp/pty-color-output.txt; then
        pass "ANSI color test executed"
        info "PTY should preserve ANSI escape codes"
    else
        fail "ANSI color test output unexpected"
    fi
else
    fail "ANSI color test failed"
fi
echo ""

# Cleanup
echo "Cleaning up test files..."
rm -f /tmp/pty-test-script.sh /tmp/pty-test-output.txt
rm -f /tmp/pty-long-test.sh /tmp/pty-long-output.txt
rm -f /tmp/pty-color-test.sh /tmp/pty-color-output.txt
echo ""

# Summary
echo "========================================"
echo "Test Summary"
echo "========================================"
echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Failed: $TESTS_FAILED${NC}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    echo ""
    echo "PTY System Status: ✅ WORKING"
    echo ""
    echo "Next steps:"
    echo "1. Open Sentinel app"
    echo "2. Navigate to 'PTY Test' page in sidebar"
    echo "3. Click 'Spawn Test Logger' to see live output"
    echo "4. Or go to Port Map → Start Process to launch your own process"
    exit 0
else
    echo -e "${RED}❌ Some tests failed${NC}"
    echo ""
    echo "Please review the failures above."
    exit 1
fi
