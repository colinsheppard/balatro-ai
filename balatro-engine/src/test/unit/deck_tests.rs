//! Unit tests for the deck system

use crate::deck::{Deck, DeckType};
use crate::card::{Suit, Rank};

#[test]
fn test_deck_creation() {
    let deck = Deck::new(DeckType::Red);
    assert_eq!(deck.remaining_cards(), 52);
    assert!(!deck.is_empty());
    assert_eq!(deck.discard_pile.len(), 0);
}

#[test]
fn test_deck_drawing() {
    let mut deck = Deck::new(DeckType::Red);
    
    // Draw one card
    let card = deck.draw().unwrap();
    assert!(card.is_some());
    assert_eq!(deck.remaining_cards(), 51);
    
    // Draw multiple cards
    let cards = deck.draw_multiple(5).unwrap();
    assert_eq!(cards.len(), 5);
    assert_eq!(deck.remaining_cards(), 46);
}

#[test]
fn test_deck_drawing_empty() {
    let mut deck = Deck::new(DeckType::Red);
    
    // Draw all cards
    for _ in 0..52 {
        deck.draw().unwrap();
    }
    
    assert!(deck.is_empty());
    assert_eq!(deck.remaining_cards(), 0);
    
    // Try to draw from empty deck
    let card = deck.draw().unwrap();
    assert!(card.is_none());
}

#[test]
fn test_deck_discarding() {
    let mut deck = Deck::new(DeckType::Red);
    let card = deck.draw().unwrap().unwrap();
    
    deck.discard(card.clone());
    assert_eq!(deck.discard_pile.len(), 1);
    assert_eq!(deck.discard_pile[0], card);
}

#[test]
fn test_deck_shuffling() {
    let mut deck1 = Deck::new(DeckType::Red);
    let mut deck2 = Deck::new(DeckType::Red);
    
    // Shuffle one deck
    deck1.shuffle();
    
    // Draw cards from both decks and compare
    let mut different = false;
    for _ in 0..10 {
        let card1 = deck1.draw().unwrap();
        let card2 = deck2.draw().unwrap();
        if card1 != card2 {
            different = true;
            break;
        }
    }
    
    // It's very unlikely they'd be identical after shuffling
    assert!(different);
}

#[test]
fn test_deck_type_variants() {
    let deck_types = [
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
    ];
    
    for deck_type in deck_types {
        let deck = Deck::new(deck_type.clone());
        assert_eq!(deck.deck_type, deck_type);
        assert_eq!(deck.remaining_cards(), 52);
    }
}

#[test]
fn test_standard_deck_composition() {
    let mut deck = Deck::new(DeckType::Red);
    let mut suit_counts = [0; 4];
    let mut rank_counts = [0; 13];
    
    // Count all cards
    while let Some(card) = deck.draw().unwrap() {
        match card.suit {
            Suit::Hearts => suit_counts[0] += 1,
            Suit::Diamonds => suit_counts[1] += 1,
            Suit::Clubs => suit_counts[2] += 1,
            Suit::Spades => suit_counts[3] += 1,
        }
        
        rank_counts[card.rank as usize - 1] += 1;
    }
    
    // Check we have 13 cards of each suit
    for count in suit_counts {
        assert_eq!(count, 13);
    }
    
    // Check we have 4 cards of each rank
    for count in rank_counts {
        assert_eq!(count, 4);
    }
}
