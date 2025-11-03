//! Non-blocking step driver for Python control

use crate::{BalatroEngine, GamePhase};

#[derive(Debug, Clone)]
pub struct InputContext {
    pub phase: GamePhase,
    pub actions: Vec<(u32, String)>,
}

#[derive(Debug, Clone)]
pub enum StepResult {
    Progressed,
    NeedsInput(InputContext),
    Finished { should_restart: bool },
}

pub enum ProvidedInput {
    Choice(u32),
    None,
}

fn playing_actions_to_tuples(engine: &BalatroEngine) -> Vec<(u32, String)> {
    crate::actions::helpers::create_playing_actions(engine.game_state())
        .into_iter()
        .map(|(idx, a)| (idx, format!("{}", a)))
        .collect()
}

fn shop_actions_to_tuples() -> Vec<(u32, String)> {
    crate::actions::helpers::create_shop_actions()
        .into_iter()
        .map(|(idx, a)| (idx, format!("{}", a)))
        .collect()
}

fn round_end_actions_to_tuples() -> Vec<(u32, String)> {
    crate::actions::helpers::create_round_end_actions()
        .into_iter()
        .map(|(idx, a)| (idx, format!("{}", a)))
        .collect()
}

fn blind_select_actions_to_tuples(engine: &BalatroEngine) -> Vec<(u32, String)> {
    // 1: Play blind; 2: Skip if allowed
    let mut actions = vec![(1, "Play next blind".to_string())];
    if let Some(next_blind) = engine.game_state().upcoming_blinds.borrow().get_next_upcoming_blind() {
        if next_blind.borrow().can_skip() {
            actions.push((2, "Skip blind".to_string()));
        }
    }
    actions
}

fn game_over_actions_to_tuples() -> Vec<(u32, String)> {
    vec![
        (1, "Play Again".to_string()),
        (4, "Exit".to_string()),
    ]
}

pub fn step(engine: &mut BalatroEngine, input: ProvidedInput) -> Result<StepResult, Box<dyn std::error::Error>> {
    // 1) If input is provided for a phase that expects it, process it
    match (engine.game_state().phase.clone(), &input) {
        (GamePhase::Shop, ProvidedInput::Choice(choice)) => {
            let shop_actions = crate::actions::helpers::create_shop_actions();
            super::phases::process_shop_action(engine, &shop_actions, *choice)?;
        }
        (GamePhase::BlindSelect, ProvidedInput::Choice(choice)) => {
            super::phases::process_blind_select_action(engine, *choice)?;
        }
        (GamePhase::Playing, ProvidedInput::Choice(choice)) => {
            let playing = crate::actions::helpers::create_playing_actions(engine.game_state());
            super::phases::process_playing_action(engine, &playing, *choice)?;
        }
        (GamePhase::RoundEnd, ProvidedInput::Choice(choice)) => {
            let round_end = crate::actions::helpers::create_round_end_actions();
            super::phases::process_round_end_action(engine, &round_end, *choice)?;
        }
        (GamePhase::GameOver, ProvidedInput::Choice(choice)) => {
            let should_restart = super::phases::process_game_over_action(engine, *choice)?;
            return Ok(StepResult::Finished { should_restart });
        }
        _ => {}
    }

    // 2) Decide what is needed next
    let phase = engine.game_state().phase.clone();
    match phase {
        GamePhase::Shop => Ok(StepResult::NeedsInput(InputContext { phase, actions: shop_actions_to_tuples() })),
        GamePhase::BlindSelect => Ok(StepResult::NeedsInput(InputContext { phase, actions: blind_select_actions_to_tuples(engine) })),
        GamePhase::Playing => Ok(StepResult::NeedsInput(InputContext { phase, actions: playing_actions_to_tuples(engine) })),
        GamePhase::RoundEnd => Ok(StepResult::NeedsInput(InputContext { phase, actions: round_end_actions_to_tuples() })),
        GamePhase::GameOver => Ok(StepResult::NeedsInput(InputContext { phase, actions: game_over_actions_to_tuples() })),
        GamePhase::ShopPackSelection => Ok(StepResult::Progressed),
    }
}


