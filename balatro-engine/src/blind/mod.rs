//! Blind system for Balatro game engine

use serde::{Deserialize, Serialize};
use crate::error::{GameError, GameResult};

/// Types of blinds in the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlindType {
    Small,
    Big,
    Boss,
    Finisher,
}

/// Boss blind effects
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BossEffect {
    // Add specific boss effects here as we implement them
    None,
}

/// A blind that the player must overcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blind {
    pub name: String,
    pub blind_type: BlindType,
    pub required_score: i32,
    pub reward_money: i32,
    pub boss_effect: Option<BossEffect>,
}

impl Blind {
    /// Create a new blind
    pub fn new(name: String, blind_type: BlindType, required_score: i32, reward_money: i32) -> Self {
        Self {
            name,
            blind_type,
            required_score,
            reward_money,
            boss_effect: None,
        }
    }

    /// Create a boss blind with an effect
    pub fn new_boss(name: String, required_score: i32, reward_money: i32, effect: BossEffect) -> Self {
        Self {
            name,
            blind_type: BlindType::Boss,
            required_score,
            reward_money,
            boss_effect: Some(effect),
        }
    }

    /// Check if this blind can be skipped
    pub fn can_skip(&self) -> bool {
        matches!(self.blind_type, BlindType::Small | BlindType::Big)
    }
}
