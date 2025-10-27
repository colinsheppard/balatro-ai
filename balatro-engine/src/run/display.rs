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
    println!("\n--- PLAYING PHASE ---");
    println!("Jokers:");
    for (i, joker) in game_state.jokers.iter().enumerate() {
        println!("  {}: {:?}", i + 1, joker);
    }
    println!("Hand:");
    for (i, card) in game_state.hand.cards().iter().enumerate() {
        let selected = if game_state.hand.selected_indices().contains(&i) { " [SELECTED]" } else { "" };
        println!("  {}: {:?}{}", i + 1, card, selected);
    }
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
pub fn display_playing_actions() {
    println!("\nAvailable Actions:");
    println!("1. Select Card");
    println!("2. Deselect Card");
    println!("3. Play Hand");
    println!("4. Discard Hand");
    println!("5. View Jokers");
    println!("6. Use Consumable");
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

