//! Consumable system for Balatro game engine

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of consumables
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsumableType {
    TarotCard,
    PlanetCard,
    SpectralCard,
    Voucher,
    BoosterPack,
}

/// A consumable item that can be used during a run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consumable {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub consumable_type: ConsumableType,
    pub cost: i32,
}

impl Consumable {
    /// Create a new consumable
    pub fn new(name: String, description: String, consumable_type: ConsumableType, cost: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            consumable_type,
            cost,
        }
    }
}
