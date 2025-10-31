//! Phase handling for the game

use crate::{BalatroEngine, GamePhase, BlindStatus};
use crate::run::display::*;
use crate::run::user_input::get_user_input;

/// Handle the Shop phase
pub fn handle_shop_phase(engine: &mut BalatroEngine) -> Result<(), Box<dyn std::error::Error>> {
    display_shop_phase_state(engine.game_state());
    let shop_actions = crate::actions::helpers::create_shop_actions();
    display_shop_actions(&shop_actions);
    let choice = get_user_input()?;
    process_shop_action(engine, &shop_actions, choice)?;
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
    let round_end_actions = crate::actions::helpers::create_round_end_actions();
    display_round_end_actions(&round_end_actions);
    let choice = get_user_input()?;
    process_round_end_action(engine, &round_end_actions, choice)?;
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

/// Process Shop action
fn process_shop_action(engine: &mut BalatroEngine, shop_actions: &[(u32, crate::actions::ShopAction)], choice: u32) -> Result<(), Box<dyn std::error::Error>> {
    let (_action_num, action) = shop_actions.get(choice as usize)
        .ok_or_else(|| format!("Invalid choice: {}", choice))?;
    
    match action {
        crate::actions::ShopAction::NextRound => {
            println!("Moving to next round...");
            engine.game_state_mut().phase = GamePhase::BlindSelect;
            Ok(())
        }
    }
}

/// Process BlindSelect action
fn process_blind_select_action(engine: &mut BalatroEngine, choice: u32) -> Result<(), Box<dyn std::error::Error>> {
    let game_state = engine.game_state_mut();
    // Limit the immutable borrow of `upcoming_blinds` to this block
    let next_blind_opt = {
        game_state.upcoming_blinds.borrow().get_next_upcoming_blind()
    };

    if let Some(next_blind) = next_blind_opt {
        match choice {
            1 => {
                // Play the blind
                println!("Playing {}...", next_blind.borrow().name);
                next_blind.borrow_mut().set_status(BlindStatus::Active);
                game_state.phase = GamePhase::Playing;
                game_state.deck.borrow_mut().shuffle();
                game_state.clear_and_draw_hand()?;
                // Reset play limits when entering playing phase
                game_state.play_limits.borrow_mut().reset_remaining();
            }
            2 => {
                // Skip the blind (only available for Small/Big blinds)
                if next_blind.borrow().can_skip() {
                    println!("Skipping {}...", next_blind.borrow().name);
                    // Scope this borrow too so it drops before further mutations
                    let blind_mut_opt = {
                        game_state.upcoming_blinds.borrow().get_next_upcoming_blind()
                    };
                    if let Some(blind_mut) = blind_mut_opt {
                        blind_mut.borrow_mut().status = BlindStatus::Skipped;
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
            let game_state = engine.game_state_mut();
            
            // Check if hands remain
            if !game_state.play_limits.borrow().has_hands_remaining() {
                println!("No hands remaining! Game Over.");
                game_state.phase = GamePhase::GameOver;
                return Ok(());
            }
            
            let _score = game_state.play_hand().unwrap_or(0);
            
            // Decrement hands remaining after playing
            game_state.play_limits.borrow_mut().decrement_hands();
            
            // Check if we've beaten the blind
            if _score >= game_state.get_current_blind().unwrap().borrow().required_score { 
                game_state.phase = GamePhase::RoundEnd;
            } else if !game_state.play_limits.borrow().has_hands_remaining() {
                // No hands remaining, transition to game over after scoring
                game_state.phase = GamePhase::GameOver;
            } else {
                game_state.draw_hand().unwrap();
            }
            Ok(())
        }
        crate::actions::PlayingAction::DiscardSelectedCards => {
            println!("Discarding selected cards...");
            let game_state = engine.game_state_mut();
            
            // Check if discards remain (shouldn't happen if action filtering works correctly)
            if !game_state.play_limits.borrow().has_discards_remaining() {
                println!("No discards remaining! This action should not be available.");
                return Ok(());
            }
            
            game_state.hand.borrow_mut().discard_selected_cards(game_state.deck.clone()).unwrap();
            game_state.draw_hand().unwrap();
            
            // Decrement discards remaining after discarding
            game_state.play_limits.borrow_mut().decrement_discards();
            
            Ok(())
        }
        crate::actions::PlayingAction::SelectCard(card_idx) => {
            println!("Selecting card at index {}", card_idx);
            engine.game_state_mut().hand.borrow_mut().select_card(*card_idx)?;
            Ok(())
        }
        crate::actions::PlayingAction::DeselectCard(card_idx) => {
            println!("Deselecting card at index {}", card_idx);
            engine.game_state_mut().hand.borrow_mut().deselect_card(*card_idx);
            Ok(())
        }
        crate::actions::PlayingAction::MoveRight(card_idx) => {
            println!("Moving card {} right", card_idx);
            engine.game_state_mut().hand.borrow_mut().move_right(*card_idx)?;
            Ok(())
        }
        crate::actions::PlayingAction::MoveLeft(card_idx) => {
            println!("Moving card {} left", card_idx);
            engine.game_state_mut().hand.borrow_mut().move_left(*card_idx)?;
            Ok(())
        }
        crate::actions::PlayingAction::SortByRank => {
            println!("Sorting hand by rank...");
            engine.game_state_mut().hand.borrow_mut().sort_by_rank_desc();
            Ok(())
        }
        crate::actions::PlayingAction::SortBySuit => {
            println!("Sorting hand by suit...");
            engine.game_state_mut().hand.borrow_mut().sort_by_suit_then_rank();
            Ok(())
        }
    }
}

/// Process RoundEnd action
fn process_round_end_action(engine: &mut BalatroEngine, round_end_actions: &[(u32, crate::actions::RoundEndAction)], choice: u32) -> Result<(), Box<dyn std::error::Error>> {
    let (_action_num, action) = round_end_actions.get(choice as usize)
        .ok_or_else(|| format!("Invalid choice: {}", choice))?;
    
    match action {
        crate::actions::RoundEndAction::CashOut => {
            println!("Cashing out...");
            engine.game_state_mut().phase = GamePhase::Shop;
            Ok(())
        }
    }
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

