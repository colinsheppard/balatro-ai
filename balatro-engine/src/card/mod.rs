//! Card system for Balatro game engine

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Suit of a playing card
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

/// Rank of a playing card
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

/// Card enhancements that modify scoring
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Enhancement {
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

/// Card editions that provide special effects
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Edition {
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

/// Card seals that provide passive effects
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Seal {
    Red,
    Blue,
    Purple,
    Gold,
}

/// A playing card in the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub id: Uuid,
    pub suit: Suit,
    pub rank: Rank,
    pub enhancement: Option<Enhancement>,
    pub edition: Edition,
    pub seal: Option<Seal>,
}

impl Card {
    /// Create a new basic card
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self {
            id: Uuid::new_v4(),
            suit,
            rank,
            enhancement: None,
            edition: Edition::Base,
            seal: None,
        }
    }

    /// Get the base chip value of this card
    pub fn chip_value(&self) -> i32 {
        self.rank as i32
    }

    /// Get the base multiplier value of this card
    pub fn mult_value(&self) -> f32 {
        1.0
    }

    /// Check if this card is a face card (Jack, Queen, King)
    pub fn is_face_card(&self) -> bool {
        matches!(self.rank, Rank::Jack | Rank::Queen | Rank::King)
    }

    /// Check if this card is an ace
    pub fn is_ace(&self) -> bool {
        self.rank == Rank::Ace
    }
}
