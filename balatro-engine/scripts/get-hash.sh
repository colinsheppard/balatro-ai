#!/bin/bash
# Get the current test binary hash for VS Code debugging

# Get the hash from cargo test --no-run output
HASH=$(cargo test --no-run --message-format=json 2>/dev/null | grep -o '"executable":"[^"]*balatro_engine-[^"]*"' | sed 's/.*balatro_engine-\([^\/]*\)".*/\1/' | head -1)

if [ -z "$HASH" ]; then
    # Fallback method
    HASH=$(cargo test --no-run 2>&1 | grep -o 'balatro_engine-[a-f0-9]*' | head -1 | sed 's/balatro_engine-//')
fi

if [ -z "$HASH" ]; then
    echo "Failed to get test hash"
    exit 1
fi

echo "$HASH"
