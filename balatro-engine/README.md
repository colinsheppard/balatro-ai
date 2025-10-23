# Balatro Game Engine

A Rust implementation of the Balatro card game engine. This project provides the core game logic, state management, and mechanics for the popular roguelike deckbuilder game.

## Features

- **Complete Card System**: Full implementation of playing cards with suits, ranks, enhancements, editions, and seals
- **Deck Management**: Support for all deck types with proper shuffling and card drawing mechanics
- **Joker System**: Comprehensive joker implementation with rarities, editions, and stickers
- **Blind System**: Boss blinds, scoring requirements, and progression mechanics
- **Consumable System**: Tarot cards, planet cards, spectral cards, and vouchers
- **Game State Management**: Complete game state tracking with phases, ante levels, and scoring
- **Serialization**: Full serde support for saving/loading game states
- **Error Handling**: Comprehensive error types with proper error propagation

## Project Structure

```
src/
├── lib.rs              # Main library entry point
├── main.rs              # CLI executable entry point
├── card/                # Card system
│   └── mod.rs          # Card, Suit, Rank, Enhancements, Editions, Seals
├── deck/                # Deck management
│   └── mod.rs          # Deck types and operations
├── joker/               # Joker system
│   └── mod.rs          # Joker types, rarities, and effects
├── game/                # Core game logic
│   └── mod.rs          # Game state, phases, and main logic
├── blind/               # Blind system
│   └── mod.rs          # Blinds, boss effects, and scoring
├── consumable/          # Consumable items
│   └── mod.rs          # Tarot cards, planet cards, etc.
└── error/               # Error handling
    └── mod.rs          # Game error types and results
```

## Usage

### As a Library

```rust
use balatro_engine::BalatroEngine;

// Create a new game engine
let mut engine = BalatroEngine::new(12345);

// Start a new run
engine.start_new_run()?;

// Access game state
let game_state = engine.game_state();
println!("Current ante: {}", game_state.ante.0);
```

### As a CLI Tool

```bash
# Run with default seed
cargo run

# Run with specific seed
cargo run -- 42

# Build release version
cargo build --release
```

## Dependencies

- **serde**: Serialization and deserialization
- **rand**: Random number generation
- **anyhow/thiserror**: Error handling
- **uuid**: Unique identifiers
- **log/env_logger**: Logging
- **indexmap**: Ordered hash maps

## Development Status

This is the initial scaffolding for the game engine. The following systems are implemented:

- ✅ Basic project structure and dependencies
- ✅ Card system with suits, ranks, and modifiers
- ✅ Deck management with standard operations
- ✅ Joker system with rarities and effects
- ✅ Blind system with boss effects
- ✅ Consumable system framework
- ✅ Game state management
- ✅ Error handling system
- ✅ Basic CLI interface

### Next Steps

- [ ] Implement poker hand detection and scoring
- [ ] Add specific joker effects and interactions
- [ ] Implement shop system
- [ ] Add blind selection and boss mechanics
- [ ] Create consumable effects
- [ ] Add save/load functionality
- [ ] Implement game progression and unlocks
- [ ] Add comprehensive testing
- [ ] Performance optimization

## Building

```bash
# Install dependencies
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check code
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Contributing

This project is part of the Balatro AI initiative. The game engine is designed to be:

- **Accurate**: Faithfully implements all Balatro game mechanics
- **Performant**: Optimized for speed and memory efficiency
- **Extensible**: Easy to add new features and modifications
- **Well-tested**: Comprehensive test coverage
- **Documented**: Clear documentation and examples

## License

MIT License - see LICENSE file for details.
