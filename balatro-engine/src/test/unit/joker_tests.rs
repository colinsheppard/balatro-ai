//! Unit tests for the TOML-based joker system

use crate::joker::{JokerManager, JokerRarity, JokerEdition, JokerSticker};
use crate::card::{Card, Suit, Rank};

#[test]
fn test_joker_manager_from_str() {
    let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "test_joker"
name = "Test Joker"
description = "A test joker"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4
"#;

    let manager = JokerManager::from_str(toml_content).unwrap();
    assert_eq!(manager.schema_version(), "1.0.0");
    assert_eq!(manager.get_all_ids(), vec!["test_joker"]);
    
    let definition = manager.get_definition("test_joker").unwrap();
    assert_eq!(definition.name, "Test Joker");
    assert_eq!(definition.rarity, JokerRarity::Common);
    assert_eq!(definition.cost, 3);
}

#[test]
fn test_joker_instance_creation() {
    let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "test_joker"
name = "Test Joker"
description = "A test joker"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4
"#;

    let manager = JokerManager::from_str(toml_content).unwrap();
    let joker = manager.create_joker("test_joker").unwrap();
    
    assert_eq!(joker.definition.name, "Test Joker");
    assert_eq!(joker.sell_value(), 3);
}

#[test]
fn test_joker_instance_stickers() {
    let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "test_joker"
name = "Test Joker"
description = "A test joker"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4
"#;

    let manager = JokerManager::from_str(toml_content).unwrap();
    let mut joker = manager.create_joker("test_joker").unwrap();
    
    joker.stickers.push(JokerSticker::Eternal);
    assert!(joker.has_sticker(JokerSticker::Eternal));
    assert!(!joker.has_sticker(JokerSticker::Perishable));
}

#[test]
fn test_joker_rarity_sell_values() {
    let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "common_joker"
name = "Common Joker"
description = "A common joker"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4

[[jokers]]
id = "uncommon_joker"
name = "Uncommon Joker"
description = "An uncommon joker"
rarity = "uncommon"
cost = 5

[jokers.effect]
type = "scoring"
mult = 8

[[jokers]]
id = "rare_joker"
name = "Rare Joker"
description = "A rare joker"
rarity = "rare"
cost = 7

[jokers.effect]
type = "scoring"
mult = 12

[[jokers]]
id = "legendary_joker"
name = "Legendary Joker"
description = "A legendary joker"
rarity = "legendary"
cost = 10

[jokers.effect]
type = "scoring"
mult = 20
"#;

    let manager = JokerManager::from_str(toml_content).unwrap();
    
    let common = manager.create_joker("common_joker").unwrap();
    let uncommon = manager.create_joker("uncommon_joker").unwrap();
    let rare = manager.create_joker("rare_joker").unwrap();
    let legendary = manager.create_joker("legendary_joker").unwrap();
    
    assert_eq!(common.sell_value(), 3);
    assert_eq!(uncommon.sell_value(), 4);
    assert_eq!(rare.sell_value(), 5);
    assert_eq!(legendary.sell_value(), 6);
}

#[test]
fn test_joker_rarity_ordering() {
    assert!(JokerRarity::Common < JokerRarity::Uncommon);
    assert!(JokerRarity::Uncommon < JokerRarity::Rare);
    assert!(JokerRarity::Rare < JokerRarity::Legendary);
}

#[test]
fn test_joker_editions() {
    let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "test_joker"
name = "Test Joker"
description = "A test joker"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4
"#;

    let manager = JokerManager::from_str(toml_content).unwrap();
    let mut joker = manager.create_joker("test_joker").unwrap();
    
    let editions = [
        JokerEdition::Base,
        JokerEdition::Foil,
        JokerEdition::Holographic,
        JokerEdition::Polychrome,
        JokerEdition::Negative,
    ];
    
    for edition in editions {
        joker.edition = edition.clone();
        assert_eq!(joker.edition, edition);
    }
}

