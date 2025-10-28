//! Blind system for Balatro game engine

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use crate::error::{GameError, GameResult};

/// Types of blinds in the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlindType {
    Small,
    Big,
    Boss
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

/// Data structure for CSV row containing base required scores
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BaseScoreRow {
    ante: i32,
    required_score: i32,
    green_stake_required_score: i32,
    purple_stake_required_score: i32,
}

/// Manager for loading and accessing base required scores by ante
#[derive(Debug, Clone)]
pub struct BaseScoreManager {
    scores: HashMap<i32, BaseScoreRow>,
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

impl BaseScoreManager {
    /// Parse scientific notation string to i32, clamping to i32::MAX if too large
    fn parse_scientific_to_i32(s: &str) -> GameResult<i32> {
        // Try parsing as f64 first to handle scientific notation
        let value = s.parse::<f64>()
            .map_err(|e| GameError::InvalidGameState(format!("Failed to parse number: {}", e)))?;
        
        // Clamp to i32 range
        if value > i32::MAX as f64 {
            Ok(i32::MAX)
        } else if value < i32::MIN as f64 {
            Ok(i32::MIN)
        } else {
            Ok(value as i32)
        }
    }
    
    /// Create a new BaseScoreManager by loading the CSV file
    pub fn new() -> GameResult<Self> {
        let csv_path = ".config/base_required_scores_by_ante.csv";
        let content = fs::read_to_string(csv_path)
            .map_err(|e| GameError::InvalidGameState(format!("Failed to read CSV file: {}", e)))?;
        
        let mut scores = HashMap::new();
        let mut lines = content.lines();
        
        // Skip header line
        lines.next();
        
        for line in lines {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 4 {
                let ante = parts[0].parse::<i32>()
                    .map_err(|e| GameError::InvalidGameState(format!("Invalid ante number: {}", e)))?;
                
                // Parse scientific notation and convert to i32, clamping to i32::MAX if too large
                let required_score = Self::parse_scientific_to_i32(parts[1])
                    .map_err(|e| GameError::InvalidGameState(format!("Invalid required_score: {}", e)))?;
                let green_stake_required_score = Self::parse_scientific_to_i32(parts[2])
                    .map_err(|e| GameError::InvalidGameState(format!("Invalid green_stake_required_score: {}", e)))?;
                let purple_stake_required_score = Self::parse_scientific_to_i32(parts[3])
                    .map_err(|e| GameError::InvalidGameState(format!("Invalid purple_stake_required_score: {}", e)))?;
                
                scores.insert(ante, BaseScoreRow {
                    ante,
                    required_score,
                    green_stake_required_score,
                    purple_stake_required_score,
                });
            }
        }
        
        Ok(Self { scores })
    }
    
    /// Get the base required score for a given ante and stake level
    pub fn get_base_score(&self, ante: i32, stake_level: &crate::stakes::StakeLevel) -> GameResult<i32> {
        self.scores.get(&ante)
            .map(|row| {
                match stake_level {
                    // White and Red stakes use the first column (required_score)
                    crate::stakes::StakeLevel::White | crate::stakes::StakeLevel::Red => row.required_score,
                    // Green, Blue, Black stakes use the second column (green_stake_required_score)
                    crate::stakes::StakeLevel::Green | crate::stakes::StakeLevel::Blue | crate::stakes::StakeLevel::Black => row.green_stake_required_score,
                    // Purple, Orange, Gold stakes use the third column (purple_stake_required_score)
                    crate::stakes::StakeLevel::Purple | crate::stakes::StakeLevel::Orange | crate::stakes::StakeLevel::Gold => row.purple_stake_required_score,
                }
            })
            .ok_or_else(|| GameError::InvalidGameState(format!("No base score found for ante {}", ante)))
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
        }
    }

    /// Get a mutable reference to a blind by type
    pub fn get_blind_mut(&mut self, blind_type: BlindType) -> Option<&mut Blind> {
        match blind_type {
            BlindType::Small => Some(&mut self.small),
            BlindType::Big => Some(&mut self.big),
            BlindType::Boss => Some(&mut self.boss),
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

    /// Get the currently active blind
    pub fn get_active_blind(&self) -> Option<&Blind> {
        if self.small.status == BlindStatus::Active {
            Some(&self.small)
        } else if self.big.status == BlindStatus::Active {
            Some(&self.big)
        } else if self.boss.status == BlindStatus::Active {
            Some(&self.boss)
        } else {
            None
        }
    }
}

/// Processor for generating and managing blinds
pub struct BlindProcessor {
    base_score_manager: BaseScoreManager,
}

impl BlindProcessor {
    /// Create a new BlindProcessor
    pub fn new() -> GameResult<Self> {
        let base_score_manager = BaseScoreManager::new()?;
        Ok(Self { base_score_manager })
    }
    
    /// Generate the three blinds for a given ante
    pub fn generate_blinds(&self, ante: u32, stake: &crate::stakes::Stake) -> GameResult<UpcomingBlinds> {
        // Get the base score from CSV data based on stake level
        let base_score = self.base_score_manager.get_base_score(ante as i32, &stake.level)?;
        
        // Calculate required scores based on wiki rules:
        // Small Blind = 1x base chips
        // Big Blind = 1.5x base chips  
        // Boss Blind = 2x base chips
        let small_score = base_score;
        let big_score = (base_score as f32 * 1.5) as i32;
        let boss_score = base_score * 2;
        
        // Calculate base money rewards (these can stay simple for now)
        let base_small_money = (ante * 2) as i32;
        let base_big_money = (ante * 3) as i32;
        let base_boss_money = (ante * 4) as i32;

        // Apply stake modifiers to money rewards only (scores are already correct from CSV)
        let small_money = (base_small_money as f32 * stake.modifiers.money_reward_multiplier) as i32;
        let big_money = (base_big_money as f32 * stake.modifiers.money_reward_multiplier) as i32;
        let boss_money = (base_boss_money as f32 * stake.modifiers.money_reward_multiplier) as i32;

        // Create the blinds
        let small = Blind::new(
            format!("Small Blind, Ante{}", ante),
            BlindType::Small,
            small_score,
            small_money,
        );

        let big = Blind::new(
            format!("Big Blind, Ante{}", ante),
            BlindType::Big,
            big_score,
            big_money,
        );

        let boss = Blind::new_boss(
            format!("Boss Blind, Ante{}", ante),
            boss_score,
            boss_money,
            BossEffect::None, // TODO: Implement specific boss effects based on ante
        );

        Ok(UpcomingBlinds::new(small, big, boss))
    }
}
