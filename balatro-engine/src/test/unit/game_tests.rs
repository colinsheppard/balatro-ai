//! Unit tests for the game state system

use crate::game::{GameState, GamePhase};
use crate::deck::DeckType;
use crate::card::{Card, Suit, Rank};

#[test]
fn test_game_state_creation() {
    let game_state = GameState::new();
    assert_eq!(game_state.phase, GamePhase::Menu);
    assert_eq!(game_state.ante, 1);
    assert_eq!(game_state.hand_size, 8);
    assert_eq!(game_state.money, 4);
    assert_eq!(game_state.score, 0);
    assert_eq!(game_state.jokers.len(), 0);
    assert_eq!(game_state.hand.len(), 0);
    assert_eq!(game_state.round_number, 1);
}

#[test]
fn test_drawing_hand() {
    let mut game_state = GameState::new();
    game_state.hand_size = 5;
    
    game_state.draw_hand().unwrap();
    assert_eq!(game_state.hand.len(), 5);
    assert_eq!(game_state.deck.remaining_cards(), 47);
}

#[test]
fn test_playing_hand() {
    let mut game_state = GameState::new();
    game_state.hand_size = 5;
    game_state.draw_hand().unwrap();
    
    // Play first 3 cards
    let selected_cards = vec![0, 1, 2];
    let score = game_state.play_hand(selected_cards).unwrap();
    
    assert!(score > 0); // Should have some score
    assert_eq!(game_state.hand.len(), 2); // Should have 2 cards left
}

#[test]
fn test_playing_empty_hand() {
    let mut game_state = GameState::new();
    let result = game_state.play_hand(vec![]);
    assert!(result.is_err());
}

#[test]
fn test_ending_round() {
    let mut game_state = GameState::new();
    let initial_round = game_state.round_number;
    
    game_state.end_round().unwrap();
    assert_eq!(game_state.round_number, initial_round + 1);
    assert_eq!(game_state.phase, GamePhase::RoundEnd);
}

#[test]
fn test_starting_new_ante() {
    let mut game_state = GameState::new();
    let initial_ante = game_state.ante;
    
    game_state.start_new_ante().unwrap();
    assert_eq!(game_state.ante, initial_ante + 1);
    assert_eq!(game_state.phase, GamePhase::BlindSelect);
}

#[test]
fn test_game_phase_variants() {
    let phases = [
        GamePhase::Menu,
        GamePhase::Shop,
        GamePhase::BlindSelect,
        GamePhase::Playing,
        GamePhase::RoundEnd,
        GamePhase::GameOver,
    ];
    
    for phase in phases {
        let mut game_state = GameState::new();
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
