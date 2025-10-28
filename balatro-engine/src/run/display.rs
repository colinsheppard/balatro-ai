//! Display functions for game state and UI

use crate::GameState;

/// Display the current game state
pub fn display_game_state(game_state: &GameState) {
    println!("\n=== BALATRO GAME STATE ===");
    println!("Phase: {:?}", game_state.phase);
    println!("Deck: {:?}", game_state.deck.deck_type);
    println!("Stake: {:?}", game_state.stake.level);
    println!("Ante: {}", game_state.ante);
    println!("Round: {}", game_state.round_number);
    println!("Money: ${}", game_state.money);
    println!("Hand Size: {}", game_state.hand_size);
    let mut jokers_line = String::from("");
    for (i, joker) in game_state.jokers.iter().enumerate() {
        jokers_line.push_str(&format!("{} ", joker));
    }
    println!("Jokers ({}): {}", game_state.jokers.len(), jokers_line);
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

/// Display Shop phase specific state
pub fn display_shop_phase_state(game_state: &GameState) {
    println!("\n--- SHOP PHASE ---");
    println!("Available Money: ${}", game_state.money);
    println!("Current Jokers: {}", game_state.jokers.len());
    println!("Available Consumables: {}", game_state.consumables.len());
    // TODO: Display actual shop items
}

/// Display BlindSelect phase specific state
pub fn display_blind_select_phase_state(game_state: &GameState) {
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
pub fn display_playing_phase_state(game_state: &GameState) {
    if let Some(active_blind) = game_state.upcoming_blinds.get_active_blind() {
        println!("\n--- PLAYING {} ---", active_blind.name);
        println!("Score: ({} / {})", game_state.score, active_blind.required_score);
    } else {
        // Fallback if no active blind is found
        println!("\n--- PLAYING PHASE ---");
    }
    
    
    // Display hand in horizontal layout with selection info
    let selected_indices: std::collections::HashSet<usize> = game_state.hand.selected_indices().iter().copied().collect();
    let cards = game_state.hand.cards();
    
    // Collect cards by selection status, maintaining position
    let mut selected_line =   String::from("selected:   ");
    let mut unselected_line = String::from("unselected: ");
    let mut indices_line =    String::from("to select:  ");
    
    for (i, card) in cards.iter().enumerate() {
        if selected_indices.contains(&i) {
            selected_line.push_str(&format!("{:<2} ", card));
            unselected_line.push_str(&"   ");
        } else {
            selected_line.push_str(&"   ");
            unselected_line.push_str(&format!("{:<2} ", card));
        }
        indices_line.push_str(&format!("{:<2} ", i + 2));
    }
    
    println!("\nHand:");
    println!("{}", selected_line);
    println!("{}", unselected_line);
    println!("{}", indices_line);
}

/// Display RoundEnd phase specific state
pub fn display_round_end_phase_state(game_state: &GameState) {
    println!("\n--- ROUND END PHASE ---");
    println!("Round {} Complete!", game_state.round_number);
    println!("Final Score: {}", game_state.score);
    println!("Money Earned: ${}", game_state.money);
}

/// Display GameOver phase specific state
pub fn display_game_over_phase_state(game_state: &GameState) {
    println!("\n--- GAME OVER ---");
    println!("Final Score: {}", game_state.score);
    println!("Antes Reached: {}", game_state.ante);
    println!("Rounds Played: {}", game_state.round_number);
}

/// Display available Shop actions
pub fn display_shop_actions() {
    println!("\nAvailable Actions:");
    println!("1. Buy Joker");
    println!("2. Buy Consumable");
    println!("3. Sell Joker");
    println!("4. Skip Shop");
    println!("5. View Deck");
}

/// Display available BlindSelect actions
pub fn display_blind_select_actions(game_state: &GameState) {
    println!("\nAvailable Actions:");
    
    if let Some(next_blind) = game_state.upcoming_blinds.get_next_upcoming_blind() {
        println!("1. Play {} ", next_blind.name);
        
        if next_blind.can_skip() {
            println!("2. Skip {}", next_blind.name);
        }
    } else {
        println!("All blinds completed for this ante!");
        println!("1. Continue to next ante");
    }
}

/// Display available Playing actions
pub fn display_playing_actions(game_state: &crate::GameState, actions: &[(u32, crate::actions::PlayingAction)]) {
    let cards = game_state.hand.cards();
    let selected_indices: Vec<usize> = game_state.hand.selected_indices().iter().copied().collect();
    
    println!("\nAvailable Actions:");
    
    for (action_num, action) in actions {
        match action {
            crate::actions::PlayingAction::PlaySelectedCards => {
                println!("{}: {}", action_num, action);
            }
            crate::actions::PlayingAction::DiscardSelectedCards => {
                println!("{}: {}", action_num, action);
            }
            crate::actions::PlayingAction::SortByRank => {
                println!("{}: {}", action_num, action);
            }
            crate::actions::PlayingAction::SortBySuit => {
                println!("{}: {}", action_num, action);
            }
            crate::actions::PlayingAction::SelectCard(card_idx) => {
                println!("{}: Select {}", action_num, &cards[*card_idx]);
            }
            crate::actions::PlayingAction::DeselectCard(card_idx) => {
                println!("{}: Deselect {}", action_num, &cards[*card_idx]);
            }
            crate::actions::PlayingAction::MoveRight(card_idx) => {
                println!("{}: Move right {}", action_num, &cards[*card_idx]);
            }
            crate::actions::PlayingAction::MoveLeft(card_idx) => {
                println!("{}: Move left {}", action_num, &cards[*card_idx]);
            }
            crate::actions::PlayingAction::SortByRank => {
                println!("{}: Sort by rank", action_num);
            }
            crate::actions::PlayingAction::SortBySuit => {
                println!("{}: Sort by suit", action_num);
            }
        }
    }
}

/// Display available RoundEnd actions
pub fn display_round_end_actions() {
    println!("\nAvailable Actions:");
    println!("1. Continue to Shop");
    println!("2. View Statistics");
    println!("3. Save Game");
}

/// Display available GameOver actions
pub fn display_game_over_actions() {
    println!("\nAvailable Actions:");
    println!("1. Play Again");
    println!("2. Main Menu");
    println!("3. View Final Statistics");
    println!("4. Exit");
}

