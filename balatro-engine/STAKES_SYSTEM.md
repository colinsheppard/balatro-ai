# Stakes System

The stakes system provides difficulty modifiers for the Balatro game engine. Unlike the original game, all stakes are immediately available for selection without requiring unlocking.

## Stake Levels

The system includes 8 stake levels, each with increasing difficulty:

1. **White Stake** - Base difficulty (1.0x multiplier)
2. **Red Stake** - Blind scores increased by 10% (1.1x multiplier)
3. **Green Stake** - Skip costs increased by $2 (1.2x multiplier)
4. **Blue Stake** - Blind scores +20%, skip costs +$3 (1.3x multiplier)
5. **Black Stake** - Money rewards reduced by 25% (1.4x multiplier)
6. **Purple Stake** - Joker costs increased by 50% (1.5x multiplier)
7. **Orange Stake** - One less hand per round (1.6x multiplier)
8. **Gold Stake** - All previous modifiers combined (1.7x multiplier)

## Modifiers

Each stake can modify various game parameters:

- **Blind Score Multiplier**: Affects required scores for blinds
- **Money Reward Multiplier**: Affects money rewards from blinds
- **Skip Cost Bonus**: Additional cost for skipping blinds
- **Joker Cost Multiplier**: Affects joker purchase costs
- **Consumable Cost Multiplier**: Affects consumable purchase costs
- **Hands per Round Bonus**: Additional/fewer hands per round
- **Discards per Round Bonus**: Additional/fewer discards per round
- **Starting Money Modifier**: Changes starting money
- **Starting Hand Size Modifier**: Changes starting hand size

## Usage

```rust
use balatro_engine::stakes::{StakeManager, StakeLevel};

// Create a stake manager
let manager = StakeManager::new();

// Get all available stakes
let all_stakes = manager.all_stakes();

// Get a specific stake
let white_stake = manager.get_stake(StakeLevel::White);

// Apply modifiers to game values
let modified_score = stake.modifiers.apply_to_score(base_score);
let modified_money = stake.modifiers.apply_to_money(base_money);
```

## Safety Features

The system includes safety checks to prevent invalid game states:

- Minimum 1 hand per round
- Minimum 0 discards per round
- Minimum 0 starting money
- Minimum 1 card in starting hand

## Testing

Run the stakes example to see the system in action:

```bash
cargo run --example stakes_example
```

All stakes are immediately available for selection, making the system suitable for game emulation and testing scenarios.
