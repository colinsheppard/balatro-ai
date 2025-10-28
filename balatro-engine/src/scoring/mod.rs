//! Scoring system for Balatro game engine
//! 
//! This module handles the four-phase scoring system for calculating hand scores.

use crate::GameError;
use crate::card::{Card, Enhancement, Edition, Seal};
use crate::game::{self, GameState};
use crate::joker::config::{JokerCondition, ActionType};
use crate::error::GameResult;

/// Hand score tracking
#[derive(Debug, Clone, Copy)]
pub struct HandScore {
    pub chip_score: i32,
    pub mult_score: f32,
}

impl HandScore {
    pub fn new() -> Self {
        Self {
            chip_score: 0,
            mult_score: 1.0,
        }
    }

    /// Get the final score by multiplying chips by mult
    pub fn final_score(&self) -> f32 {
        self.chip_score as f32 * self.mult_score
    }
}

impl Default for HandScore {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate the score for a played hand
/// Detailed explanation of the scoring process is here:
/// https://www.reddit.com/r/balatro/comments/1blbexa/detailed_break_down_of_balatro_scoring_system_and/
pub fn calculate_hand_score(game_state: &mut GameState) -> GameResult<HandScore> {
    if game_state.hand.selected_indices().is_empty() {
        return Err(crate::error::GameError::InvalidGameState("Cannot play empty hand".to_string()));
    }

    // Phase 1: Pre-scoring
    apply_pre_scoring(game_state)?;

    // Phase 2: Played Hand Scoring
    let mut hand_score = apply_played_hand_scoring(game_state)?;

    // Phase 3: Effects in Hand
    hand_score = apply_effects_in_hand(game_state, hand_score)?;

    // Phase 4: Joker Scoring
    hand_score = apply_joker_scoring(game_state, hand_score)?;

    Ok(hand_score)
}

/// Phase 1: Pre-scoring
/// Apply pre-scoring effects before the hand is evaluated
fn apply_pre_scoring(game_state: &mut GameState) -> GameResult<()> {
    // Iterate through each joker and apply pre-scoring effects
    // We need to use unsafe here because we're borrowing game_state mutably
    // while also accessing game_state.jokers immutably. This is safe because
    // joker pre-scoring effects don't modify the jokers vector.
    let num_jokers = game_state.jokers.len();
    for idx in 0..num_jokers {
        let joker: *const _ = &game_state.jokers[idx];
        unsafe {
            (*joker).apply_pre_scoring_effects(game_state)?;
        }
    }
    Ok(())
}

/// Phase 2: Played Hand Scoring
/// Evaluate each card in the selected hand
fn apply_played_hand_scoring(game_state: &mut GameState) -> GameResult<HandScore> {
    let selected_cards = game_state.hand.selected_cards_mut();
    let poker_hand = game_state.planets.detect_poker_hand(&selected_cards).ok_or(GameError::InvalidGameState("No poker hand detected".to_string()))?;
    let mut hand_score = game_state.planets.get_planet(poker_hand).ok_or(GameError::InvalidGameState("No planet found for poker hand".to_string()))?.get_base_score();

    for card in selected_cards{
        // Count retriggers before scoring the card and add 1 for the base card
        let retrigger_count = 1 + count_retriggers(game_state, &card)?;

        // Apply retriggers
        for _ in 0..retrigger_count {
            hand_score = score_card(game_state, &card, hand_score)?;
        }
    }
    Ok(hand_score)
}

/// Score a single card, applying A through F
fn score_card(game_state: &mut GameState, card: &Card, mut hand_score: HandScore) -> GameResult<HandScore> {
    // A - Add Base Card Chips
    hand_score.chip_score += card.chip_value();

    // B - Apply Enhanced Card Effects
    hand_score = apply_enhanced_card_effects(card, hand_score)?;

    // C - Trigger Card Editions
    hand_score = apply_card_edition(card, hand_score)?;

    // D - Trigger Joker Effects (per_card effects)
    hand_score = apply_per_card_joker_effects(game_state, card, hand_score)?;

    // E - Gold Seal: if card has gold seal, give $3
    if let Some(Seal::Gold) = card.seal {
        game_state.money += 3;
    }

    // F - Retriggers are handled by calling function

    Ok(hand_score)
}

/// Apply enhanced card effects (Step B)
fn apply_enhanced_card_effects(card: &Card, mut hand_score: HandScore) -> GameResult<HandScore> {
    if let Some(enhancement) = &card.enhancement {
        match enhancement {
            Enhancement::Bonus => {
                hand_score.chip_score += 30;
            }
            Enhancement::Mult => {
                hand_score.mult_score += 4.0;
            }
            Enhancement::Wild => {
                // Has no impact on card score
            }
            Enhancement::Glass => {
                hand_score.mult_score *= 2.0;
            }
            Enhancement::Steel => {
                // Has no impact on card score 
            }
            Enhancement::Stone => {
                hand_score.chip_score += 50;
            }
            Enhancement::Gold => {
                // Has no impact on card score 
            }
            Enhancement::Lucky => {
                // TODO: Implement lucky enhancement
            }
        }
    }
    Ok(hand_score)
}

/// Apply card edition effects (Step C)
fn apply_card_edition(card: &Card, mut hand_score: HandScore) -> GameResult<HandScore> {
    match card.edition {
        Edition::Foil => {
            hand_score.chip_score += 50;
        }
        Edition::Holographic => {
            hand_score.mult_score += 10.0;
        }
        Edition::Polychrome => {
            hand_score.mult_score *= 1.5;
        }
        Edition::Negative => {
            // No scoring effects for negative edition
        }
        Edition::Base => {
            // No effects for base edition
        }
    }
    Ok(hand_score)
}

/// Apply per_card joker effects (Step D)
fn apply_per_card_joker_effects(game_state: &GameState, card: &Card, mut hand_score: HandScore) -> GameResult<HandScore> {
    
    for joker in &game_state.jokers {
        // Check if this joker has a conditional effect that is per_card
        if let Some(JokerCondition { .. }) = &joker.definition.effect.condition {
            if joker.definition.effect.per_card {
                let (chip_mod, mult_mod) = joker.apply_per_card_effects(card.clone())?;
                hand_score.chip_score += chip_mod;
                hand_score.mult_score *= mult_mod;
            }
        }
    }
    
    Ok(hand_score)
}

/// Count the number of retriggers for a card
fn count_retriggers(game_state: &GameState, card: &Card) -> GameResult<usize> {
    let mut retrigger_count = 0;

    // Check if card has red seal
    if let Some(Seal::Red) = card.seal {
        retrigger_count += 1;
    }

    // Check jokers for retrigger effects
    for joker in &game_state.jokers {
        if let Some(action) = &joker.definition.effect.action {
            if matches!(action.action_type, ActionType::Retrigger | ActionType::RetriggerInHand) {
                // TODO: Check if joker condition is met for this card
                // For now, count all retrigger jokers
                retrigger_count += 1;
            }
        }
    }

    Ok(retrigger_count)
}

/// Phase 3: Effects in Hand
/// Process unselected cards from left to right
fn apply_effects_in_hand(game_state: &mut GameState, mut hand_score: HandScore) -> GameResult<HandScore> {
    // Get unselected cards in order (left to right)
    let unselected_cards = game_state.hand.get_unselected_cards();

    // Process each unselected card left to right
    for card_shared in unselected_cards {
        let card = card_shared.borrow();
        // Count retriggers before processing the card
        let retrigger_count = count_retriggers_in_hand(game_state, &card)?;

        // Apply card effects with retriggers
        for _ in 0..retrigger_count + 1 {
            hand_score = apply_card_effects_in_hand(game_state, &card, hand_score)?;
        }
    }
    
    Ok(hand_score)
}

/// Apply effects from a single unselected card in hand
fn apply_card_effects_in_hand(game_state: &GameState, card: &Card, mut hand_score: HandScore) -> GameResult<HandScore> {
    // A - Steal: If card has Steal enhancement, multiply mult by 1.5
    if let Some(Enhancement::Steel) = &card.enhancement {
        hand_score.mult_score *= 1.5;
    }

    // B - Joker Effects: Jokers triggered by cards held in hand
    let selected_cards: Vec<Card> = game_state.hand.selected_cards_mut();
    for joker in &game_state.jokers {
        // TODO: Check if joker should be triggered by cards in hand
        // This would be jokers with conditional effects that check for held cards
        if joker.definition.effect.condition.is_some() && joker.definition.effect.per_card {
            let (chip_mod, mult_mod) = joker.apply_in_hand_effects(selected_cards.clone())?;
            hand_score.chip_score += chip_mod;
            hand_score.mult_score *= mult_mod;
        }
    }

    Ok(hand_score)
}

/// Count retriggers for cards in hand
fn count_retriggers_in_hand(game_state: &GameState, card: &Card) -> GameResult<usize> {
    let mut retrigger_count = 0;

    // Check if card has red seal
    if let Some(Seal::Red) = card.seal {
        retrigger_count += 1;
    }

    // Check jokers for retrigger_in_hand effects
    for joker in &game_state.jokers {
        // TODO: Check if joker has retrigger_in_hand action type
        // This would need to be added to the joker effect configuration
        if let Some(action) = &joker.definition.effect.action {
            if matches!(action.action_type, ActionType::RetriggerInHand) && joker.definition.effect.per_card {
                retrigger_count += 1;
            }
        }
    }

    Ok(retrigger_count)
}

/// Phase 4: Joker Scoring
/// Apply effects from active jokers
fn apply_joker_scoring(game_state: &GameState, mut hand_score: HandScore) -> GameResult<HandScore> {
    
    let selected_cards: Vec<Card> = game_state.hand.selected_cards_mut();
    for joker in &game_state.jokers {
        // TODO: Implement full joker effect application
        // For now, apply basic joker effects
        let (chip_mod, mult_mod) = joker.apply_played_hand_effects(selected_cards.clone())?;
        hand_score.chip_score += chip_mod;
        hand_score.mult_score *= mult_mod;
    }
    
    Ok(hand_score)
}

