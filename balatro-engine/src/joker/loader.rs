//! Joker loader and manager for TOML-based configuration

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::error::{GameError, GameResult};
use crate::joker::config::{JokerConfig, JokerDefinition, JokerInstance, JokerRarity};

/// Manager for loading and managing joker definitions
pub struct JokerManager {
    definitions: HashMap<String, JokerDefinition>,
    config: JokerConfig,
}

impl JokerManager {
    /// Load joker definitions from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> GameResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| GameError::IoError(e))?;
        
        let config: JokerConfig = toml::from_str(&content)
            .map_err(|e| GameError::InvalidJokerOperation(format!("TOML parsing error: {}", e)))?;
        
        let mut definitions = HashMap::new();
        for joker in &config.jokers {
            definitions.insert(joker.id.clone(), joker.clone());
        }
        
        Ok(Self {
            definitions,
            config,
        })
    }

    /// Load joker definitions from a string
    pub fn from_str(content: &str) -> GameResult<Self> {
        let config: JokerConfig = toml::from_str(content)
            .map_err(|e| GameError::InvalidJokerOperation(format!("TOML parsing error: {}", e)))?;
        
        let mut definitions = HashMap::new();
        for joker in &config.jokers {
            definitions.insert(joker.id.clone(), joker.clone());
        }
        
        Ok(Self {
            definitions,
            config,
        })
    }

    /// Get a joker definition by ID
    pub fn get_definition(&self, id: &str) -> Option<&JokerDefinition> {
        self.definitions.get(id)
    }

    /// Create a joker instance from a definition ID
    pub fn create_joker(&self, id: &str) -> GameResult<JokerInstance> {
        let definition = self.definitions.get(id)
            .ok_or_else(|| GameError::InvalidJokerOperation(format!("Joker '{}' not found", id)))?;
        
        Ok(JokerInstance::new(definition.clone()))
    }

    /// Get all joker definitions
    pub fn all_definitions(&self) -> &HashMap<String, JokerDefinition> {
        &self.definitions
    }

    /// Get jokers by rarity
    pub fn get_by_rarity(&self, rarity: JokerRarity) -> Vec<&JokerDefinition> {
        self.definitions.values()
            .filter(|joker| joker.rarity == rarity)
            .collect()
    }

    /// Get all joker IDs
    pub fn get_all_ids(&self) -> Vec<String> {
        self.definitions.keys().cloned().collect()
    }

    /// Get jokers by cost range
    pub fn get_by_cost_range(&self, min_cost: i32, max_cost: i32) -> Vec<&JokerDefinition> {
        self.definitions.values()
            .filter(|joker| joker.cost >= min_cost && joker.cost <= max_cost)
            .collect()
    }

    /// Get the schema version
    pub fn schema_version(&self) -> &str {
        &self.config.schema_version
    }

    /// Validate the configuration
    pub fn validate(&self) -> GameResult<()> {
        // Check for duplicate IDs
        let mut seen_ids = std::collections::HashSet::new();
        for joker in &self.config.jokers {
            if !seen_ids.insert(&joker.id) {
                return Err(GameError::InvalidJokerOperation(
                    format!("Duplicate joker ID: {}", joker.id)
                ));
            }
        }

        // Validate each joker definition
        for joker in &self.config.jokers {
            self.validate_joker_definition(joker)?;
        }

        Ok(())
    }

    /// Validate a single joker definition
    fn validate_joker_definition(&self, joker: &JokerDefinition) -> GameResult<()> {
        // Check that ID is not empty
        if joker.id.is_empty() {
            return Err(GameError::InvalidJokerOperation(
                "Joker ID cannot be empty".to_string()
            ));
        }

        // Check that name is not empty
        if joker.name.is_empty() {
            return Err(GameError::InvalidJokerOperation(
                format!("Joker '{}' name cannot be empty", joker.id)
            ));
        }

        // Check that cost is non-negative
        if joker.cost < 0 {
            return Err(GameError::InvalidJokerOperation(
                format!("Joker '{}' cost cannot be negative", joker.id)
            ));
        }

        // Validate effect configuration
        self.validate_joker_effect(&joker.effect, &joker.id)?;

        Ok(())
    }

    /// Validate a joker effect
    fn validate_joker_effect(&self, effect: &crate::joker::config::JokerEffect, joker_id: &str) -> GameResult<()> {
        match effect.effect_type {
            crate::joker::config::JokerEffectType::Scoring => {
                // Scoring effects should have mult or chips
                if effect.mult.is_none() && effect.chips.is_none() {
                    return Err(GameError::InvalidJokerOperation(
                        format!("Joker '{}' scoring effect must have mult or chips", joker_id)
                    ));
                }
            },
            crate::joker::config::JokerEffectType::Conditional => {
                // Conditional effects should have condition and action
                if effect.condition.is_none() || effect.action.is_none() {
                    return Err(GameError::InvalidJokerOperation(
                        format!("Joker '{}' conditional effect must have condition and action", joker_id)
                    ));
                }
            },
            crate::joker::config::JokerEffectType::Dynamic => {
                // Dynamic effects should have base_effect and state_modifiers
                if effect.base_effect.is_none() && effect.state_modifiers.is_empty() {
                    return Err(GameError::InvalidJokerOperation(
                        format!("Joker '{}' dynamic effect must have base_effect or state_modifiers", joker_id)
                    ));
                }
            },
            crate::joker::config::JokerEffectType::Calculate => {
                // Calculate effects should have formula and result_type
                if effect.formula.is_none() || effect.result_type.is_none() {
                    return Err(GameError::InvalidJokerOperation(
                        format!("Joker '{}' calculate effect must have formula and result_type", joker_id)
                    ));
                }
            },
            crate::joker::config::JokerEffectType::Special => {
                // Special effects should have special_type
                if effect.special_type.is_none() {
                    return Err(GameError::InvalidJokerOperation(
                        format!("Joker '{}' special effect must have special_type", joker_id)
                    ));
                }
            },
        }

        Ok(())
    }
}

/// Default joker manager with built-in definitions
impl Default for JokerManager {
    fn default() -> Self {
        // For now, return an empty manager
        // In the future, this could load from embedded TOML or provide default jokers
        Self {
            definitions: HashMap::new(),
            config: crate::joker::config::JokerConfig {
                schema_version: "1.0.0".to_string(),
                jokers: Vec::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker::config::JokerRarity;

    #[test]
    fn test_load_from_str() {
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
    fn test_create_joker_instance() {
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
    fn test_get_by_rarity() {
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
    fn test_validation() {
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
    fn test_validation_duplicate_id() {
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

[[jokers]]
id = "test_joker"
name = "Another Test Joker"
description = "Another test joker"
rarity = "common"
cost = 3

[jokers.effect]
type = "scoring"
mult = 4
"#;

        let manager = JokerManager::from_str(toml_content).unwrap();
        assert!(manager.validate().is_err());
    }
}
