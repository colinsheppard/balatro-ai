//! Unit tests for planet system and poker hand detection

use crate::planet::{Planets, PokerHand};
use crate::card::{Card, Suit, Rank, SharedCard};
use std::rc::Rc;
use std::cell::RefCell;

fn to_shared(cards: Vec<Card>) -> Vec<SharedCard> {
    cards.into_iter().map(|c| Rc::new(RefCell::new(c))).collect()
}

#[test]
fn test_high_card() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Random high cards
    let cards = to_shared(vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Five),
        Card::new(Suit::Clubs, Rank::Seven),
        Card::new(Suit::Spades, Rank::Nine),
        Card::new(Suit::Hearts, Rank::Jack),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::HighCard);
    assert_eq!(res.1.len(), 1);
    assert_eq!(res.1[0].borrow().rank, Rank::Jack);
    
    // Test 2: Different high cards
    let cards = to_shared(vec![
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ten),
        Card::new(Suit::Diamonds, Rank::Eight),
        Card::new(Suit::Spades, Rank::Four),
        Card::new(Suit::Clubs, Rank::Three),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::HighCard);
    assert_eq!(res.1.len(), 1);
    assert_eq!(res.1[0].borrow().rank, Rank::Ace);
}

#[test]
fn test_pair() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Single pair
    let cards = to_shared(vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Two),
        Card::new(Suit::Clubs, Rank::Seven),
        Card::new(Suit::Spades, Rank::Nine),
        Card::new(Suit::Hearts, Rank::Jack),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::Pair);
    assert_eq!(res.1.len(), 2);
    assert!(res.1.iter().all(|c| c.borrow().rank == Rank::Two));
    
    // Test 2: Different pair
    let cards = to_shared(vec![
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Eight),
        Card::new(Suit::Spades, Rank::Four),
        Card::new(Suit::Clubs, Rank::Three),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::Pair);
    assert_eq!(res.1.len(), 2);
    assert!(res.1.iter().all(|c| c.borrow().rank == Rank::Ace));
}

#[test]
fn test_two_pair() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Two pairs
    let cards = to_shared(vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Two),
        Card::new(Suit::Clubs, Rank::Seven),
        Card::new(Suit::Spades, Rank::Seven),
        Card::new(Suit::Hearts, Rank::Jack),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::TwoPair);
    assert_eq!(res.1.len(), 4);
    let ranks: Vec<Rank> = res.1.iter().map(|c| c.borrow().rank).collect();
    assert!(ranks.iter().filter(|&&r| r == Rank::Two).count() == 2);
    assert!(ranks.iter().filter(|&&r| r == Rank::Seven).count() == 2);
    
    // Test 2: Different two pairs
    let cards = to_shared(vec![
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Eight),
        Card::new(Suit::Spades, Rank::Eight),
        Card::new(Suit::Clubs, Rank::Three),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::TwoPair);
    assert_eq!(res.1.len(), 4);
    let ranks: Vec<Rank> = res.1.iter().map(|c| c.borrow().rank).collect();
    assert!(ranks.iter().filter(|&&r| r == Rank::Ace).count() == 2);
    assert!(ranks.iter().filter(|&&r| r == Rank::Eight).count() == 2);
}

#[test]
fn test_three_of_a_kind() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Three of a kind
    let cards = to_shared(vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Two),
        Card::new(Suit::Clubs, Rank::Two),
        Card::new(Suit::Spades, Rank::Seven),
        Card::new(Suit::Hearts, Rank::Jack),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::ThreeOfAKind);
    assert_eq!(res.1.len(), 3);
    assert!(res.1.iter().all(|c| c.borrow().rank == Rank::Two));
    
    // Test 2: Different three of a kind
    let cards = to_shared(vec![
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Ace),
        Card::new(Suit::Spades, Rank::Eight),
        Card::new(Suit::Clubs, Rank::Three),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::ThreeOfAKind);
    assert_eq!(res.1.len(), 3);
    assert!(res.1.iter().all(|c| c.borrow().rank == Rank::Ace));
}

#[test]
fn test_straight() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Sequential straight
    let cards = to_shared(vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Three),
        Card::new(Suit::Clubs, Rank::Four),
        Card::new(Suit::Spades, Rank::Five),
        Card::new(Suit::Hearts, Rank::Six),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::Straight);
    assert_eq!(res.1.len(), 5);
    let ranks: Vec<Rank> = res.1.iter().map(|c| c.borrow().rank).collect();
    assert!(matches!(ranks[..], [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six]));
    
    // Test 2: Different straight
    let cards = to_shared(vec![
        Card::new(Suit::Clubs, Rank::Eight),
        Card::new(Suit::Hearts, Rank::Nine),
        Card::new(Suit::Diamonds, Rank::Ten),
        Card::new(Suit::Spades, Rank::Jack),
        Card::new(Suit::Clubs, Rank::Queen),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::Straight);
    assert_eq!(res.1.len(), 5);
    let ranks: Vec<Rank> = res.1.iter().map(|c| c.borrow().rank).collect();
    assert!(matches!(ranks[..], [Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen]));
}

