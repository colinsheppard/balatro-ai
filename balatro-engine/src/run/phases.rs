//! Phase handling for the game

use crate::{BalatroEngine, GamePhase, BlindStatus};
use crate::run::display::*;
use crate::run::user_input::get_user_input;

/// Handle the Shop phase
pub fn handle_shop_phase(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    display_shop_phase_state(engine.game_state());
    display_shop_actions();
    let choice = get_user_input()?;
    process_shop_action(engine, choice)?;
    Ok(())
}

/// Handle the BlindSelect phase
pub fn handle_blind_select_phase(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    display_blind_select_phase_state(engine.game_state());
    display_blind_select_actions(engine.game_state());
    let choice = get_user_input()?;
    process_blind_select_action(engine, choice)?;
    Ok(())
}

/// Handle the Playing phase
pub fn handle_playing_phase(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    display_playing_phase_state(engine.game_state());
    let playing_actions = crate::actions::helpers::create_playing_actions(engine.game_state());
    display_playing_actions(engine.game_state(), &playing_actions);
    let choice = get_user_input()?;
    process_playing_action(engine, &playing_actions, choice)?;
    Ok(())
}

/// Handle the RoundEnd phase
pub fn handle_round_end_phase(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    display_round_end_phase_state(engine.game_state());
    display_round_end_actions();
    let choice = get_user_input()?;
    process_round_end_action(engine, choice)?;
    Ok(())
}

/// Handle the GameOver phase
pub fn handle_game_over_phase(engine: &mut BalatroEngine) -> Result<bool, Box<dyn std::error::Error>> {
    display_game_over_phase_state(engine.game_state());
    display_game_over_actions();
    let choice = get_user_input()?;
    let should_restart = process_game_over_action(engine, choice)?;
    Ok(should_restart)
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
                game_state.clear_and_draw_hand().unwrap();
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
fn process_playing_action(engine: &mut BalatroEngine, playing_actions: &[(u32, crate::actions::PlayingAction)], choice: u32) -> Result<(), Box<dyn std::error::Error>> {
    let (_action_num, action) = playing_actions.get(choice as usize)
        .ok_or_else(|| format!("Invalid choice: {}", choice))?;
    
    match action {
        crate::actions::PlayingAction::PlaySelectedCards => {
            println!("Playing selected cards...");
            let _score = engine.game_state_mut().play_hand().unwrap_or(0);
            engine.game_state_mut().phase = GamePhase::RoundEnd;
            Ok(())
        }
        crate::actions::PlayingAction::DiscardSelectedCards => {
            println!("Discarding selected cards...");
            engine.game_state_mut().phase = GamePhase::RoundEnd;
            Ok(())
        }
        crate::actions::PlayingAction::SelectCard(card_idx) => {
            println!("Selecting card at index {}", card_idx);
            engine.game_state_mut().hand.select_card(*card_idx);
            Ok(())
        }
        crate::actions::PlayingAction::DeselectCard(card_idx) => {
            println!("Deselecting card at index {}", card_idx);
            engine.game_state_mut().hand.deselect_card(*card_idx);
            Ok(())
        }
        crate::actions::PlayingAction::MoveRight(card_idx) => {
            println!("Moving card {} right", card_idx);
            let game_state = engine.game_state_mut();
            if *card_idx < game_state.hand.len() - 1 {
                // TODO: Implement card movement
            }
            Ok(())
        }
        crate::actions::PlayingAction::MoveLeft(card_idx) => {
            println!("Moving card {} left", card_idx);
            let game_state = engine.game_state_mut();
            if *card_idx > 0 {
                // TODO: Implement card movement
            }
            Ok(())
        }
    }
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
fn process_game_over_action(_engine: &mut BalatroEngine, choice: u32) -> Result<bool, Box<dyn std::error::Error>> {
    println!("GameOver action {} selected (stub)", choice);
    // TODO: Implement actual game over actions
    match choice {
        1 => {
            println!("Starting new game...");
            // Return true to indicate we should restart
            Ok(true)
        }
        4 => {
            println!("Exiting...");
            // Return false to indicate we should exit
            Ok(false)
        }
        _ => {
            println!("Invalid game over choice: {}", choice);
            Ok(false)
        }
    }
}

