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

// Re-export commonly used types
pub use card::*;
pub use deck::*;
pub use joker::*;
pub use game::*;
pub use blind::*;
pub use consumable::*;
pub use error::*;
pub use stakes::*;

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
}

#[cfg(test)]
mod test;