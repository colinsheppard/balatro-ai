//! Input source handling for automated testing and interactive mode

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write, Read};
use std::sync::mpsc;
use std::time::Duration;

/// Input source for automated testing and user interaction
pub enum InputSource {
    Interactive,
    InteractiveRecording(File), // Interactive mode with recording
    File(BufReader<File>),
    FileRecording(BufReader<File>, File), // File input with recording
    FileThenInteractive(BufReader<File>, bool), // File input that falls back to interactive
    FileThenInteractiveRecording(BufReader<File>, File, bool), // File then interactive with recording
    Stdin(BufReader<io::Stdin>),
    StdinRecording(BufReader<io::Stdin>, File), // Stdin input with recording
    Python(mpsc::Receiver<u32>), // Input from Python via channel
}

impl InputSource {
    /// Create a new input source based on environment and stdin availability
    pub fn new() -> Self {
        // Check if recording is enabled first
        let recording_enabled = env::var("BALATRO_RECORD").is_ok();
        
        // Check if input file is provided via environment variable
        if let Ok(input_file) = env::var("BALATRO_INPUT_FILE") {
            let file = File::open(&input_file)
                .expect(&format!("Failed to open input file: {}", input_file));
            
            if recording_enabled {
                // Create recording file and wrap the input file reader
                let recording_file = Self::create_recording_file_internal();
                // Always enable fallback to interactive for file-based input
                Self::FileThenInteractiveRecording(BufReader::new(file), recording_file, false)
            } else {
                // Always enable fallback to interactive for file-based input
                Self::FileThenInteractive(BufReader::new(file), false)
            }
        } else {
            // Check if stdin has data (for piping)
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            let mut buffer = [0; 1];
            match handle.read(&mut buffer) {
                Ok(0) => {
                    // No data available, check if recording is enabled
                    if recording_enabled {
                        Self::create_recording_file()
                    } else {
                        Self::Interactive
                    }
                }
                Ok(_) => {
                    // Data available, read from stdin
                    if recording_enabled {
                        let recording_file = Self::create_recording_file_internal();
                        Self::StdinRecording(BufReader::new(io::stdin()), recording_file)
                    } else {
                        Self::Stdin(BufReader::new(io::stdin()))
                    }
                }
                Err(_) => {
                    // Check if recording is enabled
                    if recording_enabled {
                        Self::create_recording_file()
                    } else {
                        Self::Interactive
                    }
                }
            }
        }
    }

    fn create_recording_file() -> Self {
        // Create timestamped recording file
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("output/session_{}.txt", timestamp);
        
        // Ensure output directory exists
        std::fs::create_dir_all("output").expect("Failed to create output directory");
        
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&filename)
            .expect(&format!("Failed to create recording file: {}", filename));
        
        println!("Recording session to: {}", filename);
        Self::InteractiveRecording(file)
    }

    fn create_recording_file_internal() -> File {
        // Create timestamped recording file
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("output/session_{}.txt", timestamp);
        
        // Ensure output directory exists
        std::fs::create_dir_all("output").expect("Failed to create output directory");
        
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&filename)
            .expect(&format!("Failed to create recording file: {}", filename));
        
        println!("Recording session to: {}", filename);
        file
    }

    /// Read a line of input from the current source
    pub fn read_line(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::Interactive => {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                Ok(input)
            }
            Self::InteractiveRecording(file) => {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                // Record the input to file
                writeln!(file, "{}", input.trim())?;
                file.flush()?;
                
                Ok(input)
            }
            Self::File(reader) => {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                Ok(line)
            }
            Self::FileRecording(reader, file) => {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                
                // Record the input to file
                writeln!(file, "{}", line.trim())?;
                file.flush()?;
                
                Ok(line)
            }
            Self::FileThenInteractive(reader, eof_reached) => {
                if *eof_reached {
                    // File is exhausted, fall back to interactive input
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    Ok(input)
                } else {
                    // Try to read from file
                    let mut line = String::new();
                    let bytes_read = reader.read_line(&mut line)?;
                    
                    if bytes_read == 0 {
                        // EOF reached, switch to interactive mode
                        *eof_reached = true;
                        println!("\n--- File input exhausted, switching to interactive mode ---");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        Ok(input)
                    } else {
                        Ok(line)
                    }
                }
            }
            Self::FileThenInteractiveRecording(reader, recording_file, eof_reached) => {
                if *eof_reached {
                    // File is exhausted, fall back to interactive input with recording
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    
                    // Record the input to file
                    writeln!(recording_file, "{}", input.trim())?;
                    recording_file.flush()?;
                    
                    Ok(input)
                } else {
                    // Try to read from file
                    let mut line = String::new();
                    let bytes_read = reader.read_line(&mut line)?;
                    
                    if bytes_read == 0 {
                        // EOF reached, switch to interactive mode with recording
                        *eof_reached = true;
                        println!("\n--- File input exhausted, switching to interactive mode ---");
                        
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        
                        // Record the input to file
                        writeln!(recording_file, "{}", input.trim())?;
                        recording_file.flush()?;
                        
                        Ok(input)
                    } else {
                        // Record the file input to recording file
                        writeln!(recording_file, "{}", line.trim())?;
                        recording_file.flush()?;
                        
                        Ok(line)
                    }
                }
            }
            Self::Stdin(reader) => {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                Ok(line)
            }
            Self::StdinRecording(reader, file) => {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                
                // Record the input to file
                writeln!(file, "{}", line.trim())?;
                file.flush()?;
                
                Ok(line)
            }
            Self::Python(receiver) => {
                // In debug mode, use non-blocking polling so IDE debuggers can interrupt cleanly
                let debug_nonblock = std::env::var("BALATRO_DEBUG_NONBLOCK").is_ok();
                if debug_nonblock {
                    loop {
                        match receiver.try_recv() {
                            Ok(choice) => return Ok(choice.to_string()),
                            Err(mpsc::TryRecvError::Empty) => {
                                // Sleep briefly to avoid busy-waiting
                                std::thread::sleep(Duration::from_millis(10));
                                continue;
                            }
                            Err(mpsc::TryRecvError::Disconnected) => {
                                return Err("Python input channel disconnected".into());
                            }
                        }
                    }
                } else {
                    // Default behavior: block waiting for input, with a long timeout to allow signals
                    match receiver.recv_timeout(Duration::from_secs(3600)) {
                        Ok(choice) => Ok(choice.to_string()),
                        Err(mpsc::RecvTimeoutError::Timeout) => {
                            Err("Python input timeout - no input received within 1 hour".into())
                        }
                        Err(mpsc::RecvTimeoutError::Disconnected) => {
                            Err("Python input channel disconnected".into())
                        }
                    }
                }
            }
        }
    }

    /// Read all commands from a file (useful for testing)
    pub fn read_all_commands(file_path: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        
        Ok(reader
            .lines()
            .filter_map(|line| line.ok())
            .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
            .filter_map(|line| line.trim().parse::<u32>().ok())
            .collect())
    }
}

