//! Main game state and logic for Balatro game engine

use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;
use crate::card::{Card, Enhancement, Edition, Seal};
use crate::hand::Hand;
use crate::deck::{Deck, DeckType};
use crate::joker::JokerInstance;
use crate::joker::config::{JokerCondition, ActionType};
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

        // Phase 1: Pre-scoring
        self.apply_pre_scoring(&selected_cards)?;

        // Phase 2: Played Hand Scoring
        let mut hand_score = self.apply_played_hand_scoring(&selected_cards)?;

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
    fn apply_pre_scoring(&mut self, _cards: &[Card]) -> GameResult<()> {
        // Iterate through each joker and apply pre-scoring effects
        // We need to use unsafe here because we're borrowing self mutably
        // while also accessing self.jokers immutably. This is safe because
        // joker pre-scoring effects don't modify the jokers vector.
        let num_jokers = self.jokers.len();
        for idx in 0..num_jokers {
            let joker: *const _ = &self.jokers[idx];
            unsafe {
                (*joker).apply_joker_pre_scoring_effects(self)?;
            }
        }
        Ok(())
    }

    /// Phase 2: Played Hand Scoring
    /// Evaluate each card in the selected hand
    fn apply_played_hand_scoring(&mut self, cards: &[Card]) -> GameResult<HandScore> {
        let mut hand_score = HandScore::new();
        // Iterate through each card in the selected hand
        for card in cards {
            // Count retriggers before scoring the card
            let retrigger_count = self.count_retriggers(card)?;

            // Apply retriggers
            for _ in 0..retrigger_count {
                hand_score = self.score_card(card, hand_score)?;
            }
        }
        Ok(hand_score)
    }

    /// Score a single card, applying A through F
    fn score_card(&mut self, card: &Card, mut hand_score: HandScore) -> GameResult<HandScore> {
        // A - Add Base Card Chips
        hand_score.chip_score += card.chip_value();

        // B - Apply Enhanced Card Effects
        hand_score = self.apply_enhanced_card_effects(card, hand_score)?;

        // C - Trigger Card Editions
        hand_score = self.apply_card_edition(card, hand_score)?;

        // D - Trigger Joker Effects (per_card effects)
        hand_score = self.apply_per_card_joker_effects(card, hand_score)?;

        // E - Retriggers are handled by calling function

        // F - Gold Seal: if card has gold seal, give $3
        if let Some(Seal::Gold) = card.seal {
            self.money += 3;
        }

        Ok(hand_score)
    }

    /// Apply enhanced card effects (Step B)
    fn apply_enhanced_card_effects(&self, card: &Card, mut hand_score: HandScore) -> GameResult<HandScore> {
        if let Some(enhancement) = &card.enhancement {
            match enhancement {
                Enhancement::Bonus => {
                    hand_score.chip_score += 30;
                }
                Enhancement::Mult => {
                    hand_score.mult_score += 4;
                }
                Enhancement::Wild => {
                    // Has no impact on card score
                }
                Enhancement::Glass => {
                    hand_score.mult_score *= 2;
                }
                Enhancement::Steel => {
                    // Has no impact on card score 
                }
                Enhancement::Stone => {
                    hand_score.chip_score += 50;
                }
                Enhancement::Gold => {
                    // Has no impact on card score 
                }
                Enhancement::Lucky => {
                    // TODO: Implement lucky enhancement
                }
            }
        }
        Ok(hand_score)
    }

    /// Apply card edition effects (Step C)
    fn apply_card_edition(&self, card: &Card, mut hand_score: HandScore) -> GameResult<HandScore> {
        match card.edition {
            Edition::Foil => {
                hand_score.chip_score += 50;
            }
            Edition::Holographic => {
                hand_score.mult_score += 10.0;
            }
            Edition::Polychrome => {
                hand_score.mult_score *= 1.5;
            }
            Edition::Negative => {
                // TODO: Implement negative edition effects
            }
            Edition::Base => {
                // No effects for base edition
            }
        }
        Ok(hand_score)
    }

    /// Apply per_card joker effects (Step D)
    fn apply_per_card_joker_effects(&self, card: &Card, mut hand_score: HandScore) -> GameResult<HandScore> {
        let card_refs: Vec<&Card> = vec![card];
        
        for joker in &self.jokers {
            // Check if this joker has a conditional effect that is per_card
            if let Some(JokerCondition { .. }) = &joker.definition.effect.condition {
                if joker.definition.effect.per_card {
                    let (chip_mod, mult_mod) = joker.apply_effects(&card_refs)?;
                    hand_score.chip_score += chip_mod;
                    hand_score.mult_score *= mult_mod;
                }
            }
        }
        
        Ok(hand_score)
    }

    /// Count the number of retriggers for a card
    fn count_retriggers(&self, card: &Card) -> GameResult<usize> {
        let mut retrigger_count = 0;

        // Check if card has red seal
        if let Some(Seal::Red) = card.seal {
            retrigger_count += 1;
        }

        // Check jokers for retrigger effects
        for joker in &self.jokers {
            if let Some(action) = &joker.definition.effect.action {
                if matches!(action.action_type, ActionType::Retrigger) {
                    // TODO: Check if joker condition is met for this card
                    // For now, count all retrigger jokers
                    retrigger_count += 1;
                }
            }
        }

        Ok(retrigger_count)
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
