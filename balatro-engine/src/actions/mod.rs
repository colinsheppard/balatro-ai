//! Action system for Balatro game engine
//! 
//! This module defines the trait-based action system that allows for
//! customizable display and processing of user actions across different game phases.

use std::fmt;
use crate::deck::DeckType;
use crate::stakes::StakeLevel;

/// Base trait for all game actions
pub trait Action: fmt::Display {
    /// Get the action index/number
    fn index(&self) -> u32;
    
    /// Get a short description of the action
    fn description(&self) -> &str;
    
    /// Check if this action is valid in the current context
    fn is_valid(&self) -> bool {
        true
    }
}

/// Actions available in the Menu phase
#[derive(Debug, Clone)]
pub enum MenuAction {
    SelectDeck(DeckType),
    SelectStake(StakeLevel),
    StartGame,
    Exit,
}

impl Action for MenuAction {
    fn index(&self) -> u32 {
        match self {
            MenuAction::SelectDeck(_) => 1,
            MenuAction::SelectStake(_) => 2,
            MenuAction::StartGame => 3,
            MenuAction::Exit => 4,
        }
    }
    
    fn description(&self) -> &str {
        match self {
            MenuAction::SelectDeck(_) => "Select Deck Type",
            MenuAction::SelectStake(_) => "Select Stake Level",
            MenuAction::StartGame => "Start Game",
            MenuAction::Exit => "Exit",
        }
    }
}

impl fmt::Display for MenuAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuAction::SelectDeck(deck_type) => {
                write!(f, "{}: Select {} Deck", self.index(), deck_type)
            }
            MenuAction::SelectStake(stake_level) => {
                write!(f, "{}: Select {} Stake", self.index(), stake_level)
            }
            MenuAction::StartGame => {
                write!(f, "{}: Start Game", self.index())
            }
            MenuAction::Exit => {
                write!(f, "{}: Exit", self.index())
            }
        }
    }
}

/// Actions available in the Shop phase
#[derive(Debug, Clone)]
pub enum ShopAction {
    BuyJoker,
    BuyConsumable,
    SellJoker,
    SkipShop,
    ViewDeck,
}

impl Action for ShopAction {
    fn index(&self) -> u32 {
        match self {
            ShopAction::BuyJoker => 1,
            ShopAction::BuyConsumable => 2,
            ShopAction::SellJoker => 3,
            ShopAction::SkipShop => 4,
            ShopAction::ViewDeck => 5,
        }
    }
    
    fn description(&self) -> &str {
        match self {
            ShopAction::BuyJoker => "Buy Joker",
            ShopAction::BuyConsumable => "Buy Consumable",
            ShopAction::SellJoker => "Sell Joker",
            ShopAction::SkipShop => "Skip Shop",
            ShopAction::ViewDeck => "View Deck",
        }
    }
}

impl fmt::Display for ShopAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.index(), self.description())
    }
}

/// Actions available in the BlindSelect phase
#[derive(Debug, Clone)]
pub enum BlindAction {
    SelectBossBlind,
    SelectEliteBlind,
    SelectNormalBlind,
    ViewBlindDetails,
}

impl Action for BlindAction {
    fn index(&self) -> u32 {
        match self {
            BlindAction::SelectBossBlind => 1,
            BlindAction::SelectEliteBlind => 2,
            BlindAction::SelectNormalBlind => 3,
            BlindAction::ViewBlindDetails => 4,
        }
    }
    
    fn description(&self) -> &str {
        match self {
            BlindAction::SelectBossBlind => "Select Boss Blind",
            BlindAction::SelectEliteBlind => "Select Elite Blind",
            BlindAction::SelectNormalBlind => "Select Normal Blind",
            BlindAction::ViewBlindDetails => "View Blind Details",
        }
    }
}

impl fmt::Display for BlindAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.index(), self.description())
    }
}

/// Actions available in the Playing phase
#[derive(Debug, Clone)]
pub enum PlayingAction {
    SelectCard,
    DeselectCard,
    PlayHand,
    DiscardHand,
    ViewJokers,
    UseConsumable,
}

impl Action for PlayingAction {
    fn index(&self) -> u32 {
        match self {
            PlayingAction::SelectCard => 1,
            PlayingAction::DeselectCard => 2,
            PlayingAction::PlayHand => 3,
            PlayingAction::DiscardHand => 4,
            PlayingAction::ViewJokers => 5,
            PlayingAction::UseConsumable => 6,
        }
    }
    
    fn description(&self) -> &str {
        match self {
            PlayingAction::SelectCard => "Select Card",
            PlayingAction::DeselectCard => "Deselect Card",
            PlayingAction::PlayHand => "Play Hand",
            PlayingAction::DiscardHand => "Discard Hand",
            PlayingAction::ViewJokers => "View Jokers",
            PlayingAction::UseConsumable => "Use Consumable",
        }
    }
}

impl fmt::Display for PlayingAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.index(), self.description())
    }
}

/// Actions available in the RoundEnd phase
#[derive(Debug, Clone)]
pub enum RoundEndAction {
    ContinueToShop,
    ViewStatistics,
    SaveGame,
}

impl Action for RoundEndAction {
    fn index(&self) -> u32 {
        match self {
            RoundEndAction::ContinueToShop => 1,
            RoundEndAction::ViewStatistics => 2,
            RoundEndAction::SaveGame => 3,
        }
    }
    
