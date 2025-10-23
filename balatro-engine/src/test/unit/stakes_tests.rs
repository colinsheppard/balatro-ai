//! Unit tests for the stakes system

use crate::stakes::{Stake, StakeLevel, StakeModifiers, StakeManager};

#[test]
fn test_stake_level_ordering() {
    assert!(StakeLevel::White < StakeLevel::Red);
    assert!(StakeLevel::Red < StakeLevel::Green);
    assert!(StakeLevel::Green < StakeLevel::Blue);
    assert!(StakeLevel::Blue < StakeLevel::Black);
    assert!(StakeLevel::Black < StakeLevel::Purple);
    assert!(StakeLevel::Purple < StakeLevel::Orange);
    assert!(StakeLevel::Orange < StakeLevel::Gold);
}

#[test]
fn test_stake_creation() {
    let stake = Stake::new(
        StakeLevel::Red,
        "Red Stake".to_string(),
        "A challenging stake".to_string(),
    );
    
    assert_eq!(stake.level, StakeLevel::Red);
    assert_eq!(stake.name, "Red Stake");
    assert_eq!(stake.description, "A challenging stake");
    assert_eq!(stake.modifiers.blind_score_multiplier, 1.0);
}

#[test]
fn test_stake_with_modifiers() {
    let modifiers = StakeModifiers {
        blind_score_multiplier: 1.5,
        money_reward_multiplier: 0.8,
        skip_cost_bonus: 5,
        ..Default::default()
    };
    
    let stake = Stake::with_modifiers(
        StakeLevel::Blue,
        "Blue Stake".to_string(),
        "Harder blinds".to_string(),
        modifiers.clone(),
    );
    
    assert_eq!(stake.level, StakeLevel::Blue);
    assert_eq!(stake.modifiers.blind_score_multiplier, 1.5);
    assert_eq!(stake.modifiers.money_reward_multiplier, 0.8);
    assert_eq!(stake.modifiers.skip_cost_bonus, 5);
}

#[test]
fn test_stake_colors() {
    let manager = StakeManager::new();
    
    let white_stake = manager.get_stake(StakeLevel::White).unwrap();
    let red_stake = manager.get_stake(StakeLevel::Red).unwrap();
    let green_stake = manager.get_stake(StakeLevel::Green).unwrap();
    let blue_stake = manager.get_stake(StakeLevel::Blue).unwrap();
    let black_stake = manager.get_stake(StakeLevel::Black).unwrap();
    let purple_stake = manager.get_stake(StakeLevel::Purple).unwrap();
    let orange_stake = manager.get_stake(StakeLevel::Orange).unwrap();
    let gold_stake = manager.get_stake(StakeLevel::Gold).unwrap();
    
    assert_eq!(white_stake.color(), "white");
    assert_eq!(red_stake.color(), "red");
    assert_eq!(green_stake.color(), "green");
    assert_eq!(blue_stake.color(), "blue");
    assert_eq!(black_stake.color(), "black");
    assert_eq!(purple_stake.color(), "purple");
    assert_eq!(orange_stake.color(), "orange");
    assert_eq!(gold_stake.color(), "gold");
}

#[test]
fn test_stake_difficulty_multipliers() {
    let manager = StakeManager::new();
    
    let white_stake = manager.get_stake(StakeLevel::White).unwrap();
    let red_stake = manager.get_stake(StakeLevel::Red).unwrap();
    let gold_stake = manager.get_stake(StakeLevel::Gold).unwrap();
    
    assert_eq!(white_stake.difficulty_multiplier(), 1.0);
    assert_eq!(red_stake.difficulty_multiplier(), 1.1);
    assert_eq!(gold_stake.difficulty_multiplier(), 1.7);
}

#[test]
fn test_stake_manager_creation() {
    let manager = StakeManager::new();
    let stakes = manager.all_stakes();
    
    assert_eq!(stakes.len(), 8);
    assert!(manager.get_stake(StakeLevel::White).is_some());
    assert!(manager.get_stake(StakeLevel::Gold).is_some());
}

#[test]
fn test_stake_manager_get_all() {
    let manager = StakeManager::new();
    let stakes = manager.all_stakes();
    
    // All stakes should be available
    assert_eq!(stakes.len(), 8);
    
    // Check that all stake levels are present
    let levels: Vec<StakeLevel> = stakes.iter().map(|s| s.level).collect();
    assert!(levels.contains(&StakeLevel::White));
    assert!(levels.contains(&StakeLevel::Red));
    assert!(levels.contains(&StakeLevel::Green));
    assert!(levels.contains(&StakeLevel::Blue));
    assert!(levels.contains(&StakeLevel::Black));
    assert!(levels.contains(&StakeLevel::Purple));
    assert!(levels.contains(&StakeLevel::Orange));
    assert!(levels.contains(&StakeLevel::Gold));
}

