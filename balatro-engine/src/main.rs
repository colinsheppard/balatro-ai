//! Balatro Game Engine - Main Entry Point
//! 
//! This is the main executable for the Balatro game engine.
//! It provides a command-line interface for running the game.

use balatro_engine::{
    BalatroEngine, GameState, GamePhase, Deck, Stake, BlindStatus
};
use log::info;
use std::env;
use std::io::{self, Write, BufRead, BufReader, Read};
use std::fs::{File, OpenOptions};

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
    
    // Display blind statuses
    println!("Blinds Status:");
    println!("  Small: {:?} (Score: {}, Money: {})", 
             game_state.upcoming_blinds.small.status,
             game_state.upcoming_blinds.small.required_score,
             game_state.upcoming_blinds.small.reward_money);
    println!("  Big: {:?} (Score: {}, Money: {})", 
             game_state.upcoming_blinds.big.status,
             game_state.upcoming_blinds.big.required_score,
             game_state.upcoming_blinds.big.reward_money);
    println!("  Boss: {:?} (Score: {}, Money: {})", 
             game_state.upcoming_blinds.boss.status,
             game_state.upcoming_blinds.boss.required_score,
             game_state.upcoming_blinds.boss.reward_money);
    
    println!("========================");
}

/// Handle the Menu phase
fn handle_menu_phase(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    display_menu_phase_state(engine);
    display_menu_actions(engine);
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
    display_blind_select_actions(engine.game_state());
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
fn display_menu_phase_state(engine: &BalatroEngine) {
    let game_state = engine.game_state();
    println!("\n--- MENU PHASE ---");
    println!("Welcome to Balatro!");
    println!("Current Deck: {:?}", game_state.deck.deck_type);
    println!("Current Stake: {:?}", game_state.stake.level);
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
    
    if let Some(next_blind) = game_state.upcoming_blinds.get_next_upcoming_blind() {
        println!("Next Blind: {}", next_blind.name);
        println!("Required Score: {}", next_blind.required_score);
        println!("Reward Money: ${}", next_blind.reward_money);
        println!("Type: {:?}", next_blind.blind_type);
        
        if let Some(boss_effect) = &next_blind.boss_effect {
            println!("Boss Effect: {:?}", boss_effect);
        }
    } else {
        println!("All blinds completed for this ante!");
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
fn display_blind_select_actions(game_state: &GameState) {
    println!("\nAvailable Actions:");
    
    if let Some(next_blind) = game_state.upcoming_blinds.get_next_upcoming_blind() {
        println!("1. Play {} - Face the blind and try to beat it", next_blind.name);
        
        if next_blind.can_skip() {
            println!("2. Skip {} - Skip this blind (costs money)", next_blind.name);
        }
    } else {
        println!("All blinds completed for this ante!");
        println!("1. Continue to next ante");
    }
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

/// Input source for automated testing
enum InputSource {
    Interactive,
    InteractiveRecording(File), // Interactive mode with recording
    File(BufReader<File>),
    FileRecording(BufReader<File>, File), // File input with recording
    Stdin(BufReader<io::Stdin>),
    StdinRecording(BufReader<io::Stdin>, File), // Stdin input with recording
}

impl InputSource {
    fn new() -> Self {
        // Check if recording is enabled first
        let recording_enabled = env::var("BALATRO_RECORD").is_ok();
        
        // Check if input file is provided via environment variable
        if let Ok(input_file) = env::var("BALATRO_INPUT_FILE") {
            let file = File::open(&input_file)
                .expect(&format!("Failed to open input file: {}", input_file));
            
            if recording_enabled {
                // Create recording file and wrap the input file reader
                let recording_file = Self::create_recording_file_internal();
                Self::FileRecording(BufReader::new(file), recording_file)
            } else {
                Self::File(BufReader::new(file))
            }
        } else {
            // Check if stdin has data (for piping)
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            let mut buffer = [0; 1];
            match handle.read(&mut buffer) {
                Ok(0) => {
                    // No data available, check if recording is enabled
                    if recording_enabled {
                        Self::create_recording_file()
                    } else {
                        Self::Interactive
                    }
                }
                Ok(_) => {
                    // Data available, read from stdin
                    if recording_enabled {
                        let recording_file = Self::create_recording_file_internal();
                        Self::StdinRecording(BufReader::new(io::stdin()), recording_file)
                    } else {
                        Self::Stdin(BufReader::new(io::stdin()))
                    }
                }
                Err(_) => {
                    // Check if recording is enabled
                    if recording_enabled {
                        Self::create_recording_file()
                    } else {
                        Self::Interactive
                    }
                }
            }
        }
    }

    fn create_recording_file() -> Self {
        // Create timestamped recording file
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("output/session_{}.txt", timestamp);
        
        // Ensure output directory exists
        std::fs::create_dir_all("output").expect("Failed to create output directory");
        
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&filename)
            .expect(&format!("Failed to create recording file: {}", filename));
        
        println!("Recording session to: {}", filename);
        Self::InteractiveRecording(file)
    }

    fn create_recording_file_internal() -> File {
        // Create timestamped recording file
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("output/session_{}.txt", timestamp);
        
        // Ensure output directory exists
        std::fs::create_dir_all("output").expect("Failed to create output directory");
        
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&filename)
            .expect(&format!("Failed to create recording file: {}", filename));
        
        println!("Recording session to: {}", filename);
        file
    }

    fn read_line(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::Interactive => {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                Ok(input)
            }
            Self::InteractiveRecording(file) => {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                // Record the input to file
                writeln!(file, "{}", input.trim())?;
                file.flush()?;
                
                Ok(input)
            }
            Self::File(reader) => {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                Ok(line)
            }
            Self::FileRecording(reader, file) => {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                
                // Record the input to file
                writeln!(file, "{}", line.trim())?;
                file.flush()?;
                
                Ok(line)
            }
            Self::Stdin(reader) => {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                Ok(line)
            }
            Self::StdinRecording(reader, file) => {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                
                // Record the input to file
                writeln!(file, "{}", line.trim())?;
                file.flush()?;
                
                Ok(line)
            }
        }
    }
}

// Global input source
static mut INPUT_SOURCE: Option<InputSource> = None;

fn initialize_input_source() {
    unsafe {
        INPUT_SOURCE = Some(InputSource::new());
    }
}

fn get_user_input() -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        if let Some(ref mut source) = INPUT_SOURCE {
            match source {
                InputSource::Interactive | InputSource::InteractiveRecording(_) => {
                    print!("\nEnter your choice (number) or 'quit' to exit: ");
                    io::stdout().flush()?;
                }
                _ => {
                    // For automated input, don't print prompt
                }
            }
            
            loop {
                let input = source.read_line()?;
                let trimmed = input.trim();
                
                // Skip empty lines and comments
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }
                
                // Check for quit command
                if trimmed.to_lowercase() == "quit" {
                    println!("Exiting game...");
                    std::process::exit(0);
                }
                
                match trimmed.parse::<u32>() {
                    Ok(choice) => return Ok(choice),
                    Err(_) => {
                        if matches!(source, InputSource::Interactive | InputSource::InteractiveRecording(_)) {
                            println!("Invalid input. Please enter a number or 'quit' to exit.");
                            continue;
                        } else {
                            return Err(format!("Invalid input in automated test: '{}'", trimmed).into());
                        }
                    }
                }
            }
        } else {
            Err("Input source not initialized".into())
        }
    }
}

