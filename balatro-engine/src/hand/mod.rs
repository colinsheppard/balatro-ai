//! Hand management system for Balatro game engine

use serde::{Deserialize, Serialize};
use crate::Deck;
use crate::card::{Card, Suit, Rank};
use crate::error::{GameError, GameResult};

/// A playing hand with advanced card management capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hand {
    cards: Vec<Card>,
    selected_indices: Vec<usize>,
}

impl Hand {
    /// Create a new empty hand
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            selected_indices: Vec::new(),
        }
    }

    /// Create a hand with the given cards
    pub fn with_cards(cards: Vec<Card>) -> Self {
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

    /// Get a card by index
    pub fn get(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }

    /// Get a mutable reference to a card by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Card> {
        self.cards.get_mut(index)
    }

    /// Add a card to the hand
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn discard_selected_cards(&mut self, deck: &mut Deck) -> GameResult<()> {
        if self.selected_indices.is_empty() {
            return Err(GameError::InvalidGameState("No cards selected to discard".to_string()));
        }
        
        // Sort indices in descending order to remove from back to front
        let mut indices_to_remove = self.selected_indices.clone();
        indices_to_remove.sort_by(|a, b| b.cmp(a));
        
        for index in indices_to_remove {
            let card = self.remove_card(index)?;
            deck.discard(card);
        }
        Ok(())
    }



    /// Remove a card by index and return it
    pub fn remove_card(&mut self, index: usize) -> GameResult<Card> {
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
    pub fn remove_selected_cards(&mut self) -> GameResult<Vec<Card>> {
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
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Get all cards as a mutable slice
    pub fn cards_mut(&mut self) -> &mut [Card] {
        &mut self.cards
    }

    /// Get all selected card indices
    pub fn selected_indices(&self) -> &[usize] {
        &self.selected_indices
    }

    /// Get selected cards
    pub fn selected_cards(&self) -> Vec<&Card> {
        self.selected_indices.iter()
            .filter_map(|&i| self.cards.get(i))
            .collect()
    }

    /// Get unselected cards in order (left to right)
    pub fn get_unselected_cards(&self) -> Vec<&Card> {
        let selected_indices_set: std::collections::HashSet<usize> = 
            self.selected_indices.iter().copied().collect();
        
        self.cards
            .iter()
            .enumerate()
            .filter(|(idx, _)| !selected_indices_set.contains(idx))
            .map(|(_, card)| card)
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
            .map(|&idx| self.cards[idx].id)
            .collect();
        
        // Sort the cards
        self.cards.sort_by(|a, b| b.rank.cmp(&a.rank));
        
        // Rebuild selected_indices based on card IDs
        self.selected_indices = self.cards
            .iter()
            .enumerate()
            .filter_map(|(idx, card)| {
                if selected_ids.contains(&card.id) {
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
            .map(|&idx| self.cards[idx].id)
            .collect();
        
        // Sort the cards
        self.cards.sort_by(|a, b| {
            match a.suit.cmp(&b.suit) {
                std::cmp::Ordering::Equal => b.rank.cmp(&a.rank),
                other => other,
            }
        });
        
        // Rebuild selected_indices based on card IDs
        self.selected_indices = self.cards
            .iter()
            .enumerate()
            .filter_map(|(idx, card)| {
                if selected_ids.contains(&card.id) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();
    }

    /// Get the total chip value of all cards in the hand
    pub fn total_chip_value(&self) -> i32 {
        self.cards.iter().map(|card| card.chip_value()).sum()
    }

    /// Get the total mult value of all cards in the hand
    pub fn total_mult_value(&self) -> f32 {
        self.cards.iter().map(|card| card.mult_value()).sum()
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        Self::with_cards(cards)
    }
}

impl Into<Vec<Card>> for Hand {
    fn into(self) -> Vec<Card> {
        self.cards
    }
}
