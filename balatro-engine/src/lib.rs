//! Balatro Game Engine
//! 
//! A Rust implementation of the Balatro card game engine.
//! This module provides the core game logic, state management, and mechanics.

pub mod card;
pub mod deck;
pub mod joker;
pub mod game;
pub mod blind;
pub mod consumable;
pub mod error;
pub mod stakes;
pub mod hand;
pub mod actions;

// Re-export commonly used types
pub use card::*;
pub use deck::*;
pub use joker::*;
pub use game::*;
pub use blind::*;
pub use consumable::*;
pub use error::*;
pub use stakes::*;
pub use hand::*;
pub use actions::*;

/// The main game engine that orchestrates all game systems
pub struct BalatroEngine {
    game_state: GameState,
    random_seed: u64,
}

impl BalatroEngine {
    /// Create a new game engine with the given seed
    pub fn new(seed: u64) -> Self {
        Self {
            game_state: GameState::new(),
            random_seed: seed,
        }
    }

    /// Initialize a new game run with default settings
    pub fn start_new_default_run(&mut self) -> Result<(), GameError> {
        self.game_state = GameState::new();
        Ok(())
    }

    /// Initialize a new game run
    pub fn start_new_run(&mut self, deck_type: DeckType, stake_level: StakeLevel) -> Result<(), GameError> {
        self.game_state = GameState::new_with_settings(deck_type, stake_level);
        Ok(())
    }

    /// Get the current game state
    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    /// Get mutable access to the game state
    pub fn game_state_mut(&mut self) -> &mut GameState {
        &mut self.game_state
    }

    /// Get all available deck types
    pub fn available_deck_types(&self) -> Vec<DeckType> {
        use actions::helpers::all_deck_types;
        all_deck_types()
    }

    /// Get all available stake levels
    pub fn available_stake_levels(&self) -> Vec<StakeLevel> {
        use actions::helpers::all_stake_levels;
        all_stake_levels()
    }

    /// Get deck type description
    pub fn get_deck_type_description(&self, deck_type: &DeckType) -> String {
        match deck_type {
            DeckType::Red => "Red Deck - Standard deck with no special effects".to_string(),
            DeckType::Blue => "Blue Deck - Enhanced card values".to_string(),
            DeckType::Yellow => "Yellow Deck - Bonus money rewards".to_string(),
            DeckType::Green => "Green Deck - Extra hands per round".to_string(),
            DeckType::Black => "Black Deck - Higher risk, higher reward".to_string(),
            DeckType::Magic => "Magic Deck - Random card effects".to_string(),
            DeckType::Nebula => "Nebula Deck - Cosmic-themed effects".to_string(),
            DeckType::Ghost => "Ghost Deck - Spectral card abilities".to_string(),
            DeckType::Abandoned => "Abandoned Deck - Forgotten card powers".to_string(),
            DeckType::Checkered => "Checkered Deck - Alternating effects".to_string(),
            DeckType::Zodiac => "Zodiac Deck - Astrological influences".to_string(),
            DeckType::Painted => "Painted Deck - Artistic card modifications".to_string(),
            DeckType::Anaglyph => "Anaglyph Deck - 3D visual effects".to_string(),
            DeckType::Plasma => "Plasma Deck - Energy-based mechanics".to_string(),
            DeckType::Erratic => "Erratic Deck - Unpredictable behavior".to_string(),
        }
    }

    /// Get stake level description
    pub fn get_stake_level_description(&self, stake_level: &StakeLevel) -> String {
        match stake_level {
            StakeLevel::White => "White Stake - Base difficulty with no modifiers".to_string(),
            StakeLevel::Red => "Red Stake - Blind scores increased by 10%".to_string(),
            StakeLevel::Green => "Green Stake - Skip costs increased by $2".to_string(),
            StakeLevel::Blue => "Blue Stake - Blind scores +20%, skip costs +$3".to_string(),
            StakeLevel::Black => "Black Stake - Money rewards reduced by 25%".to_string(),
            StakeLevel::Purple => "Purple Stake - Joker costs increased by 50%".to_string(),
            StakeLevel::Orange => "Orange Stake - One less hand per round".to_string(),
            StakeLevel::Gold => "Gold Stake - All previous modifiers combined".to_string(),
        }
    }
}

#[cfg(test)]
mod test;