//! Deck system for Balatro game engine

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt;
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

impl fmt::Display for DeckType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeckType::Red => write!(f, "Red"),
            DeckType::Blue => write!(f, "Blue"),
            DeckType::Yellow => write!(f, "Yellow"),
            DeckType::Green => write!(f, "Green"),
            DeckType::Black => write!(f, "Black"),
            DeckType::Magic => write!(f, "Magic"),
            DeckType::Nebula => write!(f, "Nebula"),
            DeckType::Ghost => write!(f, "Ghost"),
            DeckType::Abandoned => write!(f, "Abandoned"),
            DeckType::Checkered => write!(f, "Checkered"),
            DeckType::Zodiac => write!(f, "Zodiac"),
            DeckType::Painted => write!(f, "Painted"),
            DeckType::Anaglyph => write!(f, "Anaglyph"),
            DeckType::Plasma => write!(f, "Plasma"),
            DeckType::Erratic => write!(f, "Erratic"),
        }
    }
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
