# TOML-Based Joker Configuration System

The Balatro game engine now supports loading joker definitions from TOML configuration files, providing a flexible and data-driven approach to defining joker effects and behaviors.

## Overview

The joker system has been refactored to use TOML configuration files instead of hardcoded implementations. This allows for:

- **Easy modification** of joker effects without code changes
- **Data-driven design** for game balancing
- **Extensible effect system** supporting complex interactions
- **Runtime loading** of joker definitions
- **Validation** of configuration integrity

## Architecture

### Core Components

1. **JokerConfig** - Main configuration structure
2. **JokerDefinition** - Individual joker definition
3. **JokerInstance** - Runtime joker instance with state
4. **JokerManager** - Loads and manages joker definitions
5. **Effect System** - Handles different types of joker effects

### File Structure

```
src/joker/
├── mod.rs          # Main joker module with legacy compatibility
├── config.rs       # TOML data structures and deserialization
└── loader.rs       # JokerManager for loading and validation
```

## TOML Schema

### Basic Structure

```toml
schema_version = "1.0.0"

[[jokers]]
id = "joker_id"
name = "Joker Name"
description = "Joker description"
rarity = "common"  # common, uncommon, rare, legendary
cost = 3

[jokers.effect]
type = "scoring"  # scoring, conditional, dynamic, calculate, special
mult = 4
```

### Effect Types

#### 1. Scoring Effects
Simple additive effects that provide fixed bonuses:

```toml
[jokers.effect]
type = "scoring"
mult = 4        # +4 Mult
chips = 10      # +10 Chips
mult_multiplier = 2.0  # ×2 Mult
chips_multiplier = 1.5 # ×1.5 Chips
```

#### 2. Conditional Effects
Effects that trigger based on specific conditions:

```toml
[jokers.effect]
type = "conditional"
per_card = true  # Apply per card or per hand

[jokers.effect.condition]
type = "suit_scored"
suit = "diamonds"

[jokers.effect.action]
type = "add_score"
mult = 3
```

**Condition Types:**
- `hand_type` - Specific poker hands (pair, flush, etc.)
- `suit_scored` - Cards of specific suit
- `rank_scored` - Cards of specific rank
- `face_card_scored` - Face cards (J, Q, K)
- `no_face_cards` - Hands without face cards
- `state_value` - Game state conditions
- `hand_size` - Hand size conditions
- `any` - Multiple conditions (OR logic)

#### 3. Dynamic Effects
Effects that change based on game state:

```toml
[jokers.effect]
type = "dynamic"

[jokers.effect.base_effect]
type = "add_score"
chips = 100

[[jokers.effect.state_modifiers]]
state_field = "hands_played"
multiplier = -5.0

[jokers.state]
persistent = true

[jokers.state.fields]
hands_played = 0

[jokers.behavior]
[jokers.behavior.on_hand_played]
type = "modify_state"
field = "hands_played"
operation = "increment"
value = 1
```

#### 4. Calculate Effects
Effects that use formulas for dynamic calculations:

```toml
[jokers.effect]
type = "calculate"
formula = "discards_remaining * 40"
result_type = "chips"
```

#### 5. Special Effects
Unique effects that don't fit other categories:

```toml
[jokers.effect]
type = "special"
special_type = "hand_type_modifier"

[jokers.effect.parameters]
flush_requirement = 4
straight_requirement = 4
```

## Usage Examples

### Loading from File

```rust
use balatro_engine::joker::JokerManager;

let manager = JokerManager::from_file("joker_definitions.toml")?;
manager.validate()?;

let joker = manager.create_joker("joker_id")?;
```

### Loading from String

```rust
let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "test_joker"
name = "Test Joker"
description = "+4 Mult"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4
"#;

let manager = JokerManager::from_str(toml_content)?;
```

### Querying Jokers

```rust
// Get all joker IDs
let all_ids = manager.get_all_ids();

// Get jokers by rarity
let common_jokers = manager.get_by_rarity(JokerRarity::Common);

// Get jokers by cost range
let cheap_jokers = manager.get_by_cost_range(1, 5);

// Get specific definition
let definition = manager.get_definition("joker_id")?;
```

## State Management

Jokers can maintain persistent or temporary state:

```toml
[jokers.state]
persistent = true  # State persists between rounds

[jokers.state.fields]
hands_played = 0
cards_destroyed = 0
money_earned = 0
```

## Behavior Events

Jokers can respond to game events:

```toml
[jokers.behavior]
[jokers.behavior.on_hand_played]
type = "modify_state"
field = "hands_played"
operation = "increment"
value = 1

[jokers.behavior.on_round_end]
type = "sequence"
actions = [
    { type = "modify_state", field = "hands_played", operation = "set", value = 0 }
]
```

**Event Types:**
- `on_hand_played` - When a hand is played
- `on_round_end` - When a round ends
- `on_discard` - When cards are discarded
- `on_shop_open` - When shop opens

**Operations:**
- `increment` - Add to value
- `decrement` - Subtract from value
- `set` - Set to specific value
- `multiply` - Multiply by value
- `divide` - Divide by value

## Backward Compatibility

The system is now purely TOML-based with no legacy compatibility layer. All joker functionality is handled through the `JokerInstance` and `JokerManager` types.

## Validation

The system includes comprehensive validation:

```rust
let manager = JokerManager::from_str(toml_content)?;
manager.validate()?;  // Validates configuration integrity
```

**Validation checks:**
- Duplicate joker IDs
- Non-empty names and IDs
- Non-negative costs
- Valid effect configurations
- Proper condition/action pairs

## Example Configuration

See `examples/joker_toml_example.rs` for a complete working example that demonstrates:

- Loading joker definitions from TOML
- Creating joker instances
- Querying jokers by rarity
- State initialization
- Effect configuration

## Future Enhancements

The TOML-based system is designed to be extensible:

1. **Effect Engine** - Full implementation of effect processing
2. **Formula Parser** - Support for complex mathematical expressions
3. **Event System** - More granular game event handling
4. **Mod Support** - Loading custom joker definitions
5. **Hot Reloading** - Runtime configuration updates

## Testing

The system includes comprehensive tests covering:

- TOML parsing and validation
- Joker creation and management
- Effect configuration
- State management
- Integration with game state
- Error handling

Run tests with:
```bash
cargo test
```

## Performance Considerations

- TOML parsing is done once at startup
- Joker instances are lightweight runtime objects
- State is stored efficiently using `HashMap<String, Value>`
- Validation is optional and can be skipped in production