    fn description(&self) -> &str {
        match self {
            RoundEndAction::ContinueToShop => "Continue to Shop",
            RoundEndAction::ViewStatistics => "View Statistics",
            RoundEndAction::SaveGame => "Save Game",
        }
    }
}

impl fmt::Display for RoundEndAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.index(), self.description())
    }
}

/// Actions available in the GameOver phase
#[derive(Debug, Clone)]
pub enum GameOverAction {
    PlayAgain,
    MainMenu,
    ViewFinalStatistics,
    Exit,
}

impl Action for GameOverAction {
    fn index(&self) -> u32 {
        match self {
            GameOverAction::PlayAgain => 1,
            GameOverAction::MainMenu => 2,
            GameOverAction::ViewFinalStatistics => 3,
            GameOverAction::Exit => 4,
        }
    }
    
    fn description(&self) -> &str {
        match self {
            GameOverAction::PlayAgain => "Play Again",
            GameOverAction::MainMenu => "Main Menu",
            GameOverAction::ViewFinalStatistics => "View Final Statistics",
            GameOverAction::Exit => "Exit",
        }
    }
}

impl fmt::Display for GameOverAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.index(), self.description())
    }
}

/// Helper functions for creating action lists
pub mod helpers {
    use super::*;
    
    /// Get all available deck types
    pub fn all_deck_types() -> Vec<DeckType> {
        vec![
            DeckType::Red,
            DeckType::Blue,
            DeckType::Yellow,
            DeckType::Green,
            DeckType::Black,
            DeckType::Magic,
            DeckType::Nebula,
            DeckType::Ghost,
            DeckType::Abandoned,
            DeckType::Checkered,
            DeckType::Zodiac,
            DeckType::Painted,
            DeckType::Anaglyph,
            DeckType::Plasma,
            DeckType::Erratic,
        ]
    }
    
    /// Get all available stake levels
    pub fn all_stake_levels() -> Vec<StakeLevel> {
        vec![
            StakeLevel::White,
            StakeLevel::Red,
            StakeLevel::Green,
            StakeLevel::Blue,
            StakeLevel::Black,
            StakeLevel::Purple,
            StakeLevel::Orange,
            StakeLevel::Gold,
        ]
    }
    
    /// Create menu actions for deck selection
    pub fn create_deck_selection_actions() -> Vec<MenuAction> {
        all_deck_types()
            .into_iter()
            .map(MenuAction::SelectDeck)
            .collect()
    }
    
    /// Create menu actions for stake selection
    pub fn create_stake_selection_actions() -> Vec<MenuAction> {
        all_stake_levels()
            .into_iter()
            .map(MenuAction::SelectStake)
            .collect()
    }
    
    /// Create all menu actions
    pub fn create_menu_actions() -> Vec<MenuAction> {
        let mut actions = Vec::new();
        actions.extend(create_deck_selection_actions());
        actions.extend(create_stake_selection_actions());
        actions.push(MenuAction::StartGame);
        actions.push(MenuAction::Exit);
        actions
    }
    
    /// Create all shop actions
    pub fn create_shop_actions() -> Vec<ShopAction> {
        vec![
            ShopAction::BuyJoker,
            ShopAction::BuyConsumable,
            ShopAction::SellJoker,
            ShopAction::SkipShop,
            ShopAction::ViewDeck,
        ]
    }
    
    /// Create all blind select actions
    pub fn create_blind_actions() -> Vec<BlindAction> {
        vec![
            BlindAction::SelectBossBlind,
            BlindAction::SelectEliteBlind,
            BlindAction::SelectNormalBlind,
            BlindAction::ViewBlindDetails,
        ]
    }
    
    /// Create all playing actions
    pub fn create_playing_actions() -> Vec<PlayingAction> {
        vec![
            PlayingAction::SelectCard,
            PlayingAction::DeselectCard,
            PlayingAction::PlayHand,
            PlayingAction::DiscardHand,
            PlayingAction::ViewJokers,
            PlayingAction::UseConsumable,
        ]
    }
    
    /// Create all round end actions
    pub fn create_round_end_actions() -> Vec<RoundEndAction> {
        vec![
            RoundEndAction::ContinueToShop,
            RoundEndAction::ViewStatistics,
            RoundEndAction::SaveGame,
        ]
    }
    
    /// Create all game over actions
    pub fn create_game_over_actions() -> Vec<GameOverAction> {
        vec![
            GameOverAction::PlayAgain,
            GameOverAction::MainMenu,
            GameOverAction::ViewFinalStatistics,
            GameOverAction::Exit,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_menu_action_display() {
        let action = MenuAction::SelectDeck(DeckType::Red);
        assert_eq!(action.index(), 1);
        assert_eq!(action.description(), "Select Deck Type");
        assert!(action.is_valid());
    }
    
    #[test]
    fn test_action_helpers() {
        let deck_types = helpers::all_deck_types();
        assert_eq!(deck_types.len(), 15);
        
        let stake_levels = helpers::all_stake_levels();
        assert_eq!(stake_levels.len(), 8);
        
        let menu_actions = helpers::create_menu_actions();
        assert!(menu_actions.len() > 10); // Deck types + stake levels + start + exit
    }
}
