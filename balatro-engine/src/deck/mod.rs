//! Deck system for Balatro game engine

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use crate::card::{Card, Suit, Rank};
use crate::error::{GameError, GameResult};

/// Different deck types available in the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeckType {
    Red,
    Blue,
    Yellow,
    Green,
    Black,
    Magic,
    Nebula,
    Ghost,
    Abandoned,
    Checkered,
    Zodiac,
    Painted,
    Anaglyph,
    Plasma,
    Erratic,
}

/// A deck of cards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub deck_type: DeckType,
    pub cards: VecDeque<Card>,
    pub discard_pile: Vec<Card>,
}

impl Deck {
    /// Create a new standard deck
    pub fn new(deck_type: DeckType) -> Self {
        let mut cards = VecDeque::new();
        
        // Create standard 52-card deck
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in [
                Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
                Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
                Rank::Jack, Rank::Queen, Rank::King,
            ] {
                cards.push_back(Card::new(suit, rank));
            }
        }
        
        Self {
            deck_type,
            cards,
            discard_pile: Vec::new(),
        }
    }

    /// Draw a card from the deck
    pub fn draw(&mut self) -> GameResult<Option<Card>> {
        Ok(self.cards.pop_front())
    }

    /// Draw multiple cards from the deck
    pub fn draw_multiple(&mut self, count: usize) -> GameResult<Vec<Card>> {
        let mut drawn = Vec::new();
        for _ in 0..count {
            if let Some(card) = self.draw()? {
                drawn.push(card);
            } else {
                break;
            }
        }
        Ok(drawn)
    }

    /// Discard a card to the discard pile
    pub fn discard(&mut self, card: Card) {
        self.discard_pile.push(card);
    }

    /// Shuffle the deck
    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        
        let mut rng = thread_rng();
        let cards_vec: Vec<Card> = self.cards.drain(..).collect();
        let shuffled: Vec<Card> = cards_vec.choose_multiple(&mut rng, cards_vec.len()).cloned().collect();
        self.cards = shuffled.into();
    }

    /// Get the number of cards remaining in the deck
    pub fn remaining_cards(&self) -> usize {
        self.cards.len()
    }

    /// Check if the deck is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}
