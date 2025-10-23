//! Integration tests for the Balatro engine

use crate::{BalatroEngine, GamePhase};
use crate::joker::{JokerManager, JokerInstance, JokerRarity};
use crate::deck::DeckType;

#[test]
fn test_engine_creation_and_initialization() {
    let engine = BalatroEngine::new(12345);
    let game_state = engine.game_state();
    
    assert_eq!(game_state.ante.0, 1);
    assert_eq!(game_state.money.0, 4);
    assert_eq!(game_state.hand_size.0, 8);
}

#[test]
fn test_engine_start_new_run() {
    let mut engine = BalatroEngine::new(12345);
    engine.start_new_run().unwrap();
    
    let game_state = engine.game_state();
    assert_eq!(game_state.phase, GamePhase::Menu);
    assert_eq!(game_state.ante.0, 1);
}

#[test]
fn test_full_game_cycle() {
    let mut engine = BalatroEngine::new(12345);
    engine.start_new_run().unwrap();
    
    let mut game_state = engine.game_state_mut();
    
    // Draw initial hand
    game_state.draw_hand().unwrap();
    assert_eq!(game_state.hand.len(), 8);
    
    // Play a hand
    let score = game_state.play_hand(vec![0, 1, 2]).unwrap();
    assert!(score.0 > 0);
    assert_eq!(game_state.hand.len(), 5);
    
    // End round
    game_state.end_round().unwrap();
    assert_eq!(game_state.phase, GamePhase::RoundEnd);
    
    // Start new ante
    game_state.start_new_ante().unwrap();
    assert_eq!(game_state.ante.0, 2);
    assert_eq!(game_state.phase, GamePhase::BlindSelect);
}

#[test]
fn test_joker_interaction_with_game_state() {
    let mut engine = BalatroEngine::new(12345);
    engine.start_new_run().unwrap();
    
    let mut game_state = engine.game_state_mut();
    
    // Add a joker using TOML-based system
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
"#;

    let manager = JokerManager::from_str(toml_content).unwrap();
    let joker = manager.create_joker("test_joker").unwrap();
    game_state.jokers.push(joker);
    
    // Draw and play hand
    game_state.draw_hand().unwrap();
    let score = game_state.play_hand(vec![0, 1, 2]).unwrap();
    
    // Score should be affected by joker (even if just base values)
    assert!(score.0 > 0);
}

#[test]
fn test_deck_interaction_with_game_state() {
    let mut engine = BalatroEngine::new(12345);
    engine.start_new_run().unwrap();
    
    let mut game_state = engine.game_state_mut();
    
    // Change deck type
    game_state.deck = crate::deck::Deck::new(DeckType::Blue);
    
    // Draw hand
    game_state.draw_hand().unwrap();
    assert_eq!(game_state.hand.len(), 8);
    assert_eq!(game_state.deck.remaining_cards(), 44);
}

#[test]
fn test_multiple_rounds() {
    let mut engine = BalatroEngine::new(12345);
    engine.start_new_run().unwrap();
    
    let mut game_state = engine.game_state_mut();
    
    // Play multiple rounds
    for round in 1..=3 {
        game_state.draw_hand().unwrap();
        game_state.play_hand(vec![0, 1, 2]).unwrap();
        game_state.end_round().unwrap();
        
        assert_eq!(game_state.round_number, round + 1);
    }
}

#[test]
fn test_engine_state_persistence() {
    let mut engine = BalatroEngine::new(12345);
    engine.start_new_run().unwrap();
    
    // Modify game state
    let mut game_state = engine.game_state_mut();
    game_state.money = crate::game::Money(100);
    game_state.ante = crate::game::Ante(3);
    
    // Verify changes persist
    let game_state = engine.game_state();
    assert_eq!(game_state.money.0, 100);
    assert_eq!(game_state.ante.0, 3);
}
