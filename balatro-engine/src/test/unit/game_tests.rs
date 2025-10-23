//! Unit tests for the game state system

use crate::game::{GameState, GamePhase, Ante, HandSize, Money, Score};
use crate::deck::DeckType;
use crate::card::{Card, Suit, Rank};

#[test]
fn test_game_state_creation() {
    let game_state = GameState::new();
    assert_eq!(game_state.phase, GamePhase::Menu);
    assert_eq!(game_state.ante.0, 1);
    assert_eq!(game_state.hand_size.0, 8);
    assert_eq!(game_state.money.0, 4);
    assert_eq!(game_state.score.0, 0);
    assert_eq!(game_state.jokers.len(), 0);
    assert_eq!(game_state.hand.len(), 0);
    assert_eq!(game_state.round_number, 1);
}

#[test]
fn test_drawing_hand() {
    let mut game_state = GameState::new();
    game_state.hand_size = HandSize(5);
    
    game_state.draw_hand().unwrap();
    assert_eq!(game_state.hand.len(), 5);
    assert_eq!(game_state.deck.remaining_cards(), 47);
}

#[test]
fn test_playing_hand() {
    let mut game_state = GameState::new();
    game_state.hand_size = HandSize(5);
    game_state.draw_hand().unwrap();
    
    // Play first 3 cards
    let selected_cards = vec![0, 1, 2];
    let score = game_state.play_hand(selected_cards).unwrap();
    
    assert!(score.0 > 0); // Should have some score
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
    let initial_ante = game_state.ante.0;
    
    game_state.start_new_ante().unwrap();
    assert_eq!(game_state.ante.0, initial_ante + 1);
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
fn test_ante_ordering() {
    assert!(Ante(1) < Ante(2));
    assert!(Ante(5) > Ante(3));
    assert_eq!(Ante(4), Ante(4));
}

#[test]
fn test_hand_size_ordering() {
    assert!(HandSize(5) < HandSize(8));
    assert!(HandSize(10) > HandSize(7));
    assert_eq!(HandSize(8), HandSize(8));
}

#[test]
fn test_money_operations() {
    assert!(Money(10) > Money(5));
    assert!(Money(3) < Money(7));
    assert_eq!(Money(4), Money(4));
}

#[test]
fn test_score_operations() {
    assert!(Score(1000) > Score(500));
    assert!(Score(200) < Score(800));
    assert_eq!(Score(1500), Score(1500));
}
