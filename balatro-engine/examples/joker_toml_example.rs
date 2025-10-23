//! Example usage of the TOML-based joker system

use balatro_engine::joker::{JokerManager, JokerInstance, JokerRarity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example TOML content for joker definitions
    let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "joker"
name = "Joker"
description = "+4 Mult"
rarity = "common"
cost = 2

[jokers.effect]
type = "scoring"
mult = 4

[[jokers]]
id = "greedy_joker"
name = "Greedy Joker"
description = "Played cards with Diamond suit give +3 Mult when scored"
rarity = "common"
cost = 5

[jokers.effect]
type = "conditional"
per_card = true

[jokers.effect.condition]
type = "suit_scored"
suit = "diamonds"

[jokers.effect.action]
type = "add_score"
mult = 3

[[jokers]]
id = "ice_cream"
name = "Ice Cream"
description = "+100 Chips, -5 Chips per hand played"
rarity = "common"
cost = 3

[jokers.effect]
type = "dynamic"

[jokers.effect.base_effect]
type = "add_score"
chips = 100

[[jokers.effect.state_modifiers]]
state_field = "hands_played"
multiplier = -5.0

[jokers.state]
persistent = true

[jokers.state.fields]
hands_played = 0

[jokers.behavior]

[jokers.behavior.on_hand_played]
type = "modify_state"
field = "hands_played"
operation = "increment"
value = 1
"#;

    // Load joker definitions from TOML
    let manager = JokerManager::from_str(toml_content)?;
    
    // Validate the configuration
    manager.validate()?;
    
    println!("Loaded {} jokers from TOML configuration", manager.get_all_ids().len());
    println!("Schema version: {}", manager.schema_version());
    
    // Get jokers by rarity
    let common_jokers = manager.get_by_rarity(JokerRarity::Common);
    println!("Found {} common jokers:", common_jokers.len());
    
    for joker_def in common_jokers {
        println!("  - {} ({}): {}", 
                 joker_def.name, 
                 joker_def.id, 
                 joker_def.description);
    }
    
    // Create joker instances
    let joker_instance = manager.create_joker("joker")?;
    println!("\nCreated joker instance: {}", joker_instance.definition.name);
    println!("Sell value: {}", joker_instance.sell_value());
    
    let greedy_joker = manager.create_joker("greedy_joker")?;
    println!("Created joker instance: {}", greedy_joker.definition.name);
    println!("Sell value: {}", greedy_joker.sell_value());
    
    let ice_cream = manager.create_joker("ice_cream")?;
    println!("Created joker instance: {}", ice_cream.definition.name);
    println!("Sell value: {}", ice_cream.sell_value());
    
    // Show state initialization
    println!("\nIce Cream initial state: {:?}", ice_cream.state);
    
    Ok(())
}