#[test]
fn test_flush() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Flush (all same suit)
    let cards = to_shared(vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Hearts, Rank::Five),
        Card::new(Suit::Hearts, Rank::Seven),
        Card::new(Suit::Hearts, Rank::Nine),
        Card::new(Suit::Hearts, Rank::Jack),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::Flush);
    assert_eq!(res.1.len(), 5);
    assert!(res.1.iter().all(|c| c.borrow().suit == Suit::Hearts));
    
    // Test 2: Different flush
    let cards = to_shared(vec![
        Card::new(Suit::Diamonds, Rank::Three),
        Card::new(Suit::Diamonds, Rank::Six),
        Card::new(Suit::Diamonds, Rank::Eight),
        Card::new(Suit::Diamonds, Rank::Ten),
        Card::new(Suit::Diamonds, Rank::King),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::Flush);
    assert_eq!(res.1.len(), 5);
    assert!(res.1.iter().all(|c| c.borrow().suit == Suit::Diamonds));
}

#[test]
fn test_full_house() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Full house (three of one rank, pair of another)
    let cards = to_shared(vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Two),
        Card::new(Suit::Clubs, Rank::Two),
        Card::new(Suit::Spades, Rank::Seven),
        Card::new(Suit::Hearts, Rank::Seven),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::FullHouse);
    assert_eq!(res.1.len(), 5);
    let ranks: Vec<Rank> = res.1.iter().map(|c| c.borrow().rank).collect();
    assert!(ranks.iter().filter(|&&r| r == Rank::Two).count() == 3);
    assert!(ranks.iter().filter(|&&r| r == Rank::Seven).count() == 2);
    
    // Test 2: Different full house
    let cards = to_shared(vec![
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Ace),
        Card::new(Suit::Spades, Rank::Eight),
        Card::new(Suit::Clubs, Rank::Eight),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::FullHouse);
    assert_eq!(res.1.len(), 5);
    let ranks: Vec<Rank> = res.1.iter().map(|c| c.borrow().rank).collect();
    assert!(ranks.iter().filter(|&&r| r == Rank::Ace).count() == 3);
    assert!(ranks.iter().filter(|&&r| r == Rank::Eight).count() == 2);
}

#[test]
fn test_four_of_a_kind() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Four of a kind
    let cards = to_shared(vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Diamonds, Rank::Two),
        Card::new(Suit::Clubs, Rank::Two),
        Card::new(Suit::Spades, Rank::Two),
        Card::new(Suit::Hearts, Rank::Jack),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::FourOfAKind);
    assert_eq!(res.1.len(), 4);
    assert!(res.1.iter().all(|c| c.borrow().rank == Rank::Two));
    
    // Test 2: Different four of a kind
    let cards = to_shared(vec![
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Diamonds, Rank::Ace),
        Card::new(Suit::Spades, Rank::Ace),
        Card::new(Suit::Clubs, Rank::Three),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::FourOfAKind);
    assert_eq!(res.1.len(), 4);
    assert!(res.1.iter().all(|c| c.borrow().rank == Rank::Ace));
}

#[test]
fn test_straight_flush() {
    let planets = Planets::new_default().unwrap();
    
    // Test 1: Straight flush
    let cards = to_shared(vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Hearts, Rank::Three),
        Card::new(Suit::Hearts, Rank::Four),
        Card::new(Suit::Hearts, Rank::Five),
        Card::new(Suit::Hearts, Rank::Six),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::StraightFlush);
    assert_eq!(res.1.len(), 5);
    assert!(res.1.iter().all(|c| c.borrow().suit == Suit::Hearts));
    
    // Test 2: Different straight flush
    let cards = to_shared(vec![
        Card::new(Suit::Diamonds, Rank::Eight),
        Card::new(Suit::Diamonds, Rank::Nine),
        Card::new(Suit::Diamonds, Rank::Ten),
        Card::new(Suit::Diamonds, Rank::Jack),
        Card::new(Suit::Diamonds, Rank::Queen),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::StraightFlush);
    assert_eq!(res.1.len(), 5);
    assert!(res.1.iter().all(|c| c.borrow().suit == Suit::Diamonds));
}

#[test]
fn test_five_of_a_kind() {
    let planets = Planets::new_default().unwrap();
    let cards = to_shared(vec![
        Card::new(Suit::Hearts, Rank::Five),
        Card::new(Suit::Spades, Rank::Five),
        Card::new(Suit::Clubs, Rank::Five),
        Card::new(Suit::Diamonds, Rank::Five),
        Card::new(Suit::Hearts, Rank::Five),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::FiveOfAKind);
    assert_eq!(res.1.len(), 5);
    assert!(res.1.iter().all(|c| c.borrow().rank == Rank::Five));
}

#[test]
fn test_flush_house() {
    let planets = Planets::new_default().unwrap();
    let cards = to_shared(vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Hearts, Rank::Seven),
        Card::new(Suit::Hearts, Rank::Seven),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::FlushHouse);
    assert_eq!(res.1.len(), 5);
    assert!(res.1.iter().all(|c| c.borrow().suit == Suit::Hearts));
}

#[test]
fn test_flush_five() {
    let planets = Planets::new_default().unwrap();
    let cards = to_shared(vec![
        Card::new(Suit::Hearts, Rank::Nine),
        Card::new(Suit::Hearts, Rank::Nine),
        Card::new(Suit::Hearts, Rank::Nine),
        Card::new(Suit::Hearts, Rank::Nine),
        Card::new(Suit::Hearts, Rank::Nine),
    ]);
    let res = planets.detect_poker_hand(&cards).unwrap();
    assert_eq!(res.0, PokerHand::FlushFive);
    assert_eq!(res.1.len(), 5);
    assert!(res.1.iter().all(|c| c.borrow().suit == Suit::Hearts && c.borrow().rank == Rank::Nine));
}

