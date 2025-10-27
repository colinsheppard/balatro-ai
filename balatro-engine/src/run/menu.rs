//! Initial menu handling

use crate::BalatroEngine;
use crate::run::user_input::get_user_input;
use log::info;

/// Handle the initial menu phase before creating GameState
pub fn handle_initial_menu(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting initial menu phase");
    
    loop {
        display_initial_menu(engine);
        let choice = get_user_input()?;
        
        if process_initial_menu_action(engine, choice)? {
            break; // Exit the menu loop
        }
    }
    
    Ok(())
}

/// Display the initial menu (before GameState creation)
fn display_initial_menu(engine: &BalatroEngine) {
    println!("\n=== BALATRO GAME MENU ===");
    println!("Welcome to Balatro!");
    
    // Display current selections
    if let Some(deck_type) = engine.selected_deck() {
        println!("Current Deck: {:?}", deck_type);
    } else {
        println!("Current Deck: Not selected");
    }
    
    if let Some(stake_level) = engine.selected_stake() {
        println!("Current Stake: {:?}", stake_level);
    } else {
        println!("Current Stake: Not selected");
    }
    
    println!("Select your deck and stake to begin:");
    
    display_menu_actions(engine);
}

/// Process initial menu action and return true if we should exit the menu
fn process_initial_menu_action(engine: &mut BalatroEngine, choice: u32) -> Result<bool, Box<dyn std::error::Error>> {
    let deck_types = engine.available_deck_types();
    let stake_levels = engine.available_stake_levels();
    let deck_count = deck_types.len() as u32;
    let stake_count = stake_levels.len() as u32;
    
    match choice {
        // Deck selection (1 to deck_count)
        n if n >= 1 && n <= deck_count => {
            let deck_index = (n - 1) as usize;
            let selected_deck = deck_types[deck_index].clone();
            println!("Selected deck: {:?}", selected_deck);
            engine.set_selected_deck(selected_deck);
        }
        // Stake selection (deck_count + 1 to deck_count + stake_count)
        n if n > deck_count && n <= deck_count + stake_count => {
            let stake_index = (n - deck_count - 1) as usize;
            let selected_stake = stake_levels[stake_index].clone();
            println!("Selected stake: {:?}", selected_stake);
            engine.set_selected_stake(selected_stake);
        }
        // Start game
        n if n == deck_count + stake_count + 1 => {
            println!("Starting new game...");
            engine.start_new_run_with_selections()?;
            return Ok(true); // Exit the menu
        }
        // Exit
        n if n == deck_count + stake_count + 2 => {
            println!("Exiting game...");
            std::process::exit(0);
        }
        _ => println!("Invalid menu choice: {}", choice),
    }
    
    Ok(false) // Continue in menu
}

/// Display available Menu actions
fn display_menu_actions(engine: &BalatroEngine) {
    println!("\nAvailable Actions:");
    let deck_types = engine.available_deck_types();
    let stake_levels = engine.available_stake_levels();
    
    // Display deck selection actions with descriptions
    for (i, deck_type) in deck_types.iter().enumerate() {
        let description = engine.get_deck_type_description(deck_type);
        println!("{}: Select {}", i + 1, description);
    }
    
    // Display stake selection actions with descriptions
    let stake_start = deck_types.len() + 1;
    for (i, stake_level) in stake_levels.iter().enumerate() {
        let description = engine.get_stake_level_description(stake_level);
        println!("{}: Select {}", stake_start + i, description);
    }
    
    // Display other actions
    let start_game_index = stake_start + stake_levels.len();
    println!("{}: Start Game", start_game_index);
    println!("{}: Exit", start_game_index + 1);
}

