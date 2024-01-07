//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use doc_ur_code::config::Logging;
//! let config = Logging::new();
//! ```
//!
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/** This module contains the configuration options for the application.
  # Examples:
  ```
  use doc_ur_code::config::Logging;
  let config = Logging::new();
  ```
 */ 
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/** This struct contains configuration options for the controlling logging.
  # Examples:
  ```
  use doc_ur_code::config::Logging;
  let config = Logging::new();
  ```
  
  Creating a new instance of the Logging struct:
  ```
  use doc_ur_code::config::{Logging, LogLevel, LogOutput};
  let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::File("log.txt".to_string()) };
  ```
*/ 
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

// create a getter for the enabled field
impl Logging {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}
// create a setter for the enabled field
impl Logging {
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}
// create a getter for the level field
impl Logging {
    pub fn level(&self) -> &LogLevel {
        &self.level
    }
}
// create a setter for the level field
impl Logging {
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }
}
// create a getter for the destination field
impl Logging {
    pub fn destination(&self) -> &LogOutput {
        &self.destination
    }
}
// create a setter for the destination field
impl Logging {
    pub fn set_destination(&mut self, destination: LogOutput) {
        self.destination = destination;
    }
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}
