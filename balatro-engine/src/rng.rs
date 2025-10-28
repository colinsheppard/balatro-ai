//! Random number generation manager for Balatro game engine

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub type SharedRngManager = Rc<RefCell<GameRngManager>>;
/// Manages RNG instances for deterministic, seeded random number generation
#[derive(Debug, Clone)]
pub struct GameRngManager {
    pub seed: u64,
    // HashMap for storing specialized RNG instances by name
    rngs: HashMap<String, StdRng>,
}

impl GameRngManager {
    /// Create a new RNG manager with the given seed
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rngs: HashMap::new(),
        }
    }

    /// Get or create a named RNG instance
    pub fn get_rng(&mut self, name: &str) -> &mut StdRng {
        if !self.rngs.contains_key(name) {
            // Hash the name to create a deterministic seed offset
            let mut hasher = DefaultHasher::new();
            name.hash(&mut hasher);
            let name_hash = hasher.finish();
            
            // Create a new RNG from the seed combined with the name hash
            let rng: StdRng = StdRng::seed_from_u64(self.seed.wrapping_add(name_hash));
            self.rngs.insert(name.to_string(), rng);
        }
        self.rngs.get_mut(name).unwrap()
    }


}

