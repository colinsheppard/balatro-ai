//! Main game loop and phase handling

use crate::{BalatroEngine, GamePhase};
use crate::run::display::*;
use crate::run::phases::*;
use log::info;

/// Main game loop
pub fn run_game_loop(engine: &mut BalatroEngine) -> Result<bool, Box<dyn std::error::Error>> {
    info!("Starting main game loop");
    
    loop {
        let game_state = engine.game_state();
        
        // Display game state
        display_game_state(game_state);
        
        // Handle phase-specific logic
        match game_state.phase {
            GamePhase::Shop => {
                handle_shop_phase(engine)?;
            }
            GamePhase::ShopPackSelection => {
                // TODO: Handle pack selection
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
                let should_restart = handle_game_over_phase(engine)?;
                return Ok(should_restart); // Return whether to restart
            }
        }
    }
}

