//! Hand management system for Balatro game engine

use serde::Serialize;
use std::rc::Rc;
use std::cell::RefCell;
use crate::{Deck, SharedDeck};
use crate::card::{Card, SharedCard};
use crate::error::{GameError, GameResult};

pub type SharedHand = Rc<RefCell<Hand>>;
/// A playing hand with advanced card management capabilities
#[derive(Debug, Clone)]
#[derive(Serialize)]
pub struct Hand {
    #[serde(skip)]
    cards: Vec<SharedCard>,
    selected_indices: Vec<usize>,
}

impl Hand {
    /// Create a new empty hand
    pub fn new() -> SharedHand{
        Rc::new(RefCell::new(Self {
            cards: Vec::new(),
            selected_indices: Vec::new(),
        }))
    }

    /// Create a hand with the given cards
    pub fn with_cards(cards: Vec<Card>) -> Self {
        let shared_cards: Vec<SharedCard> = cards.into_iter()
            .map(|card| Rc::new(RefCell::new(card)))
            .collect();
        Self {
            cards: shared_cards,
            selected_indices: Vec::new(),
        }
    }

    /// Create a hand with the given shared cards
    pub fn with_shared_cards(cards: Vec<SharedCard>) -> Self {
        Self {
            cards,
            selected_indices: Vec::new(),
        }
    }

    /// Get the number of cards in the hand
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Check if the hand is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Get a shared card by index
    pub fn get(&self, index: usize) -> Option<&SharedCard> {
        self.cards.get(index)
    }