#[test]
fn test_stake_modifiers_default() {
    let modifiers = StakeModifiers::default();
    
    assert_eq!(modifiers.blind_score_multiplier, 1.0);
    assert_eq!(modifiers.money_reward_multiplier, 1.0);
    assert_eq!(modifiers.skip_cost_bonus, 0);
    assert_eq!(modifiers.joker_cost_multiplier, 1.0);
    assert_eq!(modifiers.consumable_cost_multiplier, 1.0);
    assert_eq!(modifiers.hands_per_round_bonus, 0);
    assert_eq!(modifiers.discards_per_round_bonus, 0);
    assert_eq!(modifiers.starting_money_modifier, 0);
    assert_eq!(modifiers.starting_hand_size_modifier, 0);
}

#[test]
fn test_stake_modifiers_application() {
    let modifiers = StakeModifiers {
        blind_score_multiplier: 1.2,
        money_reward_multiplier: 0.8,
        skip_cost_bonus: 3,
        joker_cost_multiplier: 1.5,
        consumable_cost_multiplier: 1.25,
        hands_per_round_bonus: -1,
        discards_per_round_bonus: 2,
        starting_money_modifier: -2,
        starting_hand_size_modifier: -1,
    };
    
    assert_eq!(modifiers.apply_to_score(100), 120);
    assert_eq!(modifiers.apply_to_money(50), 40);
    assert_eq!(modifiers.get_skip_cost(5), 8);
    assert_eq!(modifiers.apply_to_joker_cost(10), 15);
    assert_eq!(modifiers.apply_to_consumable_cost(8), 10);
    assert_eq!(modifiers.get_hands_per_round(4), 3);
    assert_eq!(modifiers.get_discards_per_round(3), 5);
    assert_eq!(modifiers.get_starting_money(10), 8);
    assert_eq!(modifiers.get_starting_hand_size(8), 7);
}

#[test]
fn test_stake_modifiers_edge_cases() {
    let modifiers = StakeModifiers {
        hands_per_round_bonus: -10,
        discards_per_round_bonus: -10,
        starting_money_modifier: -100,
        starting_hand_size_modifier: -10,
        ..Default::default()
    };
    
    // Should enforce minimums
    assert_eq!(modifiers.get_hands_per_round(3), 1);
    assert_eq!(modifiers.get_discards_per_round(5), 0);
    assert_eq!(modifiers.get_starting_money(10), 0);
    assert_eq!(modifiers.get_starting_hand_size(8), 1);
}

#[test]
fn test_default_stakes_configuration() {
    let manager = StakeManager::new();
    
    // Test specific stake configurations
    let white_stake = manager.get_stake(StakeLevel::White).unwrap();
    assert_eq!(white_stake.modifiers.blind_score_multiplier, 1.0);
    
    let red_stake = manager.get_stake(StakeLevel::Red).unwrap();
    assert_eq!(red_stake.modifiers.blind_score_multiplier, 1.1);
    
    let green_stake = manager.get_stake(StakeLevel::Green).unwrap();
    assert_eq!(green_stake.modifiers.skip_cost_bonus, 2);
    
    let blue_stake = manager.get_stake(StakeLevel::Blue).unwrap();
    assert_eq!(blue_stake.modifiers.blind_score_multiplier, 1.2);
    assert_eq!(blue_stake.modifiers.skip_cost_bonus, 3);
    
    let black_stake = manager.get_stake(StakeLevel::Black).unwrap();
    assert_eq!(black_stake.modifiers.money_reward_multiplier, 0.75);
    
    let purple_stake = manager.get_stake(StakeLevel::Purple).unwrap();
    assert_eq!(purple_stake.modifiers.joker_cost_multiplier, 1.5);
    
    let orange_stake = manager.get_stake(StakeLevel::Orange).unwrap();
    assert_eq!(orange_stake.modifiers.hands_per_round_bonus, -1);
    
    let gold_stake = manager.get_stake(StakeLevel::Gold).unwrap();
    assert_eq!(gold_stake.modifiers.blind_score_multiplier, 1.3);
    assert_eq!(gold_stake.modifiers.money_reward_multiplier, 0.75);
    assert_eq!(gold_stake.modifiers.skip_cost_bonus, 5);
    assert_eq!(gold_stake.modifiers.joker_cost_multiplier, 1.5);
    assert_eq!(gold_stake.modifiers.consumable_cost_multiplier, 1.25);
    assert_eq!(gold_stake.modifiers.hands_per_round_bonus, -1);
    assert_eq!(gold_stake.modifiers.discards_per_round_bonus, -1);
    assert_eq!(gold_stake.modifiers.starting_money_modifier, -2);
    assert_eq!(gold_stake.modifiers.starting_hand_size_modifier, -1);
}