/// Process Menu action
fn process_menu_action(engine: &mut BalatroEngine, choice: u32) -> Result<(), Box<dyn std::error::Error>> {
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
            engine.game_state_mut().deck = Deck::new(selected_deck);
        }
        // Stake selection (deck_count + 1 to deck_count + stake_count)
        n if n > deck_count && n <= deck_count + stake_count => {
            let stake_index = (n - deck_count - 1) as usize;
            let selected_stake = stake_levels[stake_index].clone();
            println!("Selected stake: {:?}", selected_stake);
            engine.game_state_mut().stake = Stake::new(selected_stake);
        }
        // Start game
        n if n == deck_count + stake_count + 1 => {
            println!("Starting new game...");
            engine.game_state_mut().phase = GamePhase::BlindSelect;
        }
        // Exit
        n if n == deck_count + stake_count + 2 => {
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

/// Process BlindSelect action
fn process_blind_select_action(engine: &mut BalatroEngine, choice: u32) -> Result<(), Box<dyn std::error::Error>> {
    let game_state = engine.game_state_mut();
    
    if let Some(next_blind) = game_state.upcoming_blinds.get_next_upcoming_blind() {
        match choice {
            1 => {
                // Play the blind
                println!("Playing {}...", next_blind.name);
                if let Some(blind_mut) = game_state.upcoming_blinds.get_next_upcoming_blind_mut() {
                    blind_mut.status = BlindStatus::Active;
                }
                game_state.phase = GamePhase::Playing;
            }
            2 => {
                // Skip the blind (only available for Small/Big blinds)
                if next_blind.can_skip() {
                    println!("Skipping {}...", next_blind.name);
                    if let Some(blind_mut) = game_state.upcoming_blinds.get_next_upcoming_blind_mut() {
                        blind_mut.status = BlindStatus::Skipped;
                    }
                    // TODO: Deduct skip cost from money
                    println!("Blind skipped! Moving to next blind or ante completion.");
                } else {
                    println!("Cannot skip boss blinds!");
                }
            }
            _ => println!("Invalid choice: {}", choice),
        }
    } else {
        // All blinds completed, move to next ante
        match choice {
            1 => {
                println!("Moving to next ante...");
                game_state.start_new_ante()?;
                game_state.generate_blinds()?;
            }
            _ => println!("Invalid choice: {}", choice),
        }
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
