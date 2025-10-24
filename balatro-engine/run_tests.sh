#!/bin/bash
# Test script for Balatro Engine automated testing

set -e

echo "=== Balatro Engine Automated Testing ==="
echo

# Build the project first
echo "Building project..."
cargo build --release

echo
echo "Running test scenarios..."
echo

# Test 1: Quick Start
echo "1. Quick Start Test"
echo "   Starting game and playing first blind..."
BALATRO_INPUT_FILE=test_inputs/quick_start.txt cargo run --release --quiet
echo "   ✓ Quick Start Test completed"
echo

# Test 2: Skip Small Blind
echo "2. Skip Small Blind Test"
echo "   Starting game, skipping small blind, playing big blind..."
BALATRO_INPUT_FILE=test_inputs/skip_small_blind.txt cargo run --release --quiet
echo "   ✓ Skip Small Blind Test completed"
echo

# Test 3: Complete Ante
echo "3. Complete Ante Test"
echo "   Playing all three blinds in sequence..."
BALATRO_INPUT_FILE=test_inputs/complete_ante.txt cargo run --release --quiet
echo "   ✓ Complete Ante Test completed"
echo

# Test 4: Deck Selection
echo "4. Deck Selection Test"
echo "   Testing different deck types..."
BALATRO_INPUT_FILE=test_inputs/deck_selection.txt cargo run --release --quiet
echo "   ✓ Deck Selection Test completed"
echo

# Test 5: Stake Selection
echo "5. Stake Selection Test"
echo "   Testing different stake levels..."
BALATRO_INPUT_FILE=test_inputs/stake_selection.txt cargo run --release --quiet
echo "   ✓ Stake Selection Test completed"
echo

# Test 6: Mixed Strategy
echo "6. Mixed Strategy Test"
echo "   Testing mixed skip/play strategy..."
BALATRO_INPUT_FILE=test_inputs/mixed_strategy.txt cargo run --release --quiet
echo "   ✓ Mixed Strategy Test completed"
echo

echo "=== All tests completed successfully! ==="
