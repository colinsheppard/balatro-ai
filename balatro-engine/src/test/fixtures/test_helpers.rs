//! Test fixtures and utilities for testing

use crate::card::{Card, Suit, Rank};
use crate::joker::{JokerManager, JokerInstance, JokerRarity};
use crate::deck::{Deck, DeckType};

/// Create a standard test deck
pub fn create_test_deck() -> Deck {
    Deck::new(DeckType::Red)
}

/// Create a test hand with specific cards
pub fn create_test_hand() -> Vec<Card> {
    vec![
        Card::new(Suit::Hearts, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Queen),
        Card::new(Suit::Clubs, Rank::Jack),
        Card::new(Suit::Hearts, Rank::Ten),
    ]
}

/// Create a test joker manager with sample jokers
pub fn create_test_joker_manager() -> JokerManager {
    let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "test_joker"
name = "Test Joker"
description = "A test joker"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4

[[jokers]]
id = "greedy_joker"
name = "Greedy Joker"
description = "Played cards with Diamond suit give +3 Mult when scored"
rarity = "common"
cost = 5

[jokers.effect]
type = "conditional"
per_card = true

[jokers.effect.condition]
type = "suit_scored"
suit = "diamonds"

[jokers.effect.action]
type = "add_score"
mult = 3
"#;

    JokerManager::from_str(toml_content).unwrap()
}

/// Create a test joker instance
pub fn create_test_joker() -> JokerInstance {
    let manager = create_test_joker_manager();
    manager.create_joker("test_joker").unwrap()
}

/// Create a high-value test hand
pub fn create_high_value_hand() -> Vec<Card> {
    vec![
        Card::new(Suit::Hearts, Rank::King),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::King),
        Card::new(Suit::Clubs, Rank::King),
        Card::new(Suit::Hearts, Rank::Ace),
    ]
}

/// Create a low-value test hand
pub fn create_low_value_hand() -> Vec<Card> {
    vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Spades, Rank::Three),
        Card::new(Suit::Diamonds, Rank::Four),
        Card::new(Suit::Clubs, Rank::Five),
        Card::new(Suit::Hearts, Rank::Six),
    ]
}

/// Create a flush test hand
pub fn create_flush_hand() -> Vec<Card> {
    vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Hearts, Rank::Five),
        Card::new(Suit::Hearts, Rank::Seven),
        Card::new(Suit::Hearts, Rank::Ten),
        Card::new(Suit::Hearts, Rank::King),
    ]
}

/// Create a straight test hand
pub fn create_straight_hand() -> Vec<Card> {
    vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Spades, Rank::Three),
        Card::new(Suit::Diamonds, Rank::Four),
        Card::new(Suit::Clubs, Rank::Five),
        Card::new(Suit::Hearts, Rank::Six),
    ]
}

/// Create a pair test hand
pub fn create_pair_hand() -> Vec<Card> {
    vec![
        Card::new(Suit::Hearts, Rank::King),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::Ace),
        Card::new(Suit::Clubs, Rank::Two),
        Card::new(Suit::Hearts, Rank::Three),
    ]
}

/// Create a three-of-a-kind test hand
pub fn create_three_of_a_kind_hand() -> Vec<Card> {
    vec![
        Card::new(Suit::Hearts, Rank::King),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::King),
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Two),
    ]
}

/// Create a full house test hand
pub fn create_full_house_hand() -> Vec<Card> {
    vec![
        Card::new(Suit::Hearts, Rank::King),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::King),
        Card::new(Suit::Clubs, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Ace),
    ]
}

/// Create a four-of-a-kind test hand
pub fn create_four_of_a_kind_hand() -> Vec<Card> {
    vec![
        Card::new(Suit::Hearts, Rank::King),
        Card::new(Suit::Spades, Rank::King),
        Card::new(Suit::Diamonds, Rank::King),
        Card::new(Suit::Clubs, Rank::King),
        Card::new(Suit::Hearts, Rank::Ace),
    ]
}

/// Create a straight flush test hand
pub fn create_straight_flush_hand() -> Vec<Card> {
    vec![
        Card::new(Suit::Hearts, Rank::Two),
        Card::new(Suit::Hearts, Rank::Three),
        Card::new(Suit::Hearts, Rank::Four),
        Card::new(Suit::Hearts, Rank::Five),
        Card::new(Suit::Hearts, Rank::Six),
    ]
}

/// Create a royal flush test hand
pub fn create_royal_flush_hand() -> Vec<Card> {
    vec![
        Card::new(Suit::Hearts, Rank::Ten),
        Card::new(Suit::Hearts, Rank::Jack),
        Card::new(Suit::Hearts, Rank::Queen),
        Card::new(Suit::Hearts, Rank::King),
        Card::new(Suit::Hearts, Rank::Ace),
    ]
}

/// Create jokers by rarity using the manager
pub fn create_jokers_by_rarity() -> Vec<JokerInstance> {
    let manager = create_test_joker_manager();
    let mut jokers = Vec::new();
    
    // Create one joker of each rarity
    for rarity in [JokerRarity::Common, JokerRarity::Uncommon, JokerRarity::Rare, JokerRarity::Legendary] {
        let definitions = manager.get_by_rarity(rarity);
        if let Some(def) = definitions.first() {
            jokers.push(manager.create_joker(&def.id).unwrap());
        }
    }
    
    jokers
}

/// Calculate expected base score for a hand (sum of chip values)
pub fn calculate_base_score(cards: &[Card]) -> i32 {
    cards.iter().map(|card| card.chip_value()).sum()
}

/// Assert that a score is within expected range
pub fn assert_score_in_range(score: i32, min: i32, max: i32) {
    assert!(score >= min, "Score {} is below minimum {}", score, min);
    assert!(score <= max, "Score {} is above maximum {}", score, max);
}
