//! Python bindings for Balatro engine using PyO3
//!
//! This module exposes GameState and Action types to Python.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use crate::actions::{Action, MenuAction, ShopAction, BlindAction, PlayingAction, RoundEndAction, GameOverAction};
#[cfg(feature = "python")]
use crate::deck::DeckType;
#[cfg(feature = "python")]
use crate::game::{GameState, GamePhase};
#[cfg(feature = "python")]
use crate::stakes::StakeLevel;
#[cfg(feature = "python")]
use serde_json;

// Supporting enum types exposed to Python

#[cfg(feature = "python")]
#[pyclass(name = "GamePhase")]
#[derive(Clone)]
pub struct PyGamePhase {
    phase: GamePhase,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyGamePhase {
    #[new]
    fn new(phase_str: &str) -> PyResult<Self> {
        let phase = match phase_str {
            "Shop" => GamePhase::Shop,
            "ShopPackSelection" => GamePhase::ShopPackSelection,
            "BlindSelect" => GamePhase::BlindSelect,
            "Playing" => GamePhase::Playing,
            "RoundEnd" => GamePhase::RoundEnd,
            "GameOver" => GamePhase::GameOver,
            _ => return Err(PyValueError::new_err(format!("Invalid GamePhase: {}", phase_str))),
        };
        Ok(Self { phase })
    }

    fn __str__(&self) -> String {
        match self.phase {
            GamePhase::Shop => "Shop",
            GamePhase::ShopPackSelection => "ShopPackSelection",
            GamePhase::BlindSelect => "BlindSelect",
            GamePhase::Playing => "Playing",
            GamePhase::RoundEnd => "RoundEnd",
            GamePhase::GameOver => "GameOver",
        }.to_string()
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "DeckType")]
#[derive(Clone)]
pub struct PyDeckType {
    deck_type: DeckType,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyDeckType {
    #[new]
    fn new(deck_type_str: &str) -> PyResult<Self> {
        let deck_type = match deck_type_str {
            "Red" => DeckType::Red,
            "Blue" => DeckType::Blue,
            "Yellow" => DeckType::Yellow,
            "Green" => DeckType::Green,
            "Black" => DeckType::Black,
            "Magic" => DeckType::Magic,
            "Nebula" => DeckType::Nebula,
            "Ghost" => DeckType::Ghost,
            "Abandoned" => DeckType::Abandoned,
            "Checkered" => DeckType::Checkered,
            "Zodiac" => DeckType::Zodiac,
            "Painted" => DeckType::Painted,
            "Anaglyph" => DeckType::Anaglyph,
            "Plasma" => DeckType::Plasma,
            "Erratic" => DeckType::Erratic,
            _ => return Err(PyValueError::new_err(format!("Invalid DeckType: {}", deck_type_str))),
        };
        Ok(Self { deck_type })
    }

    fn __str__(&self) -> String {
        format!("{}", self.deck_type)
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "StakeLevel")]
#[derive(Clone)]
pub struct PyStakeLevel {
    stake_level: StakeLevel,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyStakeLevel {
    #[new]
    fn new(stake_level_str: &str) -> PyResult<Self> {
        let stake_level = match stake_level_str {
            "White" => StakeLevel::White,
            "Red" => StakeLevel::Red,
            "Green" => StakeLevel::Green,
            "Blue" => StakeLevel::Blue,
            "Black" => StakeLevel::Black,
            "Purple" => StakeLevel::Purple,
            "Orange" => StakeLevel::Orange,
            "Gold" => StakeLevel::Gold,
            _ => return Err(PyValueError::new_err(format!("Invalid StakeLevel: {}", stake_level_str))),
        };
        Ok(Self { stake_level })
    }

    fn __str__(&self) -> String {
        format!("{}", self.stake_level)
    }
}

// Action types exposed to Python

#[cfg(feature = "python")]
#[pyclass(name = "MenuAction")]
#[derive(Clone)]
pub struct PyMenuAction {
    action: MenuAction,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyMenuAction {
    #[new]
    fn new(action_type: &str, value: Option<PyObject>, py: Python) -> PyResult<Self> {
        let action = match action_type {
            "SelectDeck" => {
                let deck_str = value
                    .and_then(|v| v.extract::<String>(py).ok())
                    .ok_or_else(|| PyValueError::new_err("SelectDeck requires a deck type string"))?;
                let deck_type = match deck_str.as_str() {
                    "Red" => DeckType::Red,
                    "Blue" => DeckType::Blue,
                    "Yellow" => DeckType::Yellow,
                    "Green" => DeckType::Green,
                    "Black" => DeckType::Black,
                    "Magic" => DeckType::Magic,
                    "Nebula" => DeckType::Nebula,
                    "Ghost" => DeckType::Ghost,
                    "Abandoned" => DeckType::Abandoned,
                    "Checkered" => DeckType::Checkered,
                    "Zodiac" => DeckType::Zodiac,
                    "Painted" => DeckType::Painted,
                    "Anaglyph" => DeckType::Anaglyph,
                    "Plasma" => DeckType::Plasma,
                    "Erratic" => DeckType::Erratic,
                    _ => return Err(PyValueError::new_err(format!("Invalid deck type: {}", deck_str))),
                };
                MenuAction::SelectDeck(deck_type)
            }
            "SelectStake" => {
                let stake_str = value
                    .and_then(|v| v.extract::<String>(py).ok())
                    .ok_or_else(|| PyValueError::new_err("SelectStake requires a stake level string"))?;
                let stake_level = match stake_str.as_str() {
                    "White" => StakeLevel::White,
                    "Red" => StakeLevel::Red,
                    "Green" => StakeLevel::Green,
                    "Blue" => StakeLevel::Blue,
                    "Black" => StakeLevel::Black,
                    "Purple" => StakeLevel::Purple,
                    "Orange" => StakeLevel::Orange,
                    "Gold" => StakeLevel::Gold,
                    _ => return Err(PyValueError::new_err(format!("Invalid stake level: {}", stake_str))),
                };
                MenuAction::SelectStake(stake_level)
            }
            "StartGame" => MenuAction::StartGame,
            "Exit" => MenuAction::Exit,
            _ => return Err(PyValueError::new_err(format!("Invalid MenuAction type: {}", action_type))),
        };
        Ok(Self { action })
    }

    fn index(&self) -> u32 {
        self.action.index()
    }

    fn description(&self) -> &str {
        self.action.description()
    }

    fn is_valid(&self) -> bool {
        self.action.is_valid()
    }

    fn __str__(&self) -> String {
        format!("{}", self.action)
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "ShopAction")]
#[derive(Clone)]
pub struct PyShopAction {
    action: ShopAction,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyShopAction {
    #[new]
    fn new(action_type: &str) -> PyResult<Self> {
        let action = match action_type {
            "NextRound" => ShopAction::NextRound,
            _ => return Err(PyValueError::new_err(format!("Invalid ShopAction type: {}", action_type))),
        };
        Ok(Self { action })
    }

    fn index(&self) -> u32 {
        self.action.index()
    }

    fn description(&self) -> &str {
        self.action.description()
    }

    fn is_valid(&self) -> bool {
        self.action.is_valid()
    }

    fn __str__(&self) -> String {
        format!("{}", self.action)
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "BlindAction")]
#[derive(Clone)]
pub struct PyBlindAction {
    action: BlindAction,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyBlindAction {
    #[new]
    fn new(action_type: &str) -> PyResult<Self> {
        let action = match action_type {
            "SelectBossBlind" => BlindAction::SelectBossBlind,
            "SelectEliteBlind" => BlindAction::SelectEliteBlind,
            "SelectNormalBlind" => BlindAction::SelectNormalBlind,
            "ViewBlindDetails" => BlindAction::ViewBlindDetails,
            _ => return Err(PyValueError::new_err(format!("Invalid BlindAction type: {}", action_type))),
        };
        Ok(Self { action })
    }

    fn index(&self) -> u32 {
        self.action.index()
    }

    fn description(&self) -> &str {
        self.action.description()
    }

    fn is_valid(&self) -> bool {
        self.action.is_valid()
    }

    fn __str__(&self) -> String {
        format!("{}", self.action)
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "PlayingAction")]
#[derive(Clone)]
pub struct PyPlayingAction {
    action: PlayingAction,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyPlayingAction {
    #[new]
    fn new(action_type: &str, index: Option<usize>) -> PyResult<Self> {
        let action = match action_type {
            "PlaySelectedCards" => PlayingAction::PlaySelectedCards,
            "DiscardSelectedCards" => PlayingAction::DiscardSelectedCards,
            "SelectCard" => {
                let idx = index.ok_or_else(|| PyValueError::new_err("SelectCard requires an index"))?;
                PlayingAction::SelectCard(idx)
            }
            "DeselectCard" => {
                let idx = index.ok_or_else(|| PyValueError::new_err("DeselectCard requires an index"))?;
                PlayingAction::DeselectCard(idx)
            }
            "MoveRight" => {
                let idx = index.ok_or_else(|| PyValueError::new_err("MoveRight requires an index"))?;
                PlayingAction::MoveRight(idx)
            }
            "MoveLeft" => {
                let idx = index.ok_or_else(|| PyValueError::new_err("MoveLeft requires an index"))?;
                PlayingAction::MoveLeft(idx)
            }
            "SortByRank" => PlayingAction::SortByRank,
            "SortBySuit" => PlayingAction::SortBySuit,
            _ => return Err(PyValueError::new_err(format!("Invalid PlayingAction type: {}", action_type))),
        };
        Ok(Self { action })
    }

    fn index(&self) -> u32 {
        self.action.index()
    }

    fn description(&self) -> &str {
        self.action.description()
    }

    fn is_valid(&self) -> bool {
        self.action.is_valid()
    }

    fn __str__(&self) -> String {
        format!("{}", self.action)
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "RoundEndAction")]
#[derive(Clone)]
pub struct PyRoundEndAction {
    action: RoundEndAction,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyRoundEndAction {
    #[new]
    fn new(action_type: &str) -> PyResult<Self> {
        let action = match action_type {
            "CashOut" => RoundEndAction::CashOut,
            _ => return Err(PyValueError::new_err(format!("Invalid RoundEndAction type: {}", action_type))),
        };
        Ok(Self { action })
    }

    fn index(&self) -> u32 {
        self.action.index()
    }

    fn description(&self) -> &str {
        self.action.description()
    }

    fn is_valid(&self) -> bool {
        self.action.is_valid()
    }

    fn __str__(&self) -> String {
        format!("{}", self.action)
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "GameOverAction")]
#[derive(Clone)]
pub struct PyGameOverAction {
    action: GameOverAction,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyGameOverAction {
    #[new]
    fn new(action_type: &str) -> PyResult<Self> {
        let action = match action_type {
            "PlayAgain" => GameOverAction::PlayAgain,
            "MainMenu" => GameOverAction::MainMenu,
            "ViewFinalStatistics" => GameOverAction::ViewFinalStatistics,
            "Exit" => GameOverAction::Exit,
            _ => return Err(PyValueError::new_err(format!("Invalid GameOverAction type: {}", action_type))),
        };
        Ok(Self { action })
    }

    fn index(&self) -> u32 {
        self.action.index()
    }

    fn description(&self) -> &str {
        self.action.description()
    }

    fn is_valid(&self) -> bool {
        self.action.is_valid()
    }

    fn __str__(&self) -> String {
        format!("{}", self.action)
    }
}

// GameState wrapper

#[cfg(feature = "python")]
#[pyclass(name = "GameState")]
pub struct PyGameState {
    game_state: GameState,
}

// Safe to implement Send because Python's GIL ensures single-threaded access
#[cfg(feature = "python")]
unsafe impl Send for PyGameState {}

#[cfg(feature = "python")]
#[pymethods]
impl PyGameState {
    // Primitive field accessors
    fn phase(&self) -> PyGamePhase {
        PyGamePhase {
            phase: self.game_state.phase.clone(),
        }
    }

