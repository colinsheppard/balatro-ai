//! Stakes system for Balatro game engine

use serde::{Deserialize, Serialize};
use std::fmt;

/// Different stake levels that modify game difficulty
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StakeLevel {
    White,
    Red,
    Green,
    Blue,
    Black,
    Purple,
    Orange,
    Gold,
}

/// Modifiers that stakes can apply to the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeModifiers {
    /// Multiplier for blind scores (1.0 = no change)
    pub blind_score_multiplier: f32,
    /// Multiplier for blind money rewards (1.0 = no change)
    pub money_reward_multiplier: f32,
    /// Additional money cost for skipping blinds
    pub skip_cost_bonus: i32,
    /// Multiplier for joker costs (1.0 = no change)
    pub joker_cost_multiplier: f32,
    /// Multiplier for consumable costs (1.0 = no change)
    pub consumable_cost_multiplier: f32,
    /// Additional hands per round
    pub hands_per_round_bonus: i32,
    /// Additional discards per round
    pub discards_per_round_bonus: i32,
    /// Starting money modifier
    pub starting_money_modifier: i32,
    /// Starting hand size modifier
    pub starting_hand_size_modifier: i32,
}

/// A stake configuration that defines game difficulty modifiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stake {
    pub level: StakeLevel,
    pub name: String,
    pub description: String,
    pub modifiers: StakeModifiers,
}

impl Stake {
    /// Create a new stake with default modifiers
    pub fn new(level: StakeLevel) -> Self {
        Self {
            level,
            name: level.to_string(),
            description: level.to_string(),
            modifiers: StakeModifiers::default(),
        }
    }
    pub fn new_verbose(level: StakeLevel, name: String, description: String) -> Self {
        Self {
            level,
            name,
            description,
            modifiers: StakeModifiers::default(),
        }
    }

    /// Create a stake with custom modifiers
    pub fn with_modifiers(
        level: StakeLevel,
        name: String,
        description: String,
        modifiers: StakeModifiers,
    ) -> Self {
        Self {
            level,
            name,
            description,
            modifiers,
        }
    }

    /// Get the display color for this stake
    pub fn color(&self) -> &'static str {
        match self.level {
            StakeLevel::White => "white",
            StakeLevel::Red => "red",
            StakeLevel::Green => "green",
            StakeLevel::Blue => "blue",
            StakeLevel::Black => "black",
            StakeLevel::Purple => "purple",
            StakeLevel::Orange => "orange",
            StakeLevel::Gold => "gold",
        }
    }

    /// Get the difficulty multiplier for this stake
    pub fn difficulty_multiplier(&self) -> f32 {
        match self.level {
            StakeLevel::White => 1.0,
            StakeLevel::Red => 1.1,
            StakeLevel::Green => 1.2,
            StakeLevel::Blue => 1.3,
            StakeLevel::Black => 1.4,
            StakeLevel::Purple => 1.5,
            StakeLevel::Orange => 1.6,
            StakeLevel::Gold => 1.7,
        }
    }
}

impl fmt::Display for StakeLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StakeLevel::White => write!(f, "White Stake"),
            StakeLevel::Red => write!(f, "Red Stake"),
            StakeLevel::Green => write!(f, "Green Stake"),
            StakeLevel::Blue => write!(f, "Blue Stake"),
            StakeLevel::Black => write!(f, "Black Stake"),
            StakeLevel::Purple => write!(f, "Purple Stake"),
            StakeLevel::Orange => write!(f, "Orange Stake"),
            StakeLevel::Gold => write!(f, "Gold Stake"),
        }
    }
}

impl Default for StakeModifiers {
    fn default() -> Self {
        Self {
            blind_score_multiplier: 1.0,
            money_reward_multiplier: 1.0,
            skip_cost_bonus: 0,
            joker_cost_multiplier: 1.0,
            consumable_cost_multiplier: 1.0,
            hands_per_round_bonus: 0,
            discards_per_round_bonus: 0,
            starting_money_modifier: 0,
            starting_hand_size_modifier: 0,
        }
    }
}

