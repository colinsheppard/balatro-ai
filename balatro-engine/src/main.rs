//! Balatro Game Engine - Main Entry Point
//! 
//! This is the main executable for the Balatro game engine.
//! It provides a command-line interface for running the game.

use balatro_engine::{
    BalatroEngine,
    run::{
        initialize_input_source,
        handle_initial_menu,
        run_game_loop,
    },
};
use log::info;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut seed = 12345;
    let mut record_session = false;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--record" => {
                record_session = true;
                env::set_var("BALATRO_RECORD", "1");
            }
            "--seed" => {
                if i + 1 < args.len() {
                    seed = args[i + 1].parse().unwrap_or(12345);
                    i += 1; // Skip the seed value
                }
            }
            _ => {
                // Try to parse as seed (backward compatibility)
                if let Ok(parsed_seed) = args[i].parse::<u64>() {
                    seed = parsed_seed;
                }
            }
        }
        i += 1;
    }
    
    // Initialize input source for automated testing
    initialize_input_source();
    
    info!("Starting Balatro Game Engine");
    info!("Using random seed: {}", seed);
    if record_session {
        info!("Session recording enabled");
    }
    
    // Main game loop with restart capability
    loop {
        // Create the game engine (without initializing GameState yet)
        let mut engine = BalatroEngine::new(seed);
        
        info!("Game engine initialized successfully");
        
        // Handle menu phase before creating GameState
        handle_initial_menu(&mut engine)?;
        
        // Main game loop
        if !run_game_loop(&mut engine)? {
            break; // Exit the outer loop
        }
        // If we reach here, the game ended and we should restart
    }
    
    Ok(())
}
