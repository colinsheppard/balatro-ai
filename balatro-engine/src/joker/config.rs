//! Data structures for TOML-based joker configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::card::{Suit, Rank};
use crate::error::{GameError, GameResult};

/// Main configuration structure for joker definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JokerConfig {
    pub schema_version: String,
    pub jokers: Vec<JokerDefinition>,
}

/// Individual joker definition from TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JokerDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rarity: JokerRarity,
    pub cost: i32,
    pub effect: JokerEffect,
    #[serde(default)]
    pub state: Option<JokerState>,
    #[serde(default)]
    pub behavior: Option<JokerBehavior>,
}

/// Joker rarity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum JokerRarity {
    #[serde(rename = "common")]
    Common,
    #[serde(rename = "uncommon")]
    Uncommon,
    #[serde(rename = "rare")]
    Rare,
    #[serde(rename = "legendary")]
    Legendary,
}

/// Joker effect configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JokerEffect {
    #[serde(rename = "type")]
    pub effect_type: JokerEffectType,
    #[serde(default)]
    pub per_card: bool,
    #[serde(default)]
    pub condition: Option<JokerCondition>,
    #[serde(default)]
    pub action: Option<JokerAction>,
    #[serde(default)]
    pub base_effect: Option<JokerAction>,
    #[serde(default)]
    pub state_modifiers: Vec<StateModifier>,
    #[serde(default)]
    pub special_type: Option<String>,
    #[serde(default)]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
    // Direct scoring fields
    #[serde(default)]
    pub mult: Option<i32>,
    #[serde(default)]
    pub chips: Option<i32>,
    #[serde(default)]
    pub mult_multiplier: Option<f32>,
    #[serde(default)]
    pub chips_multiplier: Option<f32>,
    // Calculation fields
    #[serde(default)]
    pub formula: Option<String>,
    #[serde(default)]
    pub result_type: Option<String>,
}

/// Types of joker effects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JokerEffectType {
    #[serde(rename = "scoring")]
    Scoring,
    #[serde(rename = "conditional")]
    Conditional,
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "calculate")]
    Calculate,
    #[serde(rename = "special")]
    Special,
}

/// Conditions for conditional effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JokerCondition {
    #[serde(rename = "type")]
    pub condition_type: ConditionType,
    #[serde(default)]
    pub hand_type: Option<String>,
    #[serde(default)]
    pub suit: Option<SuitWrapper>,
    #[serde(default)]
    pub rank: Option<RankWrapper>,
    #[serde(default)]
    pub field: Option<String>,
    #[serde(default)]
    pub operator: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub size: Option<usize>,
    #[serde(default)]
    pub conditions: Option<Vec<JokerCondition>>,
}

/// Types of conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    #[serde(rename = "hand_type")]
    HandType,
    #[serde(rename = "suit_scored")]
    SuitScored,
    #[serde(rename = "rank_scored")]
    RankScored,
    #[serde(rename = "face_card_scored")]
    FaceCardScored,
    #[serde(rename = "no_face_cards")]
    NoFaceCards,
    #[serde(rename = "state_value")]
    StateValue,
    #[serde(rename = "hand_size")]
    HandSize,
    #[serde(rename = "any")]
    Any,
}

/// Actions that jokers can perform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JokerAction {
    #[serde(rename = "type")]
    pub action_type: ActionType,
    #[serde(default)]
    pub mult: Option<i32>,
    #[serde(default)]
    pub chips: Option<i32>,
    #[serde(default)]
    pub mult_multiplier: Option<f32>,
    #[serde(default)]
    pub chips_multiplier: Option<f32>,
    #[serde(default)]
    pub count: Option<i32>,
    #[serde(default)]
    pub formula: Option<String>,
    #[serde(default)]
    pub result_type: Option<String>,
}

/// Types of actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename = "add_score")]
    AddScore,
    #[serde(rename = "multiply_score")]
    MultiplyScore,
    #[serde(rename = "retrigger")]
    Retrigger,
    #[serde(rename = "calculate")]
    Calculate,
    #[serde(rename = "special")]
    Special,
}

/// State modifiers for dynamic effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateModifier {
    pub state_field: String,
    pub multiplier: f32,
}

/// Joker state configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JokerState {
    #[serde(default)]
    pub persistent: bool,
    pub fields: HashMap<String, serde_json::Value>,
}

