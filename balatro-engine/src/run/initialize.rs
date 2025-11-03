//! Initialize the input source for the game

use crate::input::InputSource;
use std::sync::mpsc;
use std::sync::Mutex;

/// Global input source
pub static mut INPUT_SOURCE: Option<InputSource> = None;

/// Global sender for Python input (None if not in Python mode)
static PYTHON_INPUT_SENDER: Mutex<Option<mpsc::Sender<u32>>> = Mutex::new(None);

/// Initialize the input source based on environment variables
pub fn initialize_input_source() {
    unsafe {
        INPUT_SOURCE = Some(InputSource::new());
    }
}

/// Initialize Python input mode (creates a channel for Python to send input)
/// Returns the sender that should be stored for Python to use
pub fn initialize_python_input() -> mpsc::Sender<u32> {
    let (sender, receiver) = mpsc::channel();
    unsafe {
        INPUT_SOURCE = Some(InputSource::Python(receiver));
    }
    // Store sender globally for Python bindings to access
    let mut global_sender = PYTHON_INPUT_SENDER.lock().unwrap();
    *global_sender = Some(sender.clone());
    sender
}

/// Get the global Python input sender (for use in Python bindings)
pub fn get_python_input_sender() -> Option<mpsc::Sender<u32>> {
    let global_sender = PYTHON_INPUT_SENDER.lock().unwrap();
    global_sender.clone()
}

