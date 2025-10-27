//! User input handling

use crate::input::InputSource;
use std::io::{self, Write};

/// Get user input as a number
pub fn get_user_input() -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        if let Some(ref mut source) = crate::run::initialize::INPUT_SOURCE {
            match source {
                InputSource::Interactive | InputSource::InteractiveRecording(_) => {
                    print!("\nEnter your choice (number) or 'quit' to exit: ");
                    io::stdout().flush()?;
                }
                _ => {
                    // For automated input, don't print prompt
                }
            }
            
            loop {
                let input = source.read_line()?;
                let trimmed = input.trim();
                
                // Skip empty lines and comments
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }
                
                // Check for quit command
                if trimmed.to_lowercase() == "quit" {
                    println!("Exiting game...");
                    std::process::exit(0);
                }
                
                match trimmed.parse::<u32>() {
                    Ok(choice) => return Ok(choice),
                    Err(_) => {
                        if matches!(source, InputSource::Interactive | InputSource::InteractiveRecording(_)) {
                            println!("Invalid input. Please enter a number or 'quit' to exit.");
                            continue;
                        } else {
                            return Err(format!("Invalid input in automated test: '{}'", trimmed).into());
                        }
                    }
                }
            }
        } else {
            Err("Input source not initialized".into())
        }
    }
}

