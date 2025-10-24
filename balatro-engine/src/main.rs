//! Balatro Game Engine - Main Entry Point
//! 
//! This is the main executable for the Balatro game engine.
//! It provides a command-line interface for running the game.

use balatro_engine::BalatroEngine;
use log::info;
use std::env;

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
    
    // TODO: Implement actual game loop
    // This will include:
    // - Handling user input
    // - Processing game state updates
    // - Rendering game state
    // - Managing game phases
    
    loop {
        let game_state = engine.game_state();
        info!("Current game phase: {:?}", game_state.phase);
        info!("Ante: {}, Money: {}, Score: {}", 
              game_state.ante, 
              game_state.money, 
              game_state.score);
        
        // For now, just break after one iteration
        // In the real implementation, this would handle user input and game logic
        break;
    }
    
    info!("Game loop ended");
    Ok(())
}
