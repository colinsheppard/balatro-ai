//! Balatro Game Engine - Main Entry Point
//! 
//! This is the main executable for the Balatro game engine.
//! It provides a command-line interface for running the game.

use balatro_engine::{BalatroEngine, GameState, GamePhase};
use log::info;
use std::env;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting Balatro Game Engine");
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let seed = if args.len() > 1 {
        args[1].parse().unwrap_or(12345)
    } else {
        12345
    };
    
    info!("Using random seed: {}", seed);
    
    // Create and initialize the game engine
    let mut engine = BalatroEngine::new(seed);
    
    // Start a new game run
    let engine = &mut engine;
    engine.start_new_default_run().unwrap();
    
    info!("Game engine initialized successfully");
    
    // Main game loop placeholder
    run_game_loop(engine)?;
    
    Ok(())
}

/// Main game loop
fn run_game_loop(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting main game loop");
    
    loop {
        let game_state = engine.game_state();
        
        // Display game state
        display_game_state(game_state);
        
        // Handle phase-specific logic
        match game_state.phase {
            GamePhase::Menu => {
                handle_menu_phase(engine)?;
            }
            GamePhase::Shop => {
                handle_shop_phase(engine)?;
            }
            GamePhase::BlindSelect => {
                handle_blind_select_phase(engine)?;
            }
            GamePhase::Playing => {
                handle_playing_phase(engine)?;
            }
            GamePhase::RoundEnd => {
                handle_round_end_phase(engine)?;
            }
            GamePhase::GameOver => {
                handle_game_over_phase(engine)?;
                break; // Exit the game loop
            }
        }
    }
    
    info!("Game loop ended");
    Ok(())
}

/// Display the current game state
fn display_game_state(game_state: &GameState) {
    println!("\n=== BALATRO GAME STATE ===");
    println!("Phase: {:?}", game_state.phase);
    println!("Ante: {}", game_state.ante);
    println!("Round: {}", game_state.round_number);
    println!("Money: ${}", game_state.money);
    println!("Score: {}", game_state.score);
    println!("Hand Size: {}", game_state.hand_size);
    println!("Jokers: {}", game_state.jokers.len());
    println!("Consumables: {}", game_state.consumables.len());
    println!("========================");
}

/// Handle the Menu phase
fn handle_menu_phase(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    display_menu_phase_state(engine.game_state());
    display_menu_actions();
    let choice = get_user_input()?;
    process_menu_action(engine, choice)?;
    Ok(())
}

/// Handle the Shop phase
fn handle_shop_phase(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    display_shop_phase_state(engine.game_state());
    display_shop_actions();
    let choice = get_user_input()?;
    process_shop_action(engine, choice)?;
    Ok(())
}

/// Handle the BlindSelect phase
fn handle_blind_select_phase(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    display_blind_select_phase_state(engine.game_state());
    display_blind_select_actions();
    let choice = get_user_input()?;
    process_blind_select_action(engine, choice)?;
    Ok(())
}

/// Handle the Playing phase
fn handle_playing_phase(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    display_playing_phase_state(engine.game_state());
    display_playing_actions();
    let choice = get_user_input()?;
    process_playing_action(engine, choice)?;
    Ok(())
}

/// Handle the RoundEnd phase
fn handle_round_end_phase(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    display_round_end_phase_state(engine.game_state());
    display_round_end_actions();
    let choice = get_user_input()?;
    process_round_end_action(engine, choice)?;
    Ok(())
}

/// Handle the GameOver phase
fn handle_game_over_phase(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    display_game_over_phase_state(engine.game_state());
    display_game_over_actions();
    let choice = get_user_input()?;
    process_game_over_action(engine, choice)?;
    Ok(())
}

/// Display Menu phase specific state
fn display_menu_phase_state(game_state: &GameState) {
    println!("\n--- MENU PHASE ---");
    println!("Welcome to Balatro!");
    println!("Current Stake: {:?}", game_state.stake);
    println!("Deck Type: {:?}", game_state.deck.deck_type);
}

/// Display Shop phase specific state
fn display_shop_phase_state(game_state: &GameState) {
    println!("\n--- SHOP PHASE ---");
    println!("Available Money: ${}", game_state.money);
    println!("Current Jokers: {}", game_state.jokers.len());
    println!("Available Consumables: {}", game_state.consumables.len());
    // TODO: Display actual shop items
}

/// Display BlindSelect phase specific state
fn display_blind_select_phase_state(game_state: &GameState) {
    println!("\n--- BLIND SELECT PHASE ---");
    println!("Ante {}", game_state.ante);
    if let Some(blind) = &game_state.current_blind {
        println!("Current Blind: {:?}", blind);
    } else {
        println!("No blind selected");
    }
}

/// Display Playing phase specific state
fn display_playing_phase_state(game_state: &GameState) {
    println!("\n--- PLAYING PHASE ---");
    println!("Hand:");
    for (i, card) in game_state.hand.cards().iter().enumerate() {
        let selected = if game_state.hand.selected_indices().contains(&i) { " [SELECTED]" } else { "" };
        println!("  {}: {:?}{}", i + 1, card, selected);
    }
    println!("Jokers:");
    for (i, joker) in game_state.jokers.iter().enumerate() {
        println!("  {}: {:?}", i + 1, joker);
    }
}

