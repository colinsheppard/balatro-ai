//! Error types for the Balatro game engine

use thiserror::Error;

/// Main error type for the game engine
#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid game state: {0}")]
    InvalidGameState(String),
    
    #[error("Invalid card operation: {0}")]
    InvalidCardOperation(String),
    
    #[error("Invalid joker operation: {0}")]
    InvalidJokerOperation(String),
    
    #[error("Invalid deck operation: {0}")]
    InvalidDeckOperation(String),
    
    #[error("Invalid blind operation: {0}")]
    InvalidBlindOperation(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Generic error: {0}")]
    Generic(#[from] anyhow::Error),
}

/// Result type alias for game operations
pub type GameResult<T> = Result<T, GameError>;