/// Manager for stakes configuration
pub struct StakeManager {
    stakes: Vec<Stake>,
}

impl StakeManager {
    /// Create a new stake manager with default stakes
    pub fn new() -> Self {
        Self {
            stakes: Self::create_default_stakes(),
        }
    }

    /// Get all stakes
    pub fn all_stakes(&self) -> &[Stake] {
        &self.stakes
    }

    /// Get a stake by level
    pub fn get_stake(&self, level: StakeLevel) -> Option<&Stake> {
        self.stakes.iter().find(|s| s.level == level)
    }

    /// Create default stakes configuration
    fn create_default_stakes() -> Vec<Stake> {
        vec![
            // White Stake - Base difficulty
            Stake::new_verbose(
                StakeLevel::White,
                "White".to_string(),
                "Base difficulty with no modifiers".to_string(),
            ),
            
            // Red Stake - Slightly harder blinds
            Stake::with_modifiers(
                StakeLevel::Red,
                "Red".to_string(),
                "Blind scores increased by 10%".to_string(),
                StakeModifiers {
                    blind_score_multiplier: 1.1,
                    ..Default::default()
                },
            ),
            
            // Green Stake - More expensive skips
            Stake::with_modifiers(
                StakeLevel::Green,
                "Green".to_string(),
                "Skip costs increased by $2".to_string(),
                StakeModifiers {
                    skip_cost_bonus: 2,
                    ..Default::default()
                },
            ),
            
            // Blue Stake - Harder blinds and more expensive skips
            Stake::with_modifiers(
                StakeLevel::Blue,
                "Blue".to_string(),
                "Blind scores +20%, skip costs +$3".to_string(),
                StakeModifiers {
                    blind_score_multiplier: 1.2,
                    skip_cost_bonus: 3,
                    ..Default::default()
                },
            ),
            
            // Black Stake - Reduced money rewards
            Stake::with_modifiers(
                StakeLevel::Black,
                "Black".to_string(),
                "Money rewards reduced by 25%".to_string(),
                StakeModifiers {
                    money_reward_multiplier: 0.75,
                    ..Default::default()
                },
            ),
            
            // Purple Stake - More expensive jokers
            Stake::with_modifiers(
                StakeLevel::Purple,
                "Purple".to_string(),
                "Joker costs increased by 50%".to_string(),
                StakeModifiers {
                    joker_cost_multiplier: 1.5,
                    ..Default::default()
                },
            ),
            
            // Orange Stake - Fewer hands per round
            Stake::with_modifiers(
                StakeLevel::Orange,
                "Orange".to_string(),
                "One less hand per round".to_string(),
                StakeModifiers {
                    hands_per_round_bonus: -1,
                    ..Default::default()
                },
            ),
            
            // Gold Stake - Ultimate challenge
            Stake::with_modifiers(
                StakeLevel::Gold,
                "Gold".to_string(),
                "All previous modifiers combined".to_string(),
                StakeModifiers {
                    blind_score_multiplier: 1.3,
                    money_reward_multiplier: 0.75,
                    skip_cost_bonus: 5,
                    joker_cost_multiplier: 1.5,
                    consumable_cost_multiplier: 1.25,
                    hands_per_round_bonus: -1,
                    discards_per_round_bonus: -1,
                    starting_money_modifier: -2,
                    starting_hand_size_modifier: -1,
                },
            ),
        ]
    }
}

impl Default for StakeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for stake operations
impl StakeModifiers {
    /// Apply modifiers to a blind score
    pub fn apply_to_score(&self, base_score: i32) -> i32 {
        (base_score as f32 * self.blind_score_multiplier) as i32
    }

    /// Apply modifiers to money reward
    pub fn apply_to_money(&self, base_money: i32) -> i32 {
        (base_money as f32 * self.money_reward_multiplier) as i32
    }

    /// Apply modifiers to joker cost
    pub fn apply_to_joker_cost(&self, base_cost: i32) -> i32 {
        (base_cost as f32 * self.joker_cost_multiplier) as i32
    }