    fn ante(&self) -> u32 {
        self.game_state.ante
    }

    fn hand_size(&self) -> usize {
        self.game_state.hand_size
    }

    fn money(&self) -> i32 {
        self.game_state.money
    }

    fn score(&self) -> i32 {
        self.game_state.score
    }

    fn round_number(&self) -> u32 {
        self.game_state.round_number
    }

    // Accessors for shared types (Rc<RefCell<>>) - return JSON serialized data
    fn deck_json(&self) -> PyResult<String> {
        let deck_ref = self.game_state.deck.borrow();
        // Create a simplified serializable representation
        #[derive(serde::Serialize)]
        struct DeckInfo {
            deck_type: String,
            cards_remaining: usize,
            full_deck_size: usize,
            discard_pile_size: usize,
        }
        let info = DeckInfo {
            deck_type: format!("{}", deck_ref.deck_type),
            cards_remaining: deck_ref.n_remaining_cards(),
            full_deck_size: deck_ref.n_cards_full_deck(),
            discard_pile_size: deck_ref.discard_pile.len(),
        };
        serde_json::to_string(&info)
            .map_err(|e| PyValueError::new_err(format!("Failed to serialize deck: {}", e)))
    }

    fn hand_json(&self) -> PyResult<String> {
        let hand_ref = self.game_state.hand.borrow();
        serde_json::to_string(&*hand_ref)
            .map_err(|e| PyValueError::new_err(format!("Failed to serialize hand: {}", e)))
    }

