# pylatro-ai

Python bindings and test suite for the `balatro-engine` Rust library.

## Setup

### Prerequisites

- Python 3.8 or higher
- Rust toolchain (for building the engine)
- The `balatro-engine` crate must be built with the `python` feature enabled

### Building the Rust Extension

Before running tests, you need to build the `balatro-engine` Rust library as a Python extension module:

**Option 1: Using the setup script (recommended)**
```bash
cd pylatro-ai
./setup_test_env.sh
```

**Option 2: Manual build**
```bash
cd ../balatro-engine
cargo build --features python --release
```

The built extension module will be located at:
- Linux: `balatro-engine/target/release/libbalatro_engine.so`
- macOS: `balatro-engine/target/release/libbalatro_engine.dylib`
- Windows: `balatro-engine/target/release/balatro_engine.dll`

### Installing Python Dependencies

```bash
pip install -e .
```

Or with development dependencies:

```bash
pip install -e ".[dev]"
```

### Running Tests

The test suite will automatically locate the extension module if it's in the standard build location. To run tests:

```bash
# Make sure PYTHONPATH includes the build directory
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest
```

Or use the setup script which handles this automatically:

```bash
./setup_test_env.sh
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest
```

## Project Structure

```
pylatro-ai/
├── pylatro_ai/          # Python package
├── tests/               # Test suite
│   ├── test_import.py   # Tests for module import
│   ├── test_game_state.py  # Tests for GameState
│   └── test_actions.py   # Tests for Action types
├── pyproject.toml        # Project configuration
└── README.md            # This file
```

## Tests

The test suite verifies:

1. **Module Import**: That `balatro_engine` can be imported successfully
2. **GameState Access**: That `GameState` class and its methods are accessible
3. **Action Types**: That all Action types (MenuAction, ShopAction, BlindAction, PlayingAction, RoundEndAction, GameOverAction) are accessible
4. **Supporting Types**: That supporting types (GamePhase, DeckType, StakeLevel) are accessible
5. **Functionality**: Basic functionality of the exposed types

