#!/bin/bash
# Setup script to build the Rust extension and prepare the test environment

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENGINE_DIR="${SCRIPT_DIR}/../balatro-engine"

echo "Building balatro-engine with Python feature..."

cd "${ENGINE_DIR}"
cargo build --features python --release

# Determine the extension module name based on the platform
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    EXTENSION_NAME="libbalatro_engine.dylib"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    EXTENSION_NAME="libbalatro_engine.so"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
    # Windows
    EXTENSION_NAME="balatro_engine.dll"
else
    echo "Unknown OS type: $OSTYPE"
    exit 1
fi

BUILD_PATH="${ENGINE_DIR}/target/release/${EXTENSION_NAME}"

if [ ! -f "${BUILD_PATH}" ]; then
    echo "Error: Extension module not found at ${BUILD_PATH}"
    echo "Build may have failed. Check the cargo output above."
    exit 1
fi

echo "Extension module built at: ${BUILD_PATH}"

# Create a symlink or copy to the test directory for easy import
TEST_DIR="${SCRIPT_DIR}/tests"
if [ -d "${TEST_DIR}" ]; then
    # Copy the extension to the tests directory for easier testing
    cp "${BUILD_PATH}" "${TEST_DIR}/balatro_engine.so" 2>/dev/null || \
    cp "${BUILD_PATH}" "${TEST_DIR}/balatro_engine.dylib" 2>/dev/null || \
    cp "${BUILD_PATH}" "${TEST_DIR}/balatro_engine.dll" 2>/dev/null || true
    
    echo "Extension module copied to tests directory"
fi

echo ""
echo "Setup complete! To run tests:"
echo "  cd ${SCRIPT_DIR}"
echo "  export PYTHONPATH=\"${ENGINE_DIR}/target/release:\$PYTHONPATH\""
echo "  pytest"

