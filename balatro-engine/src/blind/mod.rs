//! Blind system for Balatro game engine

use serde::{Deserialize, Serialize};

/// Types of blinds in the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlindType {
    Small,
    Big,
    Boss,
    Finisher,
}

/// Status of a blind in the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlindStatus {
    Upcoming,
    Active,
    Complete,
    Skipped,
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
    pub status: BlindStatus,
}

/// Container for the three blinds in an ante: Small, Big, and Boss
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpcomingBlinds {
    pub small: Blind,
    pub big: Blind,
    pub boss: Blind,
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
            status: BlindStatus::Upcoming,
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
            status: BlindStatus::Upcoming,
        }
    }

    /// Check if this blind can be skipped
    pub fn can_skip(&self) -> bool {
        matches!(self.blind_type, BlindType::Small | BlindType::Big)
    }
}

impl UpcomingBlinds {
    /// Create a new UpcomingBlinds with the three blinds for an ante
    pub fn new(small: Blind, big: Blind, boss: Blind) -> Self {
        Self { small, big, boss }
    }

    /// Get a reference to a blind by type
    pub fn get_blind(&self, blind_type: BlindType) -> Option<&Blind> {
        match blind_type {
            BlindType::Small => Some(&self.small),
            BlindType::Big => Some(&self.big),
            BlindType::Boss => Some(&self.boss),
            BlindType::Finisher => None, // Finisher blinds are not part of UpcomingBlinds
        }
    }

    /// Get a mutable reference to a blind by type
    pub fn get_blind_mut(&mut self, blind_type: BlindType) -> Option<&mut Blind> {
        match blind_type {
            BlindType::Small => Some(&mut self.small),
            BlindType::Big => Some(&mut self.big),
            BlindType::Boss => Some(&mut self.boss),
            BlindType::Finisher => None, // Finisher blinds are not part of UpcomingBlinds
        }
    }

    /// Get the next upcoming blind (first one with Upcoming status)
    pub fn get_next_upcoming_blind(&self) -> Option<&Blind> {
        if self.small.status == BlindStatus::Upcoming {
            Some(&self.small)
        } else if self.big.status == BlindStatus::Upcoming {
            Some(&self.big)
        } else if self.boss.status == BlindStatus::Upcoming {
            Some(&self.boss)
        } else {
            None
        }
    }

    /// Get a mutable reference to the next upcoming blind
    pub fn get_next_upcoming_blind_mut(&mut self) -> Option<&mut Blind> {
        if self.small.status == BlindStatus::Upcoming {
            Some(&mut self.small)
        } else if self.big.status == BlindStatus::Upcoming {
            Some(&mut self.big)
        } else if self.boss.status == BlindStatus::Upcoming {
            Some(&mut self.boss)
        } else {
            None
        }
    }
}
