//! Initialize the input source for the game

use crate::input::InputSource;

/// Global input source
pub static mut INPUT_SOURCE: Option<InputSource> = None;

/// Initialize the input source based on environment variables
pub fn initialize_input_source() {
    unsafe {
        INPUT_SOURCE = Some(InputSource::new());
    }
}

