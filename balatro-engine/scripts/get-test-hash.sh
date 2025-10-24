#!/bin/bash
# Helper script to get the current test binary hash for VS Code debugging

echo "Building tests and extracting hash..."


HASH=$(cargo test --no-run --message-format=json 2>/dev/null | grep -o '"executable":"[^"]*balatro_engine-[^"]*"' | sed 's/.*balatro_engine-\([^\/]*\)".*/\1/' | head -1)

if [ -z "$HASH" ]; then
    echo "❌ Failed to extract hash. Make sure cargo test --no-run works."
    echo "Trying alternative method..."
    HASH=$(cargo test --no-run 2>&1 | grep -o 'balatro_engine-[a-f0-9]*' | head -1 | sed 's/balatro_engine-//')
fi

if [ -z "$HASH" ]; then
    echo "❌ Still failed to extract hash. Please run 'cargo test --no-run' manually."
    exit 1
fi

echo "✅ Current test binary hash: $HASH"
echo ""
echo "📋 Copy this hash and paste it when VS Code prompts for 'testHash'"
echo "   Or update the default value in .vscode/launch.json"
echo ""
echo "🔧 Quick update command:"
echo "   sed -i '' 's/\"default\": \"[^\"]*\"/\"default\": \"$HASH\"/' .vscode/launch.json"
