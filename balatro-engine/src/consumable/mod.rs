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

/// A collection of consumables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consumables {
    pub consumables: Vec<Consumable>,
}

impl Consumables {
    /// Create a new empty Consumables collection
    pub fn new() -> Self {
        Self {
            consumables: Vec::new(),
        }
    }

    /// Get the number of consumables
    pub fn len(&self) -> usize {
        self.consumables.len()
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.consumables.is_empty()
    }

    /// Add a consumable to the collection
    pub fn push(&mut self, consumable: Consumable) {
        self.consumables.push(consumable);
    }

    /// Get an iterator over the consumables
    pub fn iter(&self) -> std::slice::Iter<'_, Consumable> {
        self.consumables.iter()
    }

    /// Get a mutable iterator over the consumables
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Consumable> {
        self.consumables.iter_mut()
    }
}

impl Default for Consumables {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a Consumables {
    type Item = &'a Consumable;
    type IntoIter = std::slice::Iter<'a, Consumable>;

    fn into_iter(self) -> Self::IntoIter {
        self.consumables.iter()
    }
}

impl<'a> IntoIterator for &'a mut Consumables {
    type Item = &'a mut Consumable;
    type IntoIter = std::slice::IterMut<'a, Consumable>;

    fn into_iter(self) -> Self::IntoIter {
        self.consumables.iter_mut()
    }
}
