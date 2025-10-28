//! Unit tests for planet system and poker hand detection

use crate::planet::{Planets, PokerHand};
use crate::card::{Card, Suit, Rank};

#[test]
fn test_high_card() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Random high cards
    let cards = vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Five),
        Card::new(Suit::Clubs, Rank::Seven),
        Card::new(Suit::Spades, Rank::Nine),
        Card::new(Suit::Hearts, Rank::Jack),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::HighCard));
    
    // Test 2: Different high cards
    let cards = vec![
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ten),
        Card::new(Suit::Diamonds, Rank::Eight),
        Card::new(Suit::Spades, Rank::Four),
        Card::new(Suit::Clubs, Rank::Three),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::HighCard));
}

#[test]
fn test_pair() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Single pair
    let cards = vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Two),
        Card::new(Suit::Clubs, Rank::Seven),
        Card::new(Suit::Spades, Rank::Nine),
        Card::new(Suit::Hearts, Rank::Jack),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::Pair));
    
    // Test 2: Different pair
    let cards = vec![
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Eight),
        Card::new(Suit::Spades, Rank::Four),
        Card::new(Suit::Clubs, Rank::Three),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::Pair));
}

#[test]
fn test_two_pair() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Two pairs
    let cards = vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Two),
        Card::new(Suit::Clubs, Rank::Seven),
        Card::new(Suit::Spades, Rank::Seven),
        Card::new(Suit::Hearts, Rank::Jack),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::TwoPair));
    
    // Test 2: Different two pairs
    let cards = vec![
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Eight),
        Card::new(Suit::Spades, Rank::Eight),
        Card::new(Suit::Clubs, Rank::Three),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::TwoPair));
}

#[test]
fn test_three_of_a_kind() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Three of a kind
    let cards = vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Two),
        Card::new(Suit::Clubs, Rank::Two),
        Card::new(Suit::Spades, Rank::Seven),
        Card::new(Suit::Hearts, Rank::Jack),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::ThreeOfAKind));
    
    // Test 2: Different three of a kind
    let cards = vec![
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Ace),
        Card::new(Suit::Spades, Rank::Eight),
        Card::new(Suit::Clubs, Rank::Three),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::ThreeOfAKind));
}

#[test]
fn test_straight() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Sequential straight
    let cards = vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Three),
        Card::new(Suit::Clubs, Rank::Four),
        Card::new(Suit::Spades, Rank::Five),
        Card::new(Suit::Hearts, Rank::Six),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::Straight));
    
    // Test 2: Different straight
    let cards = vec![
        Card::new(Suit::Clubs, Rank::Eight),
        Card::new(Suit::Hearts, Rank::Nine),
        Card::new(Suit::Diamonds, Rank::Ten),
        Card::new(Suit::Spades, Rank::Jack),
        Card::new(Suit::Clubs, Rank::Queen),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::Straight));
}

#[test]
fn test_flush() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Flush (all same suit)
    let cards = vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Hearts, Rank::Five),
        Card::new(Suit::Hearts, Rank::Seven),
        Card::new(Suit::Hearts, Rank::Nine),
        Card::new(Suit::Hearts, Rank::Jack),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::Flush));
    
    // Test 2: Different flush
    let cards = vec![
        Card::new(Suit::Diamonds, Rank::Three),
        Card::new(Suit::Diamonds, Rank::Six),
        Card::new(Suit::Diamonds, Rank::Eight),
        Card::new(Suit::Diamonds, Rank::Ten),
        Card::new(Suit::Diamonds, Rank::King),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::Flush));
}

#[test]
fn test_full_house() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Full house (three of one rank, pair of another)
    let cards = vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Two),
        Card::new(Suit::Clubs, Rank::Two),
        Card::new(Suit::Spades, Rank::Seven),
        Card::new(Suit::Hearts, Rank::Seven),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::FullHouse));
    
    // Test 2: Different full house
    let cards = vec![
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Ace),
        Card::new(Suit::Spades, Rank::Eight),
        Card::new(Suit::Clubs, Rank::Eight),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::FullHouse));
}

#[test]
fn test_four_of_a_kind() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Four of a kind
    let cards = vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Two),
        Card::new(Suit::Clubs, Rank::Two),
        Card::new(Suit::Spades, Rank::Two),
        Card::new(Suit::Hearts, Rank::Jack),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::FourOfAKind));
    
    // Test 2: Different four of a kind
    let cards = vec![
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Ace),
        Card::new(Suit::Spades, Rank::Ace),
        Card::new(Suit::Clubs, Rank::Three),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::FourOfAKind));
}

#[test]
fn test_straight_flush() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Straight flush
    let cards = vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Hearts, Rank::Three),
        Card::new(Suit::Hearts, Rank::Four),
        Card::new(Suit::Hearts, Rank::Five),
        Card::new(Suit::Hearts, Rank::Six),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::StraightFlush));
    
    // Test 2: Different straight flush
    let cards = vec![
        Card::new(Suit::Diamonds, Rank::Eight),
        Card::new(Suit::Diamonds, Rank::Nine),
        Card::new(Suit::Diamonds, Rank::Ten),
        Card::new(Suit::Diamonds, Rank::Jack),
        Card::new(Suit::Diamonds, Rank::Queen),
    ];
    assert_eq!(planets.detect_poker_hand(&cards), Some(PokerHand::StraightFlush));
}

