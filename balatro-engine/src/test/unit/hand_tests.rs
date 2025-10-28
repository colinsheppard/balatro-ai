//! Unit tests for the hand management system

use crate::hand::Hand;
use crate::card::{Card, Suit, Rank};
use crate::SharedCard;
use std::rc::Rc;
use std::cell::RefCell;

fn create_test_card(suit: Suit, rank: Rank) -> Card {
    Card::new(suit, rank)
}

#[test]
fn test_hand_creation() {
    let hand = Hand::new();
    assert!(hand.is_empty());
    assert_eq!(hand.len(), 0);
}

#[test]
fn test_hand_with_cards() {
    let cards = vec![
        create_test_card(Suit::Hearts, Rank::Ace),
        create_test_card(Suit::Spades, Rank::King),
    ];
    let hand = Hand::with_cards(cards.clone());
    
    assert_eq!(hand.len(), 2);
    // Check that the cards are there by accessing their fields
    assert_eq!(hand.cards()[0].borrow().rank, Rank::Ace);
    assert_eq!(hand.cards()[1].borrow().rank, Rank::King);
}

#[test]
fn test_add_and_remove_cards() {
    let mut hand = Hand::new();
    let card = Rc::new(RefCell::new(create_test_card(Suit::Hearts, Rank::Ace)));
    
    hand.add_card(card.clone());
    assert_eq!(hand.len(), 1);
    
    let removed_card = hand.remove_card(0).unwrap();
    assert_eq!(removed_card.borrow().id, card.borrow().id);
    assert!(hand.is_empty());
}

#[test]
fn test_card_selection() {
    let mut hand = Hand::with_cards(vec![
        create_test_card(Suit::Hearts, Rank::Ace),
        create_test_card(Suit::Spades, Rank::King),
        create_test_card(Suit::Diamonds, Rank::Queen),
    ]);
    
    // Select cards
    hand.select_card(0).unwrap();
    hand.select_card(2).unwrap();
    
    assert!(hand.is_selected(0));
    assert!(!hand.is_selected(1));
    assert!(hand.is_selected(2));
    
    assert_eq!(hand.selected_indices(), &[0, 2]);
    assert_eq!(hand.selected_cards().len(), 2);
    
    // Deselect
    hand.deselect_card(0);
    assert!(!hand.is_selected(0));
    assert_eq!(hand.selected_indices(), &[2]);
}

#[test]
fn test_toggle_selection() {
    let mut hand = Hand::with_cards(vec![
        create_test_card(Suit::Hearts, Rank::Ace),
        create_test_card(Suit::Spades, Rank::King),
    ]);
    
    hand.toggle_selection(0).unwrap();
    assert!(hand.is_selected(0));
    
    hand.toggle_selection(0).unwrap();
    assert!(!hand.is_selected(0));
}

#[test]
fn test_move_cards() {
    let mut hand = Hand::with_cards(vec![
        create_test_card(Suit::Hearts, Rank::Ace),    // 0
        create_test_card(Suit::Spades, Rank::King),   // 1
        create_test_card(Suit::Diamonds, Rank::Queen), // 2
    ]);
    
    // Move card 1 left (should swap with card 0)
    hand.move_left(1).unwrap();
    assert_eq!(hand.get(0).unwrap().borrow().suit, Suit::Spades);
    assert_eq!(hand.get(1).unwrap().borrow().suit, Suit::Hearts);
    
    // Move card 1 right (should swap back)
    hand.move_right(0).unwrap();  // Move the card that's now at position 0
    assert_eq!(hand.get(0).unwrap().borrow().suit, Suit::Hearts);
    assert_eq!(hand.get(1).unwrap().borrow().suit, Suit::Spades);
}

#[test]
fn test_sort_by_rank_desc() {
    let mut hand = Hand::with_cards(vec![
        create_test_card(Suit::Hearts, Rank::Two),     // Lowest
        create_test_card(Suit::Spades, Rank::Ace),     // Highest
        create_test_card(Suit::Diamonds, Rank::King),  // Middle
    ]);
    
    hand.sort_by_rank_desc();
    
    // Should be sorted: Ace, King, Two (descending by rank value)
    assert_eq!(hand.get(0).unwrap().borrow().rank, Rank::Ace);
    assert_eq!(hand.get(1).unwrap().borrow().rank, Rank::King);
    assert_eq!(hand.get(2).unwrap().borrow().rank, Rank::Two);
}