#[test]
fn test_joker_stickers() {
    let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "test_joker"
name = "Test Joker"
description = "A test joker"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4
"#;

    let manager = JokerManager::from_str(toml_content).unwrap();
    let mut joker = manager.create_joker("test_joker").unwrap();
    
    let stickers = [
        JokerSticker::Eternal,
        JokerSticker::Perishable,
        JokerSticker::Rental,
    ];
    
    for sticker in stickers {
        joker.stickers.push(sticker.clone());
        assert!(joker.has_sticker(sticker));
    }
}

#[test]
fn test_joker_multiple_stickers() {
    let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "test_joker"
name = "Test Joker"
description = "A test joker"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4
"#;

    let manager = JokerManager::from_str(toml_content).unwrap();
    let mut joker = manager.create_joker("test_joker").unwrap();
    
    joker.stickers.push(JokerSticker::Eternal);
    joker.stickers.push(JokerSticker::Rental);
    
    assert!(joker.has_sticker(JokerSticker::Eternal));
    assert!(joker.has_sticker(JokerSticker::Rental));
    assert!(!joker.has_sticker(JokerSticker::Perishable));
}

#[test]
fn test_joker_apply_effects() {
    let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "test_joker"
name = "Test Joker"
description = "A test joker"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4
"#;

    let manager = JokerManager::from_str(toml_content).unwrap();
    let joker = manager.create_joker("test_joker").unwrap();
    
    let ace = Card::new(Suit::Hearts, Rank::Ace);
    let king = Card::new(Suit::Spades, Rank::King);

    let cards = vec![
       &ace,
       &king,
    ];

    let (chip_mod, mult_mod) = joker.apply_effects(&cards).unwrap();
    assert_eq!(chip_mod, 0);
    assert_eq!(mult_mod, 1.0);
}

#[test]
fn test_joker_manager_get_by_rarity() {
    let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "common_joker"
name = "Common Joker"
description = "A common joker"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4

[[jokers]]
id = "uncommon_joker"
name = "Uncommon Joker"
description = "An uncommon joker"
rarity = "uncommon"
cost = 5

[jokers.effect]
type = "scoring"
mult = 8
"#;

    let manager = JokerManager::from_str(toml_content).unwrap();
    let common_jokers = manager.get_by_rarity(JokerRarity::Common);
    let uncommon_jokers = manager.get_by_rarity(JokerRarity::Uncommon);
    
    assert_eq!(common_jokers.len(), 1);
    assert_eq!(uncommon_jokers.len(), 1);
    assert_eq!(common_jokers[0].id, "common_joker");
    assert_eq!(uncommon_jokers[0].id, "uncommon_joker");
}

#[test]
fn test_joker_manager_validation() {
    let toml_content = r#"
schema_version = "1.0.0"

[[jokers]]
id = "test_joker"
name = "Test Joker"
description = "A test joker"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4
"#;

    let manager = JokerManager::from_str(toml_content).unwrap();
    assert!(manager.validate().is_ok());
}

#[test]
fn test_conditional_joker_effect() {
    let toml_content = r#"
schema_version = "1.0.0"

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
"#;

    let manager = JokerManager::from_str(toml_content).unwrap();
    let joker = manager.create_joker("greedy_joker").unwrap();
    
    assert_eq!(joker.definition.name, "Greedy Joker");
    assert_eq!(joker.definition.effect.effect_type, crate::joker::JokerEffectType::Conditional);
    assert!(joker.definition.effect.per_card);
}

#[test]
fn test_dynamic_joker_effect() {
    let toml_content = r#"
schema_version = "1.0.0"

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

    let manager = JokerManager::from_str(toml_content).unwrap();
    let joker = manager.create_joker("ice_cream").unwrap();
    
    assert_eq!(joker.definition.name, "Ice Cream");
    assert_eq!(joker.definition.effect.effect_type, crate::joker::JokerEffectType::Dynamic);
    assert!(joker.definition.state.is_some());
    assert!(joker.definition.behavior.is_some());
    
    // Check initial state
    assert_eq!(joker.state.get("hands_played").unwrap().as_i64().unwrap(), 0);
}
