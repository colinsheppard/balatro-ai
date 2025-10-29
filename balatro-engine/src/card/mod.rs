//! Card system for Balatro game engine

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{cell::RefCell, fmt, rc::Rc};

/// Suit of a playing card
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

/// Rank of a playing card
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rank {
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
    Ace = 14,
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

/// Type alias for a card that can be shared across the game
pub type SharedCard = Rc<RefCell<Card>>;

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
        match self.rank {   
            Rank::Ace => 11,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            _ => self.rank as i32,
        }
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

// Display implementation for Rank
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        write!(f, "{}", rank_str)
    }
}

// Display implementation for Suit with unicode and ANSI color codes
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (symbol, color_code) = match self {
            Suit::Hearts => ('♥', "\x1b[31m"),    // Red
            Suit::Diamonds => ('♦', "\x1b[34m"),   // Blue
            Suit::Clubs => ('♣', "\x1b[32m"),      // Green
            Suit::Spades => ('♠', "\x1b[0m"),     // Default/White
        };
        write!(f, "{}{}{}", color_code, symbol, "\x1b[0m")
    }
}

// Display implementation for Card
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}