    fn jokers_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.game_state.jokers)
            .map_err(|e| PyValueError::new_err(format!("Failed to serialize jokers: {}", e)))
    }

    fn consumables_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.game_state.consumables)
            .map_err(|e| PyValueError::new_err(format!("Failed to serialize consumables: {}", e)))
    }

    fn upcoming_blinds_json(&self) -> PyResult<String> {
        let blinds_ref = self.game_state.upcoming_blinds.borrow();
        // Create a simplified serializable representation
        #[derive(serde::Serialize)]
        struct BlindInfo {
            name: String,
            blind_type: String,
            required_score: i32,
            reward_money: i32,
            status: String,
        }
        let small = blinds_ref.small.borrow();
        let big = blinds_ref.big.borrow();
        let boss = blinds_ref.boss.borrow();
        
        #[derive(serde::Serialize)]
        struct UpcomingBlindsInfo {
            small: BlindInfo,
            big: BlindInfo,
            boss: BlindInfo,
        }
        let info = UpcomingBlindsInfo {
            small: BlindInfo {
                name: small.name.clone(),
                blind_type: format!("{:?}", small.blind_type),
                required_score: small.required_score,
                reward_money: small.reward_money,
                status: format!("{:?}", small.status),
            },
            big: BlindInfo {
                name: big.name.clone(),
                blind_type: format!("{:?}", big.blind_type),
                required_score: big.required_score,
                reward_money: big.reward_money,
                status: format!("{:?}", big.status),
            },
            boss: BlindInfo {
                name: boss.name.clone(),
                blind_type: format!("{:?}", boss.blind_type),
                required_score: boss.required_score,
                reward_money: boss.reward_money,
                status: format!("{:?}", boss.status),
            },
        };
        serde_json::to_string(&info)
            .map_err(|e| PyValueError::new_err(format!("Failed to serialize upcoming_blinds: {}", e)))
    }

