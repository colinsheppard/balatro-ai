# Balatro Engine Automated Testing & Session Recording

This directory contains automated testing tools for the Balatro game engine, allowing you to test different game scenarios without manual input, and record your gameplay sessions for later replay.

## Features

- **Input Piping**: Automatically detects and uses piped input or input files
- **Hybrid Input Mode**: Start with file input, then switch to interactive mode when the file is exhausted
- **Environment Variable Support**: Use `BALATRO_INPUT_FILE` to specify input files
- **Session Recording**: Record your manual gameplay sessions for later replay
- **Multiple Test Scenarios**: Pre-configured test cases for different game situations
- **Easy-to-use Scripts**: Simple commands to run individual or all tests

## Usage

### Method 1: Using Test Scripts

Run all tests:
```bash
./run_tests.sh
```

Run a specific test:
```bash
./test.sh quick_start
./test.sh skip_small_blind
./test.sh complete_ante
./test.sh deck_selection
./test.sh stake_selection
./test.sh mixed_strategy
```

### Method 2: Using Environment Variables

```bash
BALATRO_INPUT_FILE=input/quick_start.txt cargo run --release
```

**Hybrid Input Mode**: When using `BALATRO_INPUT_FILE`, the game will read from the file until it reaches the end of the file, then automatically switch to interactive mode. This allows you to fast-forward to a specific game state and then take over manually.

Example:
```bash
# Create a partial input file that sets up the game
echo -e "24\n1\n1" > input/partial.txt

# Run with hybrid input: file first, then interactive
BALATRO_INPUT_FILE=input/partial.txt cargo run --release
```

### Method 3: Using Input Piping

```bash
echo -e "24\n1" | cargo run --release
```

### Method 4: Session Recording

Record your gameplay session:
```bash
cargo run --release -- --record
```

Replay a recorded session:
```bash
./replay.sh output/session_20241201_143022.txt
./replay.sh output/session_20241201_143022.txt 42  # with custom seed
```

## Session Recording

### Recording Sessions

To record your manual gameplay:

```bash
# Record with default seed
cargo run --release -- --record

# Record with custom seed
cargo run --release -- --record --seed 42
```

This will:
- Create a timestamped file in the `output/` directory
- Record all your input choices during gameplay
- Display the recording file path when you start

### Replaying Sessions

To replay a recorded session:

```bash
# List available sessions
ls output/session_*.txt

# Replay a specific session
./replay.sh output/session_20241201_143022.txt

# Replay with a different seed
./replay.sh output/session_20241201_143022.txt 42
```

### Session File Format

Recorded session files contain one number per line, representing your choices:

```
24    # Start Game
1     # Play Small Blind
2     # Skip Big Blind
1     # Play Boss Blind
```

## Available Test Scenarios

### `quick_start.txt`
- Starts the game
- Plays the first blind (Small Blind)
- Minimal test to verify basic functionality

### `skip_small_blind.txt`
- Starts the game
- Skips the Small Blind
- Plays the Big Blind
- Tests skip functionality

### `complete_ante.txt`
- Plays all three blinds in sequence (Small → Big → Boss)
- Tests complete ante progression

### `deck_selection.txt`
- Tests different deck type selections
- Verifies deck switching functionality

### `stake_selection.txt`
- Tests different stake level selections
- Verifies stake switching functionality

### `mixed_strategy.txt`
- Tests mixed skip/play strategy
- Skips Small Blind, plays Big Blind, attempts to skip Boss

## Command Line Options

- `--record`: Enable session recording
- `--seed <number>`: Set random seed
- `<number>`: Set random seed (backward compatibility)

Examples:
```bash
cargo run --release -- --record --seed 42
cargo run --release -- 42  # backward compatibility
```

## Environment Variables

- `BALATRO_INPUT_FILE`: Path to input file for automated testing
- `BALATRO_RECORD`: Enable session recording (set automatically by --record flag)
- If not set, the application will check for piped input or run interactively

## Building and Running

Make sure to build the project first:
```bash
cargo build --release
```

Then run tests using any of the methods above.

## Adding New Test Scenarios

1. Create a new input file in `input/` directory
2. Add the test name to `test.sh` script
3. Add the test to `run_tests.sh` if desired

Example new test file:
```
# input/my_test.txt
24
1
2
1
```

## Session Management

### Viewing Recorded Sessions

```bash
# List all recorded sessions
ls -la output/session_*.txt

# View session contents
cat output/session_20241201_143022.txt
```

### Organizing Sessions

You can organize your sessions by:
- Moving interesting sessions to a separate folder
- Renaming sessions with descriptive names
- Creating symbolic links to frequently replayed sessions

Example:
```bash
mkdir -p output/interesting_sessions
mv output/session_20241201_143022.txt output/interesting_sessions/complete_ante_run.txt
```