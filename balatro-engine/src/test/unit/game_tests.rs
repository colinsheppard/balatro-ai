//! Unit tests for the game state system

use crate::game::{GameState, GamePhase};
use crate::deck::DeckType;
use crate::card::{Card, Suit, Rank};
use crate::test::fixtures::test_helpers::create_test_rng;

#[test]
fn test_game_state_creation() {
    let rng = create_test_rng();
    let game_state = GameState::new(rng);
    assert_eq!(game_state.phase, GamePhase::BlindSelect);
    assert_eq!(game_state.ante, 1);
    assert_eq!(game_state.hand_size, 8);
    assert_eq!(game_state.money, 4);
    assert_eq!(game_state.score, 0);
    assert_eq!(game_state.jokers.len(), 0);
    assert_eq!(game_state.hand.borrow().len(), 0);
    assert_eq!(game_state.round_number, 1);
}

#[test]
fn test_drawing_hand() {
    let rng = create_test_rng();
    let mut game_state = GameState::new(rng);
    game_state.hand_size = 5;
    
    game_state.draw_hand().unwrap();
    assert_eq!(game_state.hand.borrow().len(), 5);
    assert_eq!(game_state.deck.borrow().remaining_cards(), 47);
}

#[test]
fn test_playing_hand() {
    let rng = create_test_rng();
    let mut game_state = GameState::new(rng);
    game_state.hand_size = 5;
    game_state.draw_hand().unwrap();
    
    // Play first 3 cards
    game_state.hand.borrow_mut().select_card(0).unwrap();
    game_state.hand.borrow_mut().select_card(1).unwrap();
    game_state.hand.borrow_mut().select_card(2).unwrap();
    let score = game_state.play_hand().unwrap();
    
    assert!(score > 0); // Should have some score
    assert_eq!(game_state.hand.borrow().len(), 2); // Should have 2 cards left
}

#[test]
fn test_playing_empty_hand() {
    let rng = create_test_rng();
    let mut game_state = GameState::new(rng);
    let result = game_state.play_hand();
    assert!(result.is_err());
}

#[test]
fn test_ending_round() {
    let rng = create_test_rng();
    let mut game_state = GameState::new(rng);
    let initial_round = game_state.round_number;
    
    game_state.end_round().unwrap();
    assert_eq!(game_state.round_number, initial_round + 1);
    assert_eq!(game_state.phase, GamePhase::RoundEnd);
}

#[test]
fn test_starting_new_ante() {
    let rng = create_test_rng();
    let mut game_state = GameState::new(rng);
    let initial_ante = game_state.ante;
    
    game_state.start_new_ante().unwrap();
    assert_eq!(game_state.ante, initial_ante + 1);
    assert_eq!(game_state.phase, GamePhase::BlindSelect);
}

#[test]
fn test_game_phase_variants() {
    let phases = [
        GamePhase::Shop,
        GamePhase::BlindSelect,
        GamePhase::Playing,
        GamePhase::RoundEnd,
        GamePhase::GameOver,
    ];
    
    let rng = create_test_rng();
    for phase in phases {
        let mut game_state = GameState::new(rng.clone());
        game_state.phase = phase.clone();
        assert_eq!(game_state.phase, phase);
    }
}

#[test]
fn test_primitive_type_operations() {
    // Test that primitive types work as expected
    let ante: u32 = 1;
    let hand_size: usize = 8;
    let money: i32 = 4;
    let score: i32 = 0;
    
    assert!(ante < 2);
    assert!(hand_size > 5);
    assert!(money > 0);
    assert_eq!(score, 0);
}
