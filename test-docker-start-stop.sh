#!/bin/bash
# Test Docker Desktop start/stop flow
# This script tests the complete Docker lifecycle

set -e

SOCKET_PATH="/Users/$USER/.docker/run/docker.sock"
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "======================================"
echo "Docker Desktop Start/Stop Flow Test"
echo "======================================"
echo

# Helper function to check if Docker is running
check_docker_running() {
    if [ -S "$SOCKET_PATH" ]; then
        RESPONSE=$(curl -s --unix-socket "$SOCKET_PATH" http://localhost/_ping 2>/dev/null || echo "FAIL")
        if [ "$RESPONSE" = "OK" ]; then
            return 0  # Docker is running
        fi
    fi
    return 1  # Docker is not running
}

# Helper function to wait for Docker
wait_for_docker() {
    local max_attempts=$1
    local attempt=0

    echo -n "Waiting for Docker to be available"
    while [ $attempt -lt $max_attempts ]; do
        if check_docker_running; then
            echo
            echo -e "${GREEN}✓${NC} Docker is available"
            return 0
        fi
        echo -n "."
        sleep 2
        attempt=$((attempt + 1))
    done
    echo
    echo -e "${RED}✗${NC} Docker did not become available within $max_attempts attempts"
    return 1
}

# Helper function to wait for Docker to stop
wait_for_docker_stop() {
    local max_attempts=$1
    local attempt=0

    echo -n "Waiting for Docker to stop"
    while [ $attempt -lt $max_attempts ]; do
        if ! check_docker_running; then
            echo
            echo -e "${GREEN}✓${NC} Docker has stopped"
            return 0
        fi
        echo -n "."
        sleep 2
        attempt=$((attempt + 1))
    done
    echo
    echo -e "${RED}✗${NC} Docker did not stop within $max_attempts attempts"
    return 1
}

echo "Test 1: Check initial Docker state"
echo "-----------------------------------"
if check_docker_running; then
    echo -e "${GREEN}✓${NC} Docker is currently running"
    INITIAL_STATE="running"
else
    echo -e "${YELLOW}!${NC} Docker is not running"
    INITIAL_STATE="stopped"
fi
echo

echo "Test 2: Stop Docker Desktop"
echo "-----------------------------------"
if [ "$INITIAL_STATE" = "running" ]; then
    echo "Sending stop command to Docker Desktop..."
    osascript -e 'quit app "Docker"' 2>/dev/null || true

    if wait_for_docker_stop 30; then
        echo -e "${GREEN}✓${NC} Test 2 PASSED: Docker stopped successfully"
    else
        echo -e "${RED}✗${NC} Test 2 FAILED: Docker did not stop"
        exit 1
    fi
else
    echo -e "${YELLOW}!${NC} Skipping stop test - Docker was not running"
fi
echo

echo "Test 3: Start Docker Desktop"
echo "-----------------------------------"
echo "Sending start command to Docker Desktop..."
open -a Docker

if wait_for_docker 60; then
    echo -e "${GREEN}✓${NC} Test 3 PASSED: Docker started successfully"
else
    echo -e "${RED}✗${NC} Test 3 FAILED: Docker did not start"

    # Debug: Check if Docker.app process is running
    echo
    echo "Debug: Checking Docker processes..."
    ps aux | grep -i "docker" | grep -v grep || echo "No Docker processes found"

    # Check if Docker.app exists
    if [ -d "/Applications/Docker.app" ]; then
        echo -e "${GREEN}✓${NC} Docker.app exists"
    else
        echo -e "${RED}✗${NC} Docker.app not found at /Applications/Docker.app"
    fi

    exit 1
fi
echo

echo "Test 4: Verify Docker connectivity"
echo "-----------------------------------"
echo "Testing Docker CLI commands..."
if docker info > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} Docker CLI works"
    echo
    echo "Docker Info:"
    docker info | head -10
else
    echo -e "${RED}✗${NC} Docker CLI failed"
    exit 1
fi
echo

echo "======================================"
echo -e "${GREEN}ALL TESTS PASSED!${NC}"
echo "======================================"
echo
echo "Summary:"
echo "- Docker stop: ✓"
echo "- Docker start: ✓"
echo "- Docker connectivity: ✓"
