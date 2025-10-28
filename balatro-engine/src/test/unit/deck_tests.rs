//! Unit tests for the deck system

use crate::deck::{Deck, DeckType};
use crate::card::{Suit, Rank};
use crate::test::fixtures::test_helpers::create_test_rng;

#[test]
fn test_deck_creation() {
    let rng = create_test_rng();
    let deck = Deck::new(DeckType::Red, rng);
    assert_eq!(deck.borrow().remaining_cards(), 52);
    assert!(!deck.borrow().is_empty());
    assert_eq!(deck.borrow().discard_pile.len(), 0);
}

#[test]
fn test_deck_drawing() {
    let rng = create_test_rng();
    let deck = Deck::new(DeckType::Red, rng);
    
    // Draw one card
    let card = deck.borrow_mut().draw().unwrap();
    assert!(card.is_some());
    assert_eq!(deck.borrow().remaining_cards(), 51);
    
    // Draw multiple cards
    let cards = deck.borrow_mut().draw_multiple(5).unwrap();
    assert_eq!(cards.len(), 5);
    assert_eq!(deck.borrow().remaining_cards(), 46);
}

#[test]
fn test_deck_drawing_empty() {
    let rng = create_test_rng();
    let deck = Deck::new(DeckType::Red, rng);
    
    // Draw all cards
    for _ in 0..52 {
        deck.borrow_mut().draw().unwrap();
    }
    
    assert!(deck.borrow().is_empty());
    assert_eq!(deck.borrow().remaining_cards(), 0);
    
    // Try to draw from empty deck
    let card = deck.borrow_mut().draw().unwrap();
    assert!(card.is_none());
}

#[test]
fn test_deck_discarding() {
    let rng = create_test_rng();
    let deck = Deck::new(DeckType::Red, rng);
    let card = deck.borrow_mut().draw().unwrap().unwrap();
    
    deck.borrow_mut().discard(card.clone());
    assert_eq!(deck.borrow().discard_pile.len(), 1);
    // Note: discard_pile now contains Rc<RefCell<Card>>, so we need to compare the inner values
    assert_eq!(deck.borrow().discard_pile[0].borrow().id, card.borrow().id);
}

#[test]
fn test_deck_shuffling() {
    let rng1 = create_test_rng();
    let rng2 = create_test_rng();
    let deck1 = Deck::new(DeckType::Red, rng1);
    let deck2 = Deck::new(DeckType::Red, rng2);
    
    // Shuffle one deck
    deck1.borrow_mut().shuffle();
    
    // Draw cards from both decks and compare
    let mut different = false;
    for _ in 0..10 {
        let card1 = deck1.borrow_mut().draw().unwrap();
        let card2 = deck2.borrow_mut().draw().unwrap();
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
    let mut rng = create_test_rng();
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
        let deck = Deck::new(deck_type.clone(), rng.clone());
        assert_eq!(deck.borrow().deck_type, deck_type);
        assert_eq!(deck.borrow().remaining_cards(), 52);
    }
}

#[test]
fn test_standard_deck_composition() {
    let rng = create_test_rng();
    let deck = Deck::new(DeckType::Red, rng);
    let mut suit_counts = [0; 4];
    let mut rank_counts = [0; 13];
    
    // Count all cards
    while let Some(card) = deck.borrow_mut().draw().unwrap() {
        match card.borrow().suit {
            Suit::Hearts => suit_counts[0] += 1,
            Suit::Diamonds => suit_counts[1] += 1,
            Suit::Clubs => suit_counts[2] += 1,
            Suit::Spades => suit_counts[3] += 1,
        }
        
        // Rank values are 2-14, array indices should be 0-12
        rank_counts[(card.borrow().rank as usize - 2)] += 1;
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