    /// Get a mutable reference to a shared card by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut SharedCard> {
        self.cards.get_mut(index)
    }

    /// Add a card to the hand
    pub fn add_card(&mut self, card: SharedCard) {
        self.cards.push(card);
    }

    pub fn discard_selected_cards(&mut self, deck: SharedDeck) -> GameResult<()> {
        if self.selected_indices.is_empty() {
            return Err(GameError::InvalidGameState("No cards selected to discard".to_string()));
        }
        
        // Sort indices in descending order to remove from back to front
        let mut indices_to_remove = self.selected_indices.clone();
        indices_to_remove.sort_by(|a, b| b.cmp(a));
        
        for index in indices_to_remove {
            let card = self.remove_card(index)?;
            deck.borrow_mut().discard(card);
        }
        Ok(())
    }

    /// Remove a card by index and return it
    pub fn remove_card(&mut self, index: usize) -> GameResult<SharedCard> {
        if index >= self.cards.len() {
            return Err(GameError::InvalidGameState(format!("Index {} out of bounds for hand of size {}", index, self.cards.len())));
        }
        
        // Remove from selected indices if present
        self.selected_indices.retain(|&i| i != index);
        
        // Adjust selected indices for cards that come after the removed card
        for selected_index in &mut self.selected_indices {
            if *selected_index > index {
                *selected_index -= 1;
            }
        }
        
        Ok(self.cards.remove(index))
    }

    /// Remove multiple cards by indices (in reverse order to maintain indices)
    pub fn remove_selected_cards(&mut self) -> GameResult<Vec<SharedCard>> {
        if self.selected_indices.is_empty() {
            return Ok(Vec::new());
        }

        // Sort indices in descending order to remove from back to front
        let mut sorted_indices = self.selected_indices.clone();
        sorted_indices.sort_by(|a, b| b.cmp(a));

        let mut removed_cards = Vec::new();
        for &index in &sorted_indices {
            removed_cards.push(self.remove_card(index)?);
        }

        // Reverse to maintain original order
        removed_cards.reverse();
        Ok(removed_cards)
    }

    /// Clear all cards from the hand
    pub fn clear(&mut self) {
        self.cards.clear();
        self.selected_indices.clear();
    }

    /// Get all cards as a slice
    pub fn cards(&self) -> &[SharedCard] {
        &self.cards
    }

    /// Get all selected card indices
    pub fn selected_indices(&self) -> &[usize] {
        &self.selected_indices
    }

    /// Get selected cards
    pub fn selected_cards(&self) -> Vec<SharedCard> {
        self.selected_indices.iter()
            .filter_map(|&i| self.cards.get(i).cloned())
            .collect()
    }

    /// Get unselected cards in order (left to right)
    pub fn get_unselected_cards(&self) -> Vec<SharedCard> {
        let selected_indices_set: std::collections::HashSet<usize> = 
            self.selected_indices.iter().copied().collect();
        
        self.cards
            .iter()
            .enumerate()
            .filter(|(idx, _)| !selected_indices_set.contains(idx))
            .map(|(_, card)| card.clone())
            .collect()
    }

    /// Check if a card at the given index is selected
    pub fn is_selected(&self, index: usize) -> bool {
        self.selected_indices.contains(&index)
    }

    /// Select a card by index
    pub fn select_card(&mut self, index: usize) -> GameResult<()> {
        if index >= self.cards.len() {
            return Err(GameError::InvalidGameState(format!("Index {} out of bounds for hand of size {}", index, self.cards.len())));
        }
        
        if !self.selected_indices.contains(&index) {
            self.selected_indices.push(index);
        }
        
        Ok(())
    }

    /// Deselect a card by index
    pub fn deselect_card(&mut self, index: usize) {
        self.selected_indices.retain(|&i| i != index);
    }

    /// Toggle selection of a card by index
    pub fn toggle_selection(&mut self, index: usize) -> GameResult<()> {
        if self.is_selected(index) {
            self.deselect_card(index);
        } else {
            self.select_card(index)?;
        }
        Ok(())
    }

    /// Clear all selections
    pub fn clear_selections(&mut self) {
        self.selected_indices.clear();
    }

    /// Move a card left by one position
    pub fn move_left(&mut self, index: usize) -> GameResult<()> {
        if index == 0 || index >= self.cards.len() {
            return Err(GameError::InvalidGameState(format!("Cannot move card at index {} left", index)));
        }

        // Update selected indices
        for selected_index in &mut self.selected_indices {
            if *selected_index == index {
                *selected_index = index.saturating_sub(1);
            } else if *selected_index == index.saturating_sub(1) {
                *selected_index = index;
            }
        }

        self.cards.swap(index, index - 1);
        Ok(())
    }

    /// Move a card right by one position
    pub fn move_right(&mut self, index: usize) -> GameResult<()> {
        if self.cards.is_empty() || index >= self.cards.len() - 1 {
            return Err(GameError::InvalidGameState(format!("Cannot move card at index {} right", index)));
        }

        // Update selected indices
        for selected_index in &mut self.selected_indices {
            if *selected_index == index {
                *selected_index = index + 1;
            } else if *selected_index == index + 1 {
                *selected_index = index;
            }
        }

        self.cards.swap(index, index + 1);
        Ok(())
    }

    /// Sort cards by rank in descending order (highest first)
    /// Preserves selected state of cards after sorting
    pub fn sort_by_rank_desc(&mut self) {
        // Track which cards are selected by their IDs (since positions will change)
        let selected_ids: std::collections::HashSet<_> = self.selected_indices
            .iter()
            .map(|&idx| self.cards[idx].borrow().id)
            .collect();
        
        // Sort the cards
        self.cards.sort_by(|a, b| b.borrow().rank.cmp(&a.borrow().rank));
        
        // Rebuild selected_indices based on card IDs
        self.selected_indices = self.cards
            .iter()
            .enumerate()
            .filter_map(|(idx, card)| {
                if selected_ids.contains(&card.borrow().id) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();
    }

    /// Sort cards by suit, then by rank (descending)
    /// Preserves selected state of cards after sorting
    pub fn sort_by_suit_then_rank(&mut self) {
        // Track which cards are selected by their IDs (since positions will change)
        let selected_ids: std::collections::HashSet<_> = self.selected_indices
            .iter()
            .map(|&idx| self.cards[idx].borrow().id)
            .collect();
        
        // Sort the cards
        self.cards.sort_by(|a, b| {
            let a_ref = a.borrow();
            let b_ref = b.borrow();
            match a_ref.suit.cmp(&b_ref.suit) {
                std::cmp::Ordering::Equal => b_ref.rank.cmp(&a_ref.rank),
                other => other,
            }
        });
        
        // Rebuild selected_indices based on card IDs
        self.selected_indices = self.cards
            .iter()
            .enumerate()
            .filter_map(|(idx, card)| {
                if selected_ids.contains(&card.borrow().id) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();
    }

    /// Get the total chip value of all cards in the hand
    pub fn total_chip_value(&self) -> i32 {
        self.cards.iter().map(|card| card.borrow().chip_value()).sum()
    }

    /// Get the total mult value of all cards in the hand
    pub fn total_mult_value(&self) -> f32 {
        self.cards.iter().map(|card| card.borrow().mult_value()).sum()
    }
}

impl Default for Hand {
    fn default() -> Hand {
        Self::new().borrow().clone()
    }
}

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        Self::with_cards(cards)
    }
}

impl From<Hand> for Vec<SharedCard> {
    fn from(hand: Hand) -> Vec<SharedCard> {
        hand.cards
    }
}
