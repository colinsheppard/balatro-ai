//! Deck system for Balatro game engine

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use rand::seq::SliceRandom;
use crate::SharedCard;
use crate::card::{Card, Suit, Rank};
use crate::error::GameResult;
use crate::rng::GameRngManager;

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


/// Type alias for a card that can be shared across the game
pub type SharedDeck = Rc<RefCell<Deck>>;
/// A deck of cards
#[derive(Debug, Clone)]
pub struct Deck {
    pub deck_type: DeckType,
    pub full_deck: Vec<SharedCard>,
    pub cards: VecDeque<SharedCard>,
    pub discard_pile: Vec<SharedCard>,
    // Skip serde serialization for the RNG manager
    #[allow(dead_code)]
    rng_manager: Rc<RefCell<GameRngManager>>,
}

impl Deck {
    /// Create a new standard deck, wrapped in Rc<RefCell<>>
    pub fn new(_deck_type: DeckType, rng_manager: Rc<RefCell<GameRngManager>>) -> Rc<RefCell<Self>> {
        let mut cards = VecDeque::new();
        let mut full_deck= Vec::new();
        
        // Create standard 52-card deck
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in [
                Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
                Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
                Rank::Jack, Rank::Queen, Rank::King,
            ] {
                let c = Rc::new(RefCell::new(Card::new(suit, rank)));
                full_deck.push(c.clone());
                cards.push_back(c.clone());
            }
        }
        
        let deck = Self {
            deck_type: _deck_type,
            full_deck,
            cards,
            discard_pile: Vec::new(),
            rng_manager,
        };
        
        Rc::new(RefCell::new(deck))
    }

    /// Draw a card from the deck
    pub fn draw(&mut self) -> GameResult<Option<SharedCard>> {
        Ok(self.cards.pop_front().map(|card| card.clone()))
    }

    /// Draw multiple cards from the deck
    pub fn draw_multiple(&mut self, count: usize) -> GameResult<Vec<SharedCard>> {
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
    pub fn discard(&mut self, card: SharedCard) {
        self.discard_pile.push(card);
    }

    /// Shuffle the deck
    pub fn shuffle(&mut self) {
        self.cards.clear();
        self.discard_pile.clear();
        let mut cards_vec: Vec<_> = self.full_deck.clone().into_iter().collect();
        cards_vec.shuffle(self.rng_manager.borrow_mut().get_rng("DECK_SHUFFLE"));
        self.cards = cards_vec.into();
    }

    /// Get the number of cards remaining in the deck
    pub fn n_remaining_cards(&self) -> usize {
        self.cards.len()
    }

    /// Get the number of cards in the full deck
    pub fn n_cards_full_deck(&self) -> usize {
        self.full_deck.len()
    }

    /// Check if the deck is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}
