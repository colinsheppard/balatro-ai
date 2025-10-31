//! Integration tests for the Balatro engine

use crate::{BalatroEngine, GamePhase, InputSource, JokerInstance};
use crate::joker::JokerManager;
use crate::deck::DeckType;
use crate::run::{handle_initial_menu, initialize_input_source, run_game_loop};
use std::env;

#[test]
fn test_engine_creation_and_initialization() {
    let engine = BalatroEngine::new(12345);
    // GameState is not created until start_new_run is called
    // This test now just verifies the engine can be created
    assert_eq!(engine.random_seed(), 12345);
}

#[test]
fn test_engine_start_new_run() {
    let mut engine = BalatroEngine::new(12345);
    engine.start_new_default_run().unwrap();
    
    let game_state = engine.game_state();
    assert_eq!(game_state.phase, GamePhase::BlindSelect);
    assert_eq!(game_state.ante, 1);
}

#[test]
fn test_full_game_cycle() {
    let mut engine = BalatroEngine::new(12345);
    engine.start_new_default_run().unwrap();
    
    let game_state = engine.game_state_mut();
    
    // Draw initial hand
    game_state.clear_and_draw_hand().unwrap();
    assert_eq!(game_state.hand.borrow().len(), 8);
    
    // Play a hand
    game_state.hand.borrow_mut().select_card(0).unwrap();
    game_state.hand.borrow_mut().select_card(1).unwrap();
    game_state.hand.borrow_mut().select_card(2).unwrap();
    let score = game_state.play_hand().unwrap();
    assert!(score > 0);
    assert_eq!(game_state.hand.borrow().len(), 5);

    // Redraw the hand
    game_state.draw_hand().unwrap();
    assert_eq!(game_state.hand.borrow().len(), 8);
    
    // End round
    game_state.end_round().unwrap();
    assert_eq!(game_state.phase, GamePhase::RoundEnd);
    
    // Start new ante
    game_state.start_new_ante().unwrap();
    assert_eq!(game_state.ante, 2);
    assert_eq!(game_state.phase, GamePhase::BlindSelect);
}

#[test]
fn test_joker_interaction_with_game_state() {
    let mut engine = BalatroEngine::new(12345);
    engine.start_new_default_run().unwrap();
    
    let game_state = engine.game_state_mut();
    
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
    game_state.clear_and_draw_hand().unwrap();
    
    // Select some cards to play
    game_state.hand.borrow_mut().select_card(0).unwrap();
    game_state.hand.borrow_mut().select_card(1).unwrap();
    game_state.hand.borrow_mut().select_card(2).unwrap();
    
    let score = game_state.play_hand().unwrap();
    
    // Score should be affected by joker (even if just base values)
    assert!(score > 0);
}

#[test]
fn test_deck_interaction_with_game_state() {
    let mut engine = BalatroEngine::new(12345);
    engine.start_new_default_run().unwrap();
    
    let game_state = engine.game_state_mut();
    
    // Change deck type - need to use the RNG manager from the engine
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::rng::GameRngManager;
    let rng = Rc::new(RefCell::new(GameRngManager::new(12345)));
    game_state.deck = crate::deck::Deck::new(DeckType::Blue, rng);
    
    // Draw hand
    game_state.clear_and_draw_hand().unwrap();
    assert_eq!(game_state.hand.borrow().len(), 8);
    assert_eq!(game_state.deck.borrow().n_remaining_cards(), 44);
}

#[test]
fn test_multiple_rounds() {
    let mut engine = BalatroEngine::new(12345);
    engine.start_new_default_run().unwrap();
    
    let game_state = engine.game_state_mut();
    
    // Play multiple rounds
    for round in 1..=3 {
        game_state.clear_and_draw_hand().unwrap();
        
        // Select some cards to play
        game_state.hand.borrow_mut().select_card(0).unwrap();
        game_state.hand.borrow_mut().select_card(1).unwrap();
        game_state.hand.borrow_mut().select_card(2).unwrap();
        
        game_state.play_hand().unwrap();
        game_state.end_round().unwrap();
        
        assert_eq!(game_state.round_number, round + 1);
    }
}

#[test]
fn test_engine_state_persistence() {
    let mut engine = BalatroEngine::new(12345);
    engine.start_new_default_run().unwrap();
    
    // Modify game state
    let game_state = engine.game_state_mut();
    game_state.money = 100;
    game_state.ante = 3;
    
    // Verify changes persist
    let game_state = engine.game_state();
    assert_eq!(game_state.money, 100);
    assert_eq!(game_state.ante, 3);
}

#[test]
fn test_integration_with_main_rs_via_env() {
    // Set BALATRO_INPUT_FILE programmatically - main.rs will read this
    env::set_var("BALATRO_INPUT_FILE", "input/play_one_pair.txt");
    
    // Use InputSource's read_all_commands helper method
    let input_file = env::var("BALATRO_INPUT_FILE").unwrap();
    initialize_input_source();
    let commands = InputSource::read_all_commands(&input_file).unwrap();
    
    println!("Commands read from {}: {:?}", input_file, commands);
    
    // This demonstrates that when BALATRO_INPUT_FILE is set,
    // main.rs will use InputSource::File variant to read these commands
    // in the initialize_input_source() -> InputSource::new() function

    let mut engine = BalatroEngine::new(12345);
    engine.start_new_default_run().unwrap();
    handle_initial_menu(&mut engine).unwrap();
    engine.game_state_mut().jokers.push(create_test_joker());
    run_game_loop(&mut engine).unwrap();
}

fn create_test_joker() -> JokerInstance {
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
    joker
}