/// Display RoundEnd phase specific state
fn display_round_end_phase_state(game_state: &GameState) {
    println!("\n--- ROUND END PHASE ---");
    println!("Round {} Complete!", game_state.round_number);
    println!("Final Score: {}", game_state.score);
    println!("Money Earned: ${}", game_state.money);
}

/// Display GameOver phase specific state
fn display_game_over_phase_state(game_state: &GameState) {
    println!("\n--- GAME OVER ---");
    println!("Final Score: {}", game_state.score);
    println!("Antes Reached: {}", game_state.ante);
    println!("Rounds Played: {}", game_state.round_number);
}

/// Display available Menu actions
fn display_menu_actions() {
    println!("\nAvailable Actions:");
    println!("1. Start New Game");
    println!("2. Continue Game");
    println!("3. Settings");
    println!("4. Exit");
}

/// Display available Shop actions
fn display_shop_actions() {
    println!("\nAvailable Actions:");
    println!("1. Buy Joker");
    println!("2. Buy Consumable");
    println!("3. Sell Joker");
    println!("4. Skip Shop");
    println!("5. View Deck");
}

/// Display available BlindSelect actions
fn display_blind_select_actions() {
    println!("\nAvailable Actions:");
    println!("1. Select Boss Blind");
    println!("2. Select Elite Blind");
    println!("3. Select Normal Blind");
    println!("4. View Blind Details");
}

/// Display available Playing actions
fn display_playing_actions() {
    println!("\nAvailable Actions:");
    println!("1. Select Card");
    println!("2. Deselect Card");
    println!("3. Play Hand");
    println!("4. Discard Hand");
    println!("5. View Jokers");
    println!("6. Use Consumable");
}

/// Display available RoundEnd actions
fn display_round_end_actions() {
    println!("\nAvailable Actions:");
    println!("1. Continue to Shop");
    println!("2. View Statistics");
    println!("3. Save Game");
}

/// Display available GameOver actions
fn display_game_over_actions() {
    println!("\nAvailable Actions:");
    println!("1. Play Again");
    println!("2. Main Menu");
    println!("3. View Final Statistics");
    println!("4. Exit");
}

/// Get user input as a number
fn get_user_input() -> Result<u32, Box<dyn std::error::Error>> {
    print!("\nEnter your choice (number): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let choice = input.trim().parse::<u32>()?;
    Ok(choice)
}

/// Process Menu action (stub)
fn process_menu_action(engine: &mut BalatroEngine, choice: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("Menu action {} selected (stub)", choice);
    // TODO: Implement actual menu actions
    match choice {
        1 => {
            println!("Starting new game...");
            engine.game_state_mut().phase = GamePhase::BlindSelect;
        }
        4 => {
            println!("Exiting game...");
            engine.game_state_mut().phase = GamePhase::GameOver;
        }
        _ => println!("Invalid menu choice: {}", choice),
    }
    Ok(())
}

/// Process Shop action (stub)
fn process_shop_action(engine: &mut BalatroEngine, choice: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("Shop action {} selected (stub)", choice);
    // TODO: Implement actual shop actions
    match choice {
        4 => {
            println!("Skipping shop...");
            engine.game_state_mut().phase = GamePhase::BlindSelect;
        }
        _ => println!("Invalid shop choice: {}", choice),
    }
    Ok(())
}

/// Process BlindSelect action (stub)
fn process_blind_select_action(engine: &mut BalatroEngine, choice: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("BlindSelect action {} selected (stub)", choice);
    // TODO: Implement actual blind selection actions
    match choice {
        1..=3 => {
            println!("Blind selected, starting play phase...");
            engine.game_state_mut().phase = GamePhase::Playing;
        }
        _ => println!("Invalid blind select choice: {}", choice),
    }
    Ok(())
}

/// Process Playing action (stub)
fn process_playing_action(engine: &mut BalatroEngine, choice: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("Playing action {} selected (stub)", choice);
    // TODO: Implement actual playing actions
    match choice {
        3 => {
            println!("Playing hand...");
            engine.game_state_mut().phase = GamePhase::RoundEnd;
        }
        4 => {
            println!("Discarding hand...");
            engine.game_state_mut().phase = GamePhase::RoundEnd;
        }
        _ => println!("Invalid playing choice: {}", choice),
    }
    Ok(())
}

/// Process RoundEnd action (stub)
fn process_round_end_action(engine: &mut BalatroEngine, choice: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("RoundEnd action {} selected (stub)", choice);
    // TODO: Implement actual round end actions
    match choice {
        1 => {
            println!("Continuing to shop...");
            engine.game_state_mut().phase = GamePhase::Shop;
        }
        _ => println!("Invalid round end choice: {}", choice),
    }
    Ok(())
}

/// Process GameOver action (stub)
fn process_game_over_action(engine: &mut BalatroEngine, choice: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("GameOver action {} selected (stub)", choice);
    // TODO: Implement actual game over actions
    match choice {
        1 => {
            println!("Starting new game...");
            engine.game_state_mut().phase = GamePhase::Menu;
        }
        4 => {
            println!("Exiting...");
            // This will cause the game loop to exit
        }
        _ => println!("Invalid game over choice: {}", choice),
    }
    Ok(())
}
