//! Play limits management system for Balatro game engine

use serde::Serialize;
use std::rc::Rc;
use std::cell::RefCell;

pub type SharedPlayLimits = Rc<RefCell<PlayLimits>>;

/// Play limits tracking the number of hands and discards available per round
#[derive(Debug, Clone, Serialize)]
pub struct PlayLimits {
    pub n_hands: u32,
    pub n_discards: u32,
    pub n_hands_remaining: u32,
    pub n_discards_remaining: u32,
}

impl PlayLimits {
    /// Create a new PlayLimits with default values (4 hands, 3 discards)
    pub fn new() -> SharedPlayLimits {
        Rc::new(RefCell::new(Self {
            n_hands: 4,
            n_discards: 3,
            n_hands_remaining: 4,
            n_discards_remaining: 3,
        }))
    }

    /// Create a new PlayLimits with custom values
    pub fn with_limits(n_hands: u32, n_discards: u32) -> SharedPlayLimits {
        Rc::new(RefCell::new(Self {
            n_hands,
            n_discards,
            n_hands_remaining: n_hands,
            n_discards_remaining: n_discards,
        }))
    }

    /// Reset the remaining values to the base limits
    pub fn reset_remaining(&mut self) {
        self.n_hands_remaining = self.n_hands;
        self.n_discards_remaining = self.n_discards;
    }

    /// Decrement hands remaining (with bounds check)
    pub fn decrement_hands(&mut self) {
        if self.n_hands_remaining > 0 {
            self.n_hands_remaining -= 1;
        }
    }

    /// Decrement discards remaining (with bounds check)
    pub fn decrement_discards(&mut self) {
        if self.n_discards_remaining > 0 {
            self.n_discards_remaining -= 1;
        }
    }

    /// Check if any hands remain
    pub fn has_hands_remaining(&self) -> bool {
        self.n_hands_remaining > 0
    }

    /// Check if any discards remain
    pub fn has_discards_remaining(&self) -> bool {
        self.n_discards_remaining > 0
    }
}

impl Default for PlayLimits {
    fn default() -> Self {
        Self {
            n_hands: 4,
            n_discards: 3,
            n_hands_remaining: 4,
            n_discards_remaining: 3,
        }
    }
}

