#!/bin/bash

# Test logger script that outputs to both stdout and stderr
echo "Starting test logger..."
sleep 1
echo "INFO: Test service started on port 3000"
sleep 1
echo "DEBUG: Database connection established"
sleep 1
echo "ERROR: Failed to load configuration file" >&2
sleep 1
echo "INFO: Listening on http://localhost:3000"
sleep 1
echo "WARN: Deprecated API endpoint used" >&2
sleep 1
echo "INFO: Request received: GET /api/users"
sleep 1
echo "DEBUG: Query executed in 45ms"
sleep 1
echo "INFO: Response sent: 200 OK"
sleep 1

# Loop forever
while true; do
  sleep 2
  echo "INFO: Heartbeat - $(date)"
  sleep 2
  echo "DEBUG: Memory usage: $(( RANDOM % 100 ))MB"
  sleep 2
  if [ $(( RANDOM % 5 )) -eq 0 ]; then
    echo "ERROR: Random error occurred!" >&2
  fi
done
