use balatro_engine::stakes::{StakeManager, StakeLevel, StakeModifiers};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Balatro Stakes System Demo");
    println!("==========================\n");

    // Create a stake manager with default stakes
    let manager = StakeManager::new();
    
    println!("Available Stakes:");
    println!("=================");
    
    // Display all available stakes
    for stake in manager.all_stakes() {
        println!("{} Stake ({})", stake.name, stake.color());
        println!("  Description: {}", stake.description);
        println!("  Difficulty Multiplier: {:.1}x", stake.difficulty_multiplier());
        
        // Show key modifiers
        let modifiers = &stake.modifiers;
        if modifiers.blind_score_multiplier != 1.0 {
            println!("  Blind Score Multiplier: {:.1}x", modifiers.blind_score_multiplier);
        }
        if modifiers.money_reward_multiplier != 1.0 {
            println!("  Money Reward Multiplier: {:.1}x", modifiers.money_reward_multiplier);
        }
        if modifiers.skip_cost_bonus != 0 {
            println!("  Skip Cost Bonus: +${}", modifiers.skip_cost_bonus);
        }
        if modifiers.joker_cost_multiplier != 1.0 {
            println!("  Joker Cost Multiplier: {:.1}x", modifiers.joker_cost_multiplier);
        }
        if modifiers.hands_per_round_bonus != 0 {
            println!("  Hands per Round Bonus: {}", modifiers.hands_per_round_bonus);
        }
        if modifiers.discards_per_round_bonus != 0 {
            println!("  Discards per Round Bonus: {}", modifiers.discards_per_round_bonus);
        }
        if modifiers.starting_money_modifier != 0 {
            println!("  Starting Money Modifier: {}", modifiers.starting_money_modifier);
        }
        if modifiers.starting_hand_size_modifier != 0 {
            println!("  Starting Hand Size Modifier: {}", modifiers.starting_hand_size_modifier);
        }
        println!();
    }

    // Demonstrate stake selection
    println!("Stake Selection Demo:");
    println!("====================");
    
    // Simulate selecting different stakes
    let stakes_to_demo = [StakeLevel::White, StakeLevel::Red, StakeLevel::Blue, StakeLevel::Gold];
    
    for stake_level in stakes_to_demo {
        if let Some(stake) = manager.get_stake(stake_level) {
            println!("Selected: {} Stake", stake.name);
            
            // Demonstrate how modifiers would affect game values
            let base_blind_score = 1000;
            let base_money_reward = 50;
            let base_skip_cost = 5;
            let base_joker_cost = 10;
            let base_hands_per_round = 4;
            let base_discards_per_round = 3;
            let base_starting_money = 10;
            let base_starting_hand_size = 8;
            
            println!("  Modified Values:");
            println!("    Blind Score: {} -> {}", 
                     base_blind_score, 
                     stake.modifiers.apply_to_score(base_blind_score));
            println!("    Money Reward: ${} -> ${}", 
                     base_money_reward, 
                     stake.modifiers.apply_to_money(base_money_reward));
            println!("    Skip Cost: ${} -> ${}", 
                     base_skip_cost, 
                     stake.modifiers.get_skip_cost(base_skip_cost));
            println!("    Joker Cost: ${} -> ${}", 
                     base_joker_cost, 
                     stake.modifiers.apply_to_joker_cost(base_joker_cost));
            println!("    Hands per Round: {} -> {}", 
                     base_hands_per_round, 
                     stake.modifiers.get_hands_per_round(base_hands_per_round));
            println!("    Discards per Round: {} -> {}", 
                     base_discards_per_round, 
                     stake.modifiers.get_discards_per_round(base_discards_per_round));
            println!("    Starting Money: ${} -> ${}", 
                     base_starting_money, 
                     stake.modifiers.get_starting_money(base_starting_money));
            println!("    Starting Hand Size: {} -> {}", 
                     base_starting_hand_size, 
                     stake.modifiers.get_starting_hand_size(base_starting_hand_size));
            println!();
        }
    }

    // Demonstrate edge cases
    println!("Edge Case Testing:");
    println!("=================");
    
    let extreme_modifiers = StakeModifiers {
        hands_per_round_bonus: -10,
        discards_per_round_bonus: -10,
        starting_money_modifier: -100,
        starting_hand_size_modifier: -10,
        ..Default::default()
    };
    
    println!("Testing extreme modifiers:");
    println!("  Hands per Round (base 4): {} -> {}", 
             4, extreme_modifiers.get_hands_per_round(4));
    println!("  Discards per Round (base 3): {} -> {}", 
             3, extreme_modifiers.get_discards_per_round(3));
    println!("  Starting Money (base 10): ${} -> ${}", 
             10, extreme_modifiers.get_starting_money(10));
    println!("  Starting Hand Size (base 8): {} -> {}", 
             8, extreme_modifiers.get_starting_hand_size(8));
    
    println!("\nAll stakes are available for selection - no unlocking required!");
    
    Ok(())
}
