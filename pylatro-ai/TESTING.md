# Testing Guide

## Quick Start

1. Build the Rust extension:
   ```bash
   cd ../balatro-engine
   cargo build --features python --release
   ```

2. Set up Python environment:
   ```bash
   cd ../pylatro-ai
   pip install -e ".[dev]"
   ```

3. Run tests:
   ```bash
   export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
   pytest -v
   ```

## Test Coverage

The test suite includes:

### `test_import.py`
- Module import verification
- Class availability checks
- Class callability verification

### `test_game_state.py`
- GameState class existence
- Expected method availability
- Supporting types (GamePhase, DeckType, StakeLevel) creation and validation

### `test_actions.py`
- All Action type classes (MenuAction, ShopAction, BlindAction, PlayingAction, RoundEndAction, GameOverAction)
- Action creation with various parameters
- Action method verification (index, description, is_valid)
- String representation tests
- Input validation tests

## Test Structure

All tests use `pytest.skip()` when the `balatro_engine` module is not available, so the test suite can be run even if the Rust extension hasn't been built yet (tests will be skipped with appropriate messages).

The `conftest.py` file provides a session-scoped fixture that attempts to locate and import the extension module, making it easier to write tests that depend on the module being available.

