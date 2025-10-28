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

/// Hand score tracking
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HandScore {
    pub chip_score: i32,
    pub mult_score: f32,
}

impl HandScore {
    pub fn new() -> Self {
        Self {
            chip_score: 0,
            mult_score: 1.0,
        }
    }

    /// Get the final score by multiplying chips by mult
    pub fn final_score(&self) -> f32 {
        self.chip_score as f32 * self.mult_score
    }
}

impl Default for HandScore {
    fn default() -> Self {
        Self::new()
    }
}

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

        // Initialize hand score
        let mut hand_score = HandScore::new();

        // Phase 1: Pre-scoring
        hand_score = self.apply_pre_scoring(hand_score, &selected_cards)?;

        // Phase 2: Played Hand Scoring
        hand_score = self.apply_played_hand_scoring(hand_score, &selected_cards)?;

        // Phase 3: Effects in Hand
        hand_score = self.apply_effects_in_hand(hand_score, &selected_cards)?;

        // Phase 4: Joker Scoring
        hand_score = self.apply_joker_scoring(hand_score, &selected_cards)?;

        // Calculate final score
        let final_score = hand_score.final_score() as i32;
        self.score = final_score;
        
        // Remove played cards from hand
        self.hand.discard_selected_cards(&mut self.deck)?;

        Ok(final_score)
    }

    /// Phase 1: Pre-scoring
    /// Apply pre-scoring effects before the hand is evaluated
    fn apply_pre_scoring(&self, hand_score: HandScore, _cards: &[Card]) -> GameResult<HandScore> {
        // TODO: Implement pre-scoring effects
        // This includes effects that modify the hand before evaluation
        // Examples: cards that change suit/rank, effects that modify hand composition
        Ok(hand_score)
    }

    /// Phase 2: Played Hand Scoring
    /// Evaluate the poker hand and calculate base score
    fn apply_played_hand_scoring(&self, mut hand_score: HandScore, cards: &[Card]) -> GameResult<HandScore> {
        // TODO: Implement poker hand detection and scoring
        // This should detect hand types (pair, flush, etc.) and apply base scoring
        // For now, just sum up the chip values
        for card in cards {
            hand_score.chip_score += card.chip_value();
        }
        Ok(hand_score)
    }

    /// Phase 3: Effects in Hand
    /// Apply effects from cards in the hand
    fn apply_effects_in_hand(&self, hand_score: HandScore, _cards: &[Card]) -> GameResult<HandScore> {
        // TODO: Implement card effects
        // This includes effects from card enhancements, editions, seals
        // Cards can modify chips, mult, or trigger special effects
        Ok(hand_score)
    }

    /// Phase 4: Joker Scoring
    /// Apply effects from active jokers
    fn apply_joker_scoring(&self, mut hand_score: HandScore, cards: &[Card]) -> GameResult<HandScore> {
        // Convert &[Card] to Vec<&Card> for joker compatibility
        let card_refs: Vec<&Card> = cards.iter().collect();
        
        for joker in &self.jokers {
            // TODO: Implement full joker effect application
            // For now, apply basic joker effects
            let (chip_mod, mult_mod) = joker.apply_effects(&card_refs)?;
            hand_score.chip_score += chip_mod;
            hand_score.mult_score *= mult_mod;
        }
        
        Ok(hand_score)
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