#[test]
fn test_sort_by_suit_then_rank() {
    let mut hand = Hand::with_cards(vec![
        create_test_card(Suit::Spades, Rank::Two),
        create_test_card(Suit::Hearts, Rank::Ace),
        create_test_card(Suit::Spades, Rank::King),
        create_test_card(Suit::Hearts, Rank::Queen),
    ]);
    
    hand.sort_by_suit_then_rank();
    
    // Should be sorted by suit (Spadesfirst), then by rank (descending)
    assert_eq!(hand.get(0).unwrap().borrow().suit, Suit::Spades);
    assert_eq!(hand.get(0).unwrap().borrow().rank, Rank::King);
    assert_eq!(hand.get(1).unwrap().borrow().suit, Suit::Spades);
    assert_eq!(hand.get(1).unwrap().borrow().rank, Rank::Two);
    assert_eq!(hand.get(2).unwrap().borrow().suit, Suit::Hearts);
    assert_eq!(hand.get(2).unwrap().borrow().rank, Rank::Ace);
    assert_eq!(hand.get(3).unwrap().borrow().suit, Suit::Hearts);
    assert_eq!(hand.get(3).unwrap().borrow().rank, Rank::Queen);  // Queen (12) > Ace (1)
}

#[test]
fn test_remove_multiple_cards() {
    let mut hand = Hand::with_cards(vec![
        create_test_card(Suit::Hearts, Rank::Ace),     // 0
        create_test_card(Suit::Spades, Rank::King),    // 1
        create_test_card(Suit::Diamonds, Rank::Queen), // 2
        create_test_card(Suit::Clubs, Rank::Jack),     // 3
    ]);
    
    // Select cards at indices 0 and 2
    hand.select_card(0).unwrap();
    hand.select_card(2).unwrap();
    
    // Remove selected cards (indices 0 and 2)
    let removed = hand.remove_selected_cards().unwrap();
    assert_eq!(removed.len(), 2);
    assert_eq!(hand.len(), 2);
    
    // Remaining cards should be at original indices 1 and 3 (Spades and Clubs)
    assert_eq!(hand.get(0).unwrap().borrow().suit, Suit::Spades);
    assert_eq!(hand.get(1).unwrap().borrow().suit, Suit::Clubs);
    
    // Selections should be cleared after removal
    assert!(hand.selected_indices().is_empty());
}

#[test]
fn test_selection_persistence_after_move() {
    let mut hand = Hand::with_cards(vec![
        create_test_card(Suit::Hearts, Rank::Ace),     // 0
        create_test_card(Suit::Spades, Rank::King),    // 1
        create_test_card(Suit::Diamonds, Rank::Queen), // 2
    ]);
    
    // Select card at index 1
    hand.select_card(1).unwrap();
    assert!(hand.is_selected(1));
    
    // Move card 1 left
    hand.move_left(1).unwrap();
    
    // Card should now be selected at index 0
    assert!(!hand.is_selected(1));
    assert!(hand.is_selected(0));
}

#[test]
fn test_error_handling() {
    let mut hand = Hand::new();
    
    // Test out of bounds access
    assert!(hand.get(0).is_none());
    assert!(hand.remove_card(0).is_err());
    assert!(hand.select_card(0).is_err());
    assert!(hand.move_left(0).is_err());
    assert!(hand.move_right(0).is_err());
}

#[test]
fn test_total_values() {
    let hand = Hand::with_cards(vec![
        create_test_card(Suit::Hearts, Rank::Ace),
        create_test_card(Suit::Spades, Rank::King),
    ]);
    
    assert!(hand.total_chip_value() > 0);
    assert!(hand.total_mult_value() > 0.0);
}

#[test]
fn test_clear_operations() {
    let mut hand = Hand::with_cards(vec![
        create_test_card(Suit::Hearts, Rank::Ace),
        create_test_card(Suit::Spades, Rank::King),
    ]);
    
    hand.select_card(0).unwrap();
    assert!(!hand.is_empty());
    assert!(!hand.selected_indices().is_empty());
    
    hand.clear();
    assert!(hand.is_empty());
    assert!(hand.selected_indices().is_empty());
}

#[test]
fn test_clear_selections() {
    let mut hand = Hand::with_cards(vec![
        create_test_card(Suit::Hearts, Rank::Ace),
        create_test_card(Suit::Spades, Rank::King),
        create_test_card(Suit::Diamonds, Rank::Queen),
    ]);
    
    hand.select_card(0).unwrap();
    hand.select_card(2).unwrap();
    assert_eq!(hand.selected_indices().len(), 2);
    
    hand.clear_selections();
    assert!(hand.selected_indices().is_empty());
}

#[test]
fn test_from_into_conversions() {
    let cards = vec![
        create_test_card(Suit::Hearts, Rank::Ace),
        create_test_card(Suit::Spades, Rank::King),
    ];
    
    // Test From<Vec<Card>> for Hand
    let hand = Hand::from(cards.clone());
    assert_eq!(hand.len(), 2);
    
    // Test Into<Vec<SharedCard>> for Hand
    let converted_cards: Vec<SharedCard> = hand.into();
    assert_eq!(converted_cards.len(), 2);
}

#[test]
fn test_default_hand() {
    let hand = Hand::default();
    assert!(hand.is_empty());
    assert_eq!(hand.len(), 0);
}
