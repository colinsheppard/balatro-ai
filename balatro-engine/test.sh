#!/bin/bash
# Individual test runner for Balatro Engine

if [ $# -eq 0 ]; then
    echo "Usage: $0 <test_name>"
    echo "Available tests:"
    echo "  quick_start"
    echo "  skip_small_blind"
    echo "  complete_ante"
    echo "  deck_selection"
    echo "  stake_selection"
    echo "  mixed_strategy"
    exit 1
fi

TEST_NAME=$1
INPUT_FILE="test_inputs/${TEST_NAME}.txt"

if [ ! -f "$INPUT_FILE" ]; then
    echo "Error: Test file $INPUT_FILE not found"
    exit 1
fi

echo "Running test: $TEST_NAME"
echo "Input file: $INPUT_FILE"
echo

# Build if needed
if [ ! -f "target/release/balatro-engine" ]; then
    echo "Building project..."
    cargo build --release
fi

# Run the test
BALATRO_INPUT_FILE="$INPUT_FILE" cargo run --release --quiet

echo
echo "Test completed: $TEST_NAME"
