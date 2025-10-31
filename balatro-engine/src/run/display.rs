//! Display functions for game state and UI

use crate::GameState;

/// Display the current game state
pub fn display_game_state(game_state: &GameState) {
    println!("\n=== BALATRO GAME STATE ===");
    println!("Phase: {:?}", game_state.phase);
    println!("Deck: {:?}", game_state.deck.borrow().deck_type);
    println!("Stake: {:?}", game_state.stake.level);
    println!("Ante: {}", game_state.ante);
    println!("Round: {}", game_state.round_number);
    println!("Money: ${}", game_state.money);
    println!("Hand Size: {}", game_state.hand_size);
    let mut jokers_line = String::from("");
    for joker in game_state.jokers.iter() {
        jokers_line.push_str(&format!("{} ", joker));
    }
    println!("Jokers ({}): {}", game_state.jokers.len(), jokers_line);
    println!("Consumables: {}", game_state.consumables.len());
    
    // Display blind statuses
    println!("Blinds Status:");
    println!("  Small: {:?} (Score: {}, Money: {})", 
             game_state.upcoming_blinds.borrow().small.borrow().status,
             game_state.upcoming_blinds.borrow().small.borrow().required_score,
             game_state.upcoming_blinds.borrow().small.borrow().reward_money);
    println!("  Big: {:?} (Score: {}, Money: {})", 
             game_state.upcoming_blinds.borrow().big.borrow().status,
             game_state.upcoming_blinds.borrow().big.borrow().required_score,
             game_state.upcoming_blinds.borrow().big.borrow().reward_money);
    println!("  Boss: {:?} (Score: {}, Money: {})", 
             game_state.upcoming_blinds.borrow().boss.borrow().status,
             game_state.upcoming_blinds.borrow().boss.borrow().required_score,
             game_state.upcoming_blinds.borrow().boss.borrow().reward_money);
    
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
    
    if let Some(next_blind) = game_state.upcoming_blinds.borrow().get_next_upcoming_blind() {
        println!("Next Blind: {}", next_blind.borrow().name);
        println!("Required Score: {}", next_blind.borrow().required_score);
        println!("Reward Money: ${}", next_blind.borrow().reward_money);
        println!("Type: {:?}", next_blind.borrow().blind_type);
        
        if let Some(boss_effect) = &next_blind.borrow().boss_effect {
            println!("Boss Effect: {:?}", boss_effect);
        }
    } else {
        println!("All blinds completed for this ante!");
    }
}

/// Display Playing phase specific state
pub fn display_playing_phase_state(game_state: &GameState) {

    if let Some(active_blind) = game_state.upcoming_blinds.borrow().get_active_blind() {
        println!("\n--- PLAYING {} ---", active_blind.borrow().name);
        println!("Score: ({} / {})", game_state.score, active_blind.borrow().required_score);
    } else {
        // Fallback if no active blind is found
        println!("\n--- PLAYING PHASE ---");
    }
    println!("Deck: ({} / {})", game_state.deck.borrow().n_remaining_cards(), game_state.deck.borrow().n_cards_full_deck());
    
    // Display hand in horizontal layout with selection info
    let hand_borrowed = game_state.hand.borrow();
    let selected_indices: std::collections::HashSet<usize> = hand_borrowed.selected_indices().iter().copied().collect();
    let cards = hand_borrowed.cards();
    
    // Collect cards by selection status, maintaining position
    let mut selected_line =   String::from("selected:   ");
    let mut unselected_line = String::from("unselected: ");
    let mut indices_line =    String::from("to select:  ");
    
    for (i, card_shared) in cards.iter().enumerate() {
        let card = card_shared.borrow();
        if selected_indices.contains(&i) {
            selected_line.push_str(&format!("{:<2} ", *card));
            unselected_line.push_str(&"   ");
        } else {
            selected_line.push_str(&"   ");
            unselected_line.push_str(&format!("{:<2} ", *card));
        }
        indices_line.push_str(&format!("{:<2} ", i + 2));
    }
    
    println!("\nHand");
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
pub fn display_shop_actions(actions: &[(u32, crate::actions::ShopAction)]) {
    println!("\nAvailable Actions:");
    
    for (action_num, action) in actions {
        println!("{}: {}", action_num, action);
    }
}

/// Display available BlindSelect actions
pub fn display_blind_select_actions(game_state: &GameState) {
    println!("\nAvailable Actions:");
    
    if let Some(next_blind) = game_state.upcoming_blinds.borrow().get_next_upcoming_blind() {
        println!("1. Play {} ", next_blind.borrow().name);
        
        if next_blind.borrow().can_skip() {
            println!("2. Skip {}", next_blind.borrow().name);
        }
    } else {
        println!("All blinds completed for this ante!");
        println!("1. Continue to next ante");
    }
}

/// Display available Playing actions
pub fn display_playing_actions(game_state: &crate::GameState, actions: &[(u32, crate::actions::PlayingAction)]) {
    let binding = game_state.hand.borrow();
    let cards = binding.cards();
    
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
                println!("{}: Sort by rank", action_num);
            }
            crate::actions::PlayingAction::SortBySuit => {
                println!("{}: Sort by suit", action_num);
            }
            crate::actions::PlayingAction::SelectCard(card_idx) => {
                println!("{}: Select {}", action_num, *cards[*card_idx].borrow());
            }
            crate::actions::PlayingAction::DeselectCard(card_idx) => {
                println!("{}: Deselect {}", action_num, *cards[*card_idx].borrow());
            }
            crate::actions::PlayingAction::MoveRight(card_idx) => {
                println!("{}: Move right {}", action_num, *cards[*card_idx].borrow());
            }
            crate::actions::PlayingAction::MoveLeft(card_idx) => {
                println!("{}: Move left {}", action_num, *cards[*card_idx].borrow());
            }
        }
    }
}

/// Display available RoundEnd actions
pub fn display_round_end_actions(actions: &[(u32, crate::actions::RoundEndAction)]) {
    println!("\nAvailable Actions:");
    
    for (action_num, action) in actions {
        println!("{}: {}", action_num, action);
    }
}

/// Display available GameOver actions
pub fn display_game_over_actions() {
    println!("\nAvailable Actions:");
    println!("1. Play Again");
    println!("2. Main Menu");
    println!("3. View Final Statistics");
    println!("4. Exit");
}

