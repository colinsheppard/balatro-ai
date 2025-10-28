//! Unit tests for the card system

use crate::card::{Card, Suit, Rank, Enhancement, Edition, Seal};

#[test]
fn test_card_creation() {
    let card = Card::new(Suit::Hearts, Rank::Ace);
    assert_eq!(card.suit, Suit::Hearts);
    assert_eq!(card.rank, Rank::Ace);
    assert_eq!(card.chip_value(), 11);
    assert!(card.is_ace());
    assert!(!card.is_face_card());
}

#[test]
fn test_face_card_detection() {
    let jack = Card::new(Suit::Spades, Rank::Jack);
    let queen = Card::new(Suit::Diamonds, Rank::Queen);
    let king = Card::new(Suit::Clubs, Rank::King);
    
    assert!(jack.is_face_card());
    assert!(queen.is_face_card());
    assert!(king.is_face_card());
    
    let ace = Card::new(Suit::Hearts, Rank::Ace);
    assert!(!ace.is_face_card());
}

#[test]
fn test_card_chip_values() {
    assert_eq!(Card::new(Suit::Hearts, Rank::Ace).chip_value(), 11);
    assert_eq!(Card::new(Suit::Hearts, Rank::Two).chip_value(), 2);
    assert_eq!(Card::new(Suit::Hearts, Rank::Seven).chip_value(), 7);
    assert_eq!(Card::new(Suit::Hearts, Rank::Ten).chip_value(), 10);
    assert_eq!(Card::new(Suit::Hearts, Rank::Jack).chip_value(), 10);
    assert_eq!(Card::new(Suit::Hearts, Rank::Queen).chip_value(), 10);
    assert_eq!(Card::new(Suit::Hearts, Rank::King).chip_value(), 10);
}

#[test]
fn test_card_mult_values() {
    let card = Card::new(Suit::Hearts, Rank::Ace);
    assert_eq!(card.mult_value(), 1.0);
}

#[test]
fn test_suit_equality() {
    assert_eq!(Suit::Hearts, Suit::Hearts);
    assert_ne!(Suit::Hearts, Suit::Diamonds);
}

#[test]
fn test_rank_ordering() {
    assert!(Rank::Ace > Rank::Two);
    assert!(Rank::King > Rank::Queen);
    assert_eq!(Rank::Ten, Rank::Ten);
}

#[test]
fn test_enhancement_variants() {
    let enhancements = [
        Enhancement::Bonus,
        Enhancement::Mult,
        Enhancement::Wild,
        Enhancement::Glass,
        Enhancement::Steel,
        Enhancement::Stone,
        Enhancement::Gold,
        Enhancement::Lucky,
    ];
    
    for enhancement in enhancements {
        // Test that we can create cards with enhancements
        let mut card = Card::new(Suit::Hearts, Rank::Ace);
        card.enhancement = Some(enhancement.clone());
        assert_eq!(card.enhancement, Some(enhancement));
    }
}

#[test]
fn test_edition_variants() {
    let editions = [
        Edition::Base,
        Edition::Foil,
        Edition::Holographic,
        Edition::Polychrome,
        Edition::Negative,
    ];
    
    for edition in editions {
        let mut card = Card::new(Suit::Hearts, Rank::Ace);
        card.edition = edition.clone();
        assert_eq!(card.edition, edition);
    }
}

#[test]
fn test_seal_variants() {
    let seals = [
        Seal::Red,
        Seal::Blue,
        Seal::Purple,
        Seal::Gold,
    ];
    
    for seal in seals {
        let mut card = Card::new(Suit::Hearts, Rank::Ace);
        card.seal = Some(seal.clone());
        assert_eq!(card.seal, Some(seal));
    }
}
