//! Game running logic for Balatro engine
//! 
//! This module contains all the functions to run a game instance,
//! handle different phases, display game state, and process user input.

// Re-export commonly used functions
pub use initialize::initialize_input_source;
pub use user_input::get_user_input;
pub use menu::handle_initial_menu;
pub use run::run_game_loop;

pub mod initialize;
pub mod user_input;
pub mod menu;
pub mod run;
pub mod display;
pub mod phases;

