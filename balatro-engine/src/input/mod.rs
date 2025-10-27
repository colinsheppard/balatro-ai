//! Input source handling for automated testing and interactive mode

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write, Read};

/// Input source for automated testing and user interaction
pub enum InputSource {
    Interactive,
    InteractiveRecording(File), // Interactive mode with recording
    File(BufReader<File>),
    FileRecording(BufReader<File>, File), // File input with recording
    Stdin(BufReader<io::Stdin>),
    StdinRecording(BufReader<io::Stdin>, File), // Stdin input with recording
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
                Self::FileRecording(BufReader::new(file), recording_file)
            } else {
                Self::File(BufReader::new(file))
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

