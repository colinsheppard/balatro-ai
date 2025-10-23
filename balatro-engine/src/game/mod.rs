//! Main game state and logic for Balatro game engine

use serde::{Deserialize, Serialize};
use crate::card::Card;
use crate::deck::{Deck, DeckType};
use crate::joker::JokerInstance;
use crate::blind::Blind;
use crate::consumable::Consumable;
use crate::error::{GameError, GameResult};

/// Current game phase
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Menu,
    Shop,
    BlindSelect,
    Playing,
    RoundEnd,
    GameOver,
}

/// Current ante level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Ante(pub u32);

/// Current hand size
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HandSize(pub usize);

/// Player's money
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money(pub i32);

/// Player's score
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Score(pub i32);

/// Main game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub phase: GamePhase,
    pub ante: Ante,
    pub hand_size: HandSize,
    pub money: Money,
    pub score: Score,
    pub deck: Deck,
    pub jokers: Vec<JokerInstance>,
    pub hand: Vec<Card>,
    pub current_blind: Option<Blind>,
    pub consumables: Vec<Consumable>,
    pub round_number: u32,
}

impl GameState {
    /// Create a new game state
    pub fn new() -> Self {
        Self {
            phase: GamePhase::Menu,
            ante: Ante(1),
            hand_size: HandSize(8),
            money: Money(4),
            score: Score(0),
            deck: Deck::new(DeckType::Red),
            jokers: Vec::new(),
            hand: Vec::new(),
            current_blind: None,
            consumables: Vec::new(),
            round_number: 1,
        }
    }

    /// Draw cards to fill the hand
    pub fn draw_hand(&mut self) -> GameResult<()> {
        self.hand.clear();
        let cards = self.deck.draw_multiple(self.hand_size.0)?;
        self.hand = cards;
        Ok(())
    }

    /// Play a hand of cards
    pub fn play_hand(&mut self, selected_cards: Vec<usize>) -> GameResult<Score> {
        if selected_cards.is_empty() {
            return Err(GameError::InvalidGameState("Cannot play empty hand".to_string()));
        }

        // Calculate score based on poker hand
        let played_cards: Vec<Card> = selected_cards
            .iter()
            .map(|&i| self.hand[i].clone())
            .collect();

        let hand_score = self.calculate_hand_score(&played_cards)?;
        
        // Apply joker effects
        let final_score = self.apply_joker_effects(hand_score, &played_cards)?;
        
        // Remove played cards from hand
        for &i in selected_cards.iter().rev() {
            self.hand.remove(i);
        }

        Ok(Score(final_score))
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
        
        for joker in &self.jokers {
            let (chip_mod, mult_mod) = joker.apply_effects(cards)?;
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
        self.ante = Ante(self.ante.0 + 1);
        self.phase = GamePhase::BlindSelect;
        Ok(())
    }
}
