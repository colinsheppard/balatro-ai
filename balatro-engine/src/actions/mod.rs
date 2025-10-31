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
    NextRound,
}

impl Action for ShopAction {
    fn index(&self) -> u32 {
        match self {
            ShopAction::NextRound => 1,
        }
    }
    
    fn description(&self) -> &str {
        match self {
            ShopAction::NextRound => "Next Round",
        }
    }
}

impl fmt::Display for ShopAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
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
    PlaySelectedCards,
    DiscardSelectedCards,
    SelectCard(usize),      // Index in hand
    DeselectCard(usize),    // Index in hand
    MoveRight(usize),       // Index in hand
    MoveLeft(usize),        // Index in hand
    SortByRank,
    SortBySuit,
}

impl Action for PlayingAction {
    fn index(&self) -> u32 {
        match self {
            PlayingAction::PlaySelectedCards => 0,
            PlayingAction::DiscardSelectedCards => 1,
            PlayingAction::SelectCard(_) => 2,      // Will be overridden in display
            PlayingAction::DeselectCard(_) => 3,   // Will be overridden in display
            PlayingAction::MoveRight(_) => 4,      // Will be overridden in display
            PlayingAction::MoveLeft(_) => 5,       // Will be overridden in display
            PlayingAction::SortByRank => 6,
            PlayingAction::SortBySuit => 7,
        }
    }
    
    fn description(&self) -> &str {
        match self {
            PlayingAction::PlaySelectedCards => "Play selected cards",
            PlayingAction::DiscardSelectedCards => "Discard selected cards",
            PlayingAction::SelectCard(_) => "Select card",
            PlayingAction::DeselectCard(_) => "Deselect card",
            PlayingAction::MoveRight(_) => "Move right",
            PlayingAction::MoveLeft(_) => "Move left",
            PlayingAction::SortByRank => "Sort by rank",
            PlayingAction::SortBySuit => "Sort by suit",
        }
    }
}

impl PlayingAction {
    /// Get the actual index for display (calculated dynamically)
    pub fn display_index(&self, start_index: u32) -> u32 {
        match self {
            PlayingAction::PlaySelectedCards => start_index,
            PlayingAction::DiscardSelectedCards => start_index + 1,
            _ => start_index + 2 + self.card_index() as u32,
        }
    }
    
    /// Get the card index for actions that involve a card
    fn card_index(&self) -> usize {
        match self {
            PlayingAction::PlaySelectedCards | PlayingAction::DiscardSelectedCards |
            PlayingAction::SortByRank | PlayingAction::SortBySuit => 0,
            PlayingAction::SelectCard(idx) | 
            PlayingAction::DeselectCard(idx) | 
            PlayingAction::MoveRight(idx) | 
            PlayingAction::MoveLeft(idx) => *idx,
        }
    }
}

impl fmt::Display for PlayingAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayingAction::PlaySelectedCards => {
                write!(f, "Play selected cards")
            }
            PlayingAction::DiscardSelectedCards => {
                write!(f, "Discard selected cards")
            }
            PlayingAction::SortByRank => {
                write!(f, "Sort by rank")
            }
            PlayingAction::SortBySuit => {
                write!(f, "Sort by suit")
            }
            _ => {
                // For display, this will be completed by the caller
                // when they have access to the card
                write!(f, "{}", self.description())
            }
        }
    }
}

/// Actions available in the RoundEnd phase
#[derive(Debug, Clone)]
pub enum RoundEndAction {
    CashOut,
}

impl Action for RoundEndAction {
    fn index(&self) -> u32 {
        match self {
            RoundEndAction::CashOut => 1,
        }
    }
    
    fn description(&self) -> &str {
        match self {
            RoundEndAction::CashOut => "Cash Out",
        }
    }
}

impl fmt::Display for RoundEndAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
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
    pub fn create_shop_actions() -> Vec<(u32, ShopAction)> {
        vec![
            (0, ShopAction::NextRound),
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
    
    /// Create all playing actions based on current hand state
    pub fn create_playing_actions(game_state: &crate::GameState) -> Vec<(u32, PlayingAction)> {
        use std::collections::HashSet;
        let binding = game_state.hand.borrow();
        let cards = binding.cards();
        let selected_indices = binding.selected_indices();
        let selected_set: HashSet<usize> = selected_indices.iter().copied().collect();

        let mut actions = Vec::new();
        let mut action_index = 0u32;
        
        // First action is always play
        actions.push((action_index, PlayingAction::PlaySelectedCards));
        action_index += 1;
        
        // Only add discard action if discards remain
        if game_state.play_limits.borrow().has_discards_remaining() {
            actions.push((action_index, PlayingAction::DiscardSelectedCards));
            action_index += 1;
        }
        
        // Generate actions for each card in order
        for (i, _card) in cards.iter().enumerate() {
            let is_selected = selected_set.contains(&i);
            
            if !is_selected {
                // Not selected: offer to select
                actions.push((action_index, PlayingAction::SelectCard(i)));
                action_index += 1;
            } else {
                // Selected: offer to deselect
                actions.push((action_index, PlayingAction::DeselectCard(i)));
                action_index += 1;
            }
        }
            
        for (i, _card) in cards.iter().enumerate() {
            // Move right (if not last card)
            if i < cards.len() - 1 {
                actions.push((action_index, PlayingAction::MoveRight(i)));
                action_index += 1;
            }
        }
        for (i, _card) in cards.iter().enumerate() {
            // Move left (if not first card)
            if i > 0 {
                actions.push((action_index, PlayingAction::MoveLeft(i)));
                action_index += 1;
            }
        }
        
        // Add sort actions
        actions.push((action_index, PlayingAction::SortByRank));
        action_index += 1;
        
        actions.push((action_index, PlayingAction::SortBySuit));
        action_index += 1;
        
        actions
    }
    
    /// Format a playing action with its card for display
    pub fn format_playing_action(action: &PlayingAction, card: &crate::card::Card, action_num: u32) -> String {
        match action {
            PlayingAction::SelectCard(_) => {
                format!("{}: Select {}", action_num, card)
            }
            PlayingAction::DeselectCard(_) => {
                format!("{}: Deselect {}", action_num, card)
            }
            PlayingAction::MoveRight(_) => {
                format!("{}: Move {} right", action_num, card)
            }
            PlayingAction::MoveLeft(_) => {
                format!("{}: Move {} left", action_num, card)
            }
            _ => format!("{}: {}", action_num, action.description())
        }
    }
    
    /// Create all round end actions
    pub fn create_round_end_actions() -> Vec<(u32, RoundEndAction)> {
        vec![
            (0, RoundEndAction::CashOut),
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
