//! Main game state and logic for Balatro game engine

use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;
use crate::card::Card;
use crate::hand::Hand;
use crate::deck::{Deck, DeckType};
use crate::joker::JokerInstance;
use crate::stakes::{Stake, StakeLevel};
use crate::blind::{Blind, UpcomingBlinds, BlindType, BossEffect, BlindProcessor};
use crate::consumable::Consumable;
use crate::error::{GameError, GameResult};
use crate::rng::GameRngManager;

/// Current game phase
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Shop,
    BlindSelect,
    Playing,
    RoundEnd,
    GameOver,
}

// Removed tuple structs - now using primitive types directly

/// Main game state
#[derive(Debug, Clone)]
pub struct GameState {
    pub phase: GamePhase,
    pub ante: u32,
    pub hand_size: usize,
    pub money: i32,
    pub score: i32,
    pub deck: Deck,
    pub stake: Stake,
    pub jokers: Vec<JokerInstance>,
    pub hand: Hand,
    pub upcoming_blinds: UpcomingBlinds,
    pub consumables: Vec<Consumable>,
    pub round_number: u32,
}

impl GameState {
    /// Create a new game state
    pub fn new(rng_manager: Rc<RefCell<GameRngManager>>) -> Self {
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

        let mut deck = Deck::new(DeckType::Red, rng_manager);
        deck.shuffle();

        Self {
            phase: GamePhase::BlindSelect,
            ante: 1,
            hand_size: 8,
            money: 4,
            score: 0,
            deck,
            stake,
            jokers: Vec::new(),
            hand: Hand::new(),
            upcoming_blinds,
            consumables: Vec::new(),
            round_number: 1,
        }
    }

    /// Create a new game state with custom settings
    pub fn new_with_settings(deck_type: DeckType, stake_level: StakeLevel, rng_manager: Rc<RefCell<GameRngManager>>) -> Self {
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

        let mut deck = Deck::new(deck_type, rng_manager);
        deck.shuffle();

        Self {
            phase: GamePhase::BlindSelect,
            ante: 1,
            hand_size: 8,
            money: 4,
            score: 0,
            deck,
            stake,
            jokers: Vec::new(),
            hand: Hand::new(),
            upcoming_blinds,
            consumables: Vec::new(),
            round_number: 1,
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
        let cards = self.deck.draw_multiple(self.hand_size - self.hand.len())?;
        for card in cards {
            self.hand.add_card(card);
        }
        Ok(())
    }

    /// Play a hand of cards
    pub fn play_hand(&mut self) -> GameResult<i32> {
        if self.hand.selected_indices().is_empty() {
            return Err(GameError::InvalidGameState("Cannot play empty hand".to_string()));
        }

        // Get the selected cards before removing them
        let selected_cards: Vec<Card> = self.hand.selected_cards().into_iter().cloned().collect();

        let hand_score = self.calculate_hand_score(&selected_cards)?;
        
        // Apply joker effects
        let final_score = self.apply_joker_effects(hand_score, &selected_cards)?;
        
        // Remove played cards from hand
        self.hand.remove_selected_cards()?;

        Ok(final_score)
    }

    /// Calculate the score for a poker hand
    fn calculate_hand_score(&self, cards: &[Card]) -> GameResult<i32> {
        // Basic implementation - will be expanded with full poker hand logic
        let mut score = 0;
        for card in cards {
            score += card.chip_value();
        }
        Ok(score)
    }

    /// Apply joker effects to a score
    fn apply_joker_effects(&self, base_score: i32, cards: &[Card]) -> GameResult<i32> {
        let mut final_score = base_score as f32;
        
        // Convert &[Card] to &Vec<&Card> for joker compatibility
        let card_refs: Vec<&Card> = cards.iter().collect();
        
        for joker in &self.jokers {
            let (chip_mod, mult_mod) = joker.apply_effects(&card_refs)?;
            final_score = (final_score + chip_mod as f32) * mult_mod;
        }
        
        Ok(final_score as i32)
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