/// Joker behavior configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JokerBehavior {
    #[serde(default)]
    pub on_hand_played: Option<BehaviorAction>,
    #[serde(default)]
    pub on_round_end: Option<BehaviorAction>,
    #[serde(default)]
    pub on_discard: Option<BehaviorAction>,
    #[serde(default)]
    pub on_shop_open: Option<BehaviorAction>,
}

/// Behavior actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAction {
    #[serde(rename = "type")]
    pub action_type: String,
    #[serde(default)]
    pub field: Option<String>,
    #[serde(default)]
    pub operation: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub special_type: Option<String>,
    #[serde(default)]
    pub actions: Option<Vec<BehaviorAction>>,
}

/// Runtime joker instance with state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JokerInstance {
    pub definition: JokerDefinition,
    pub state: HashMap<String, serde_json::Value>,
    pub edition: JokerEdition,
    pub stickers: Vec<JokerSticker>,
}

/// Joker editions (unchanged from original)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JokerEdition {
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

/// Joker stickers (unchanged from original)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JokerSticker {
    Eternal,
    Perishable,
    Rental,
}

impl JokerInstance {
    /// Create a new joker instance from a definition
    pub fn new(definition: JokerDefinition) -> Self {
        let mut state = HashMap::new();
        
        // Initialize state fields if defined
        if let Some(state_config) = &definition.state {
            for (key, value) in &state_config.fields {
                state.insert(key.clone(), value.clone());
            }
        }
        
        Self {
            definition,
            state,
            edition: JokerEdition::Base,
            stickers: Vec::new(),
        }
    }

    /// Get the sell value based on rarity
    pub fn sell_value(&self) -> i32 {
        match self.definition.rarity {
            JokerRarity::Common => 3,
            JokerRarity::Uncommon => 4,
            JokerRarity::Rare => 5,
            JokerRarity::Legendary => 6,
        }
    }

    /// Check if this joker has a specific sticker
    pub fn has_sticker(&self, sticker: JokerSticker) -> bool {
        self.stickers.contains(&sticker)
    }

    /// Apply this joker's effects to a hand
    pub fn apply_effects(&self, cards: &Vec<&crate::card::Card>) -> GameResult<(i32, f32)> {
        // This will be implemented with the full effect system
        // For now, return base values
        Ok((0, 1.0))
    }
}

impl std::fmt::Display for JokerInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.definition.name)
    }
}

/// Helper function to convert string rarity to enum
impl std::str::FromStr for JokerRarity {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "common" => Ok(JokerRarity::Common),
            "uncommon" => Ok(JokerRarity::Uncommon),
            "rare" => Ok(JokerRarity::Rare),
            "legendary" => Ok(JokerRarity::Legendary),
            _ => Err(GameError::InvalidJokerOperation(format!("Unknown rarity: {}", s))),
        }
    }
}

/// Suit with custom deserialization for lowercase strings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "String")]
pub struct SuitWrapper(Suit);

impl From<String> for SuitWrapper {
    fn from(s: String) -> Self {
        let suit = match s.to_lowercase().as_str() {
            "hearts" => Suit::Hearts,
            "diamonds" => Suit::Diamonds,
            "clubs" => Suit::Clubs,
            "spades" => Suit::Spades,
            _ => Suit::Hearts, // Default fallback
        };
        SuitWrapper(suit)
    }
}

impl From<SuitWrapper> for Suit {
    fn from(wrapper: SuitWrapper) -> Self {
        wrapper.0
    }
}

/// Rank with custom deserialization for lowercase strings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "String")]
pub struct RankWrapper(Rank);

impl From<String> for RankWrapper {
    fn from(s: String) -> Self {
        let rank = match s.to_lowercase().as_str() {
            "ace" => Rank::Ace,
            "two" => Rank::Two,
            "three" => Rank::Three,
            "four" => Rank::Four,
            "five" => Rank::Five,
            "six" => Rank::Six,
            "seven" => Rank::Seven,
            "eight" => Rank::Eight,
            "nine" => Rank::Nine,
            "ten" => Rank::Ten,
            "jack" => Rank::Jack,
            "queen" => Rank::Queen,
            "king" => Rank::King,
            _ => Rank::Ace, // Default fallback
        };
        RankWrapper(rank)
    }
}

impl From<RankWrapper> for Rank {
    fn from(wrapper: RankWrapper) -> Self {
        wrapper.0
    }
}