    /// Apply modifiers to consumable cost
    pub fn apply_to_consumable_cost(&self, base_cost: i32) -> i32 {
        (base_cost as f32 * self.consumable_cost_multiplier) as i32
    }

    /// Get skip cost with bonus
    pub fn get_skip_cost(&self, base_skip_cost: i32) -> i32 {
        base_skip_cost + self.skip_cost_bonus
    }

    /// Get hands per round with bonus
    pub fn get_hands_per_round(&self, base_hands: i32) -> i32 {
        (base_hands + self.hands_per_round_bonus).max(1) // Minimum 1 hand
    }

    /// Get discards per round with bonus
    pub fn get_discards_per_round(&self, base_discards: i32) -> i32 {
        (base_discards + self.discards_per_round_bonus).max(0) // Minimum 0 discards
    }

    /// Get starting money with modifier
    pub fn get_starting_money(&self, base_money: i32) -> i32 {
        (base_money + self.starting_money_modifier).max(0) // Minimum 0 money
    }

    /// Get starting hand size with modifier
    pub fn get_starting_hand_size(&self, base_hand_size: i32) -> i32 {
        (base_hand_size + self.starting_hand_size_modifier).max(1) // Minimum 1 card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stake_creation() {
        let stake = Stake::new(StakeLevel::Red);
        
        assert_eq!(stake.level, StakeLevel::Red);
        assert_eq!(stake.name, "Red");
        assert_eq!(stake.modifiers.blind_score_multiplier, 1.0);
    }

    #[test]
    fn test_stake_modifiers() {
        let modifiers = StakeModifiers {
            blind_score_multiplier: 1.2,
            money_reward_multiplier: 0.8,
            skip_cost_bonus: 3,
            ..Default::default()
        };

        assert_eq!(modifiers.apply_to_score(100), 120);
        assert_eq!(modifiers.apply_to_money(50), 40);
        assert_eq!(modifiers.get_skip_cost(5), 8);
    }

    #[test]
    fn test_stake_manager() {
        let manager = StakeManager::new();
        let stakes = manager.all_stakes();
        
        assert_eq!(stakes.len(), 8);
        assert!(manager.get_stake(StakeLevel::White).is_some());
        assert!(manager.get_stake(StakeLevel::Gold).is_some());
    }

    #[test]
    fn test_stake_colors() {
        let manager = StakeManager::new();
        let white_stake = manager.get_stake(StakeLevel::White).unwrap();
        let red_stake = manager.get_stake(StakeLevel::Red).unwrap();
        
        assert_eq!(white_stake.color(), "white");
        assert_eq!(red_stake.color(), "red");
    }

    #[test]
    fn test_difficulty_multiplier() {
        let manager = StakeManager::new();
        let white_stake = manager.get_stake(StakeLevel::White).unwrap();
        let gold_stake = manager.get_stake(StakeLevel::Gold).unwrap();
        
        assert_eq!(white_stake.difficulty_multiplier(), 1.0);
        assert_eq!(gold_stake.difficulty_multiplier(), 1.7);
    }

    #[test]
    fn test_stake_ordering() {
        assert!(StakeLevel::White < StakeLevel::Red);
        assert!(StakeLevel::Red < StakeLevel::Green);
        assert!(StakeLevel::Gold > StakeLevel::Orange);
    }

    #[test]
    fn test_modifier_edge_cases() {
        let modifiers = StakeModifiers {
            hands_per_round_bonus: -5,
            discards_per_round_bonus: -10,
            starting_money_modifier: -100,
            starting_hand_size_modifier: -10,
            ..Default::default()
        };

        // Should enforce minimums
        assert_eq!(modifiers.get_hands_per_round(3), 1);
        assert_eq!(modifiers.get_discards_per_round(5), 0);
        assert_eq!(modifiers.get_starting_money(10), 0);
        assert_eq!(modifiers.get_starting_hand_size(8), 1);
    }
}
