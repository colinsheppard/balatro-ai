//! Comprehensive test suite for the Balatro game engine

use std::{cell::RefCell, rc::Rc};

use crate::{Card, SharedCard};

pub mod unit;
pub mod integration;
pub mod fixtures;

pub fn to_shared(cards: Vec<Card>) -> Vec<SharedCard> {
    cards.into_iter().map(|c| Rc::new(RefCell::new(c))).collect()
}