    fn play_limits_json(&self) -> PyResult<String> {
        let limits_ref = self.game_state.play_limits.borrow();
        serde_json::to_string(&*limits_ref)
            .map_err(|e| PyValueError::new_err(format!("Failed to serialize play_limits: {}", e)))
    }

    fn stake_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.game_state.stake)
            .map_err(|e| PyValueError::new_err(format!("Failed to serialize stake: {}", e)))
    }

    fn planets_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.game_state.planets)
            .map_err(|e| PyValueError::new_err(format!("Failed to serialize planets: {}", e)))
    }

    // Helper methods for accessing hand information
    fn hand_size_actual(&self) -> PyResult<usize> {
        let hand_ref = self.game_state.hand.borrow();
        Ok(hand_ref.len())
    }

    fn hand_selected_indices(&self) -> PyResult<Vec<usize>> {
        let hand_ref = self.game_state.hand.borrow();
        Ok(hand_ref.selected_indices().to_vec())
    }

    // Helper methods for accessing deck information
    fn deck_remaining_cards(&self) -> PyResult<usize> {
        let deck_ref = self.game_state.deck.borrow();
        Ok(deck_ref.n_remaining_cards())
    }

    fn deck_type(&self) -> PyResult<PyDeckType> {
        let deck_ref = self.game_state.deck.borrow();
        Ok(PyDeckType {
            deck_type: deck_ref.deck_type.clone(),
        })
    }

    // Helper methods for accessing play limits
    fn hands_remaining(&self) -> PyResult<u32> {
        let limits_ref = self.game_state.play_limits.borrow();
        Ok(limits_ref.n_hands_remaining)
    }

    fn discards_remaining(&self) -> PyResult<u32> {
        let limits_ref = self.game_state.play_limits.borrow();
        Ok(limits_ref.n_discards_remaining)
    }
}

#[cfg(feature = "python")]
impl From<GameState> for PyGameState {
    fn from(game_state: GameState) -> Self {
        Self { game_state }
    }
}

// Module initialization

#[cfg(feature = "python")]
#[pymodule]
fn balatro_engine(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register supporting types
    m.add_class::<PyGamePhase>()?;
    m.add_class::<PyDeckType>()?;
    m.add_class::<PyStakeLevel>()?;

    // Register Action types
    m.add_class::<PyMenuAction>()?;
    m.add_class::<PyShopAction>()?;
    m.add_class::<PyBlindAction>()?;
    m.add_class::<PyPlayingAction>()?;
    m.add_class::<PyRoundEndAction>()?;
    m.add_class::<PyGameOverAction>()?;

    // Register GameState
    m.add_class::<PyGameState>()?;

    Ok(())
}
