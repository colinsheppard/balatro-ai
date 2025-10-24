#!/bin/bash
# Script to replay recorded Balatro sessions

if [ $# -eq 0 ]; then
    echo "Usage: $0 <session_file> [seed]"
    echo
    echo "Available session files:"
    ls -la output/session_*.txt 2>/dev/null | sed 's/.*output\//  /' || echo "  No session files found"
    echo
    echo "Examples:"
    echo "  $0 output/session_20241201_143022.txt"
    echo "  $0 output/session_20241201_143022.txt 42"
    exit 1
fi

SESSION_FILE="$1"
SEED="${2:-12345}"

if [ ! -f "$SESSION_FILE" ]; then
    echo "Error: Session file '$SESSION_FILE' not found"
    exit 1
fi

echo "Replaying session: $SESSION_FILE"
echo "Using seed: $SEED"
echo

# Build if needed
if [ ! -f "target/release/balatro-engine" ]; then
    echo "Building project..."
    cargo build --release
fi

# Replay the session
BALATRO_INPUT_FILE="$SESSION_FILE" cargo run --release --quiet -- --seed "$SEED"

echo
echo "Session replay completed: $SESSION_FILE"
