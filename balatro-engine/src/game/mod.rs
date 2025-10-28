//! Main game state and logic for Balatro game engine

use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;
use crate::SharedRngManager;
use crate::hand::Hand;
use crate::deck::{Deck, DeckType, SharedDeck};
use crate::joker::JokerInstance;
use crate::planet::Planets;
use crate::stakes::{Stake, StakeLevel};
use crate::blind::{Blind, UpcomingBlinds, BlindType, BossEffect, BlindProcessor};
use crate::consumable::Consumable;
use crate::error::{GameError, GameResult};
use crate::rng::GameRngManager;

/// Current game phase
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Shop,
    ShopPackSelection,
    BlindSelect,
    Playing,
    RoundEnd,
    GameOver,
}

// Removed tuple structs - now using primitive types directly

/// Main game state
#[derive(Debug)]
pub struct GameState {
    pub phase: GamePhase,
    pub ante: u32,
    pub hand_size: usize,
    pub money: i32,
    pub score: i32,
    pub deck: SharedDeck,
    pub stake: Stake,
    pub jokers: Vec<JokerInstance>,
    pub hand: Hand,
    pub upcoming_blinds: UpcomingBlinds,
    pub consumables: Vec<Consumable>,
    pub round_number: u32,
    pub planets: Planets,
}

impl GameState {
    /// Create a new game state
    pub fn new(rng_manager: SharedRngManager) -> Self {
        let stake = Stake::new(StakeLevel::White);
        let processor = BlindProcessor::new().unwrap_or_else(|_| {
            // Fallback processor if CSV loading fails
            panic!("Failed to initialize BlindProcessor - CSV file not found or invalid");
        });
        let upcoming_blinds = processor.generate_blinds(1, &stake).unwrap_or_else(|_| {
            // Fallback blinds if generation fails
            UpcomingBlinds::new(
                Blind::new("Small Blind 1".to_string(), BlindType::Small, 300, 2),
                Blind::new("Big Blind 1".to_string(), BlindType::Big, 450, 3),
                Blind::new_boss("Boss Blind 1".to_string(), 600, 4, BossEffect::None),
            )
        });

        let deck_ref = Deck::new(DeckType::Red, rng_manager.clone());
        deck_ref.borrow_mut().shuffle();

        Self {
            phase: GamePhase::BlindSelect,
            ante: 1,
            hand_size: 8,
            money: 4,
            score: 0,
            deck: deck_ref,
            stake,
            jokers: Vec::new(),
            hand: Hand::new(),
            upcoming_blinds,
            consumables: Vec::new(),
            round_number: 1,
            planets: Planets::new_default().unwrap_or_default(),
        }
    }

    /// Create a new game state with custom settings
    pub fn new_with_settings(deck_type: DeckType, stake_level: StakeLevel, rng_manager: SharedRngManager) -> Self {
        let stake = Stake::new(stake_level);
        let processor = BlindProcessor::new().unwrap_or_else(|_| {
            panic!("Failed to initialize BlindProcessor - CSV file not found or invalid");
        });
        let upcoming_blinds = processor.generate_blinds(1, &stake).unwrap_or_else(|_| {
            UpcomingBlinds::new(
                Blind::new("Small Blind 1".to_string(), BlindType::Small, 300, 2),
                Blind::new("Big Blind 1".to_string(), BlindType::Big, 450, 3),
                Blind::new_boss("Boss Blind 1".to_string(), 600, 4, BossEffect::None),
            )
        });

        let deck_ref = Deck::new(deck_type, rng_manager.clone());
        deck_ref.borrow_mut().shuffle();

        Self {
            phase: GamePhase::BlindSelect,
            ante: 1,
            hand_size: 8,
            money: 4,
            score: 0,
            deck: deck_ref,
            stake,
            jokers: Vec::new(),
            hand: Hand::new(),
            upcoming_blinds,
            consumables: Vec::new(),
            round_number: 1,
            planets: Planets::new_default().unwrap_or_default(),
        }
    }

    /// Draw cards to fill the hand after clearing it out first
    pub fn clear_and_draw_hand(&mut self) -> GameResult<()> {
        self.hand.clear();
        self.draw_hand()?;
        self.hand.sort_by_rank_desc();
        Ok(())
    }

    /// Draw cards to fill the hand
    pub fn draw_hand(&mut self) -> GameResult<()> {
        if self.hand.len() >= self.hand_size {
            return Err(GameError::InvalidGameState("Hand is already full".to_string()));
        }
        let cards = self.deck.borrow_mut().draw_multiple(self.hand_size - self.hand.len())?;
        for card in cards {
            self.hand.add_card(card);
        }
        Ok(())
    }

    /// Play a hand of cards
    pub fn play_hand(&mut self) -> GameResult<i32> {
        // Calculate the hand score using the scoring module
        let hand_score = crate::scoring::calculate_hand_score(self)?;
        
        // Calculate final score
        let final_score = hand_score.final_score() as i32;
        self.score = final_score;
        
        // Remove played cards from hand
        self.hand.discard_selected_cards(&mut *self.deck.borrow_mut())?;

        Ok(final_score)
    }

    /// End the current round
    pub fn end_round(&mut self) -> GameResult<()> {
        self.round_number += 1;
        self.phase = GamePhase::RoundEnd;
        Ok(())
    }

    /// Start a new ante
    pub fn start_new_ante(&mut self) -> GameResult<()> {
        self.ante += 1;
        self.phase = GamePhase::BlindSelect;
        Ok(())
    }

    /// Get the current blind being faced (if any)
    pub fn get_current_blind(&self) -> Option<&Blind> {
        // For now, we'll need to track which blind is currently active
        // This could be enhanced with a current_blind_type field in GameState
        Some(&self.upcoming_blinds.small) // Default to small for now
    }

    /// Get a specific blind from upcoming blinds
    pub fn get_blind(&self, blind_type: BlindType) -> Option<&Blind> {
        self.upcoming_blinds.get_blind(blind_type)
    }

    /// Check if the player has beaten a specific blind
    pub fn has_beaten_blind(&self, _blind_type: BlindType) -> bool {
        // This would need to be tracked in GameState - for now return false
        // Could add a beaten_blinds field to track this
        false
    }

    /// Generate new blinds for the current ante
    pub fn generate_blinds(&mut self) -> GameResult<()> {
        let processor = BlindProcessor::new()?;
        let upcoming_blinds = processor.generate_blinds(self.ante, &self.stake)?;
        self.upcoming_blinds = upcoming_blinds;
        Ok(())
    }
}
