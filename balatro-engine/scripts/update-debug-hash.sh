#!/bin/bash
# Auto-update debug hash and launch VS Code debugger
# Run this from the balatro-engine directory

# Get current hash for library tests (which contains all tests)
HASH=$(cargo test --no-run --message-format=json 2>/dev/null | grep -o '"executable":"[^"]*balatro_engine-[^"]*"' | grep "src/lib.rs" | sed 's/.*balatro_engine-\([^\/]*\)".*/\1/' | head -1)

if [ -z "$HASH" ]; then
    # Fallback method - get any balatro_engine hash
    HASH=$(cargo test --no-run --message-format=json 2>/dev/null | grep -o '"executable":"[^"]*balatro_engine-[^"]*"' | sed 's/.*balatro_engine-\([^\/]*\)".*/\1/' | head -1)
fi

if [ -z "$HASH" ]; then
    echo "Failed to get test hash"
    exit 1
fi

# Use a more precise sed command that only targets the testHash input
# This looks for the testHash input block and only updates the default within that block
sed -i '' '
/^[[:space:]]*{[[:space:]]*$/,/^[[:space:]]*}[[:space:]]*$/ {
    /"id": "testHash"/,/"type": "promptString"/ {
        s/"default": "[^"]*"/"default": "'"$HASH"'"/g
    }
}
' ../.vscode/launch.json

echo "Updated debug hash to: $HASH"
echo "Now you can use 'Debug Specific Test' and only enter the test name!"