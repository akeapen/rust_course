//! This is a library that provides utilities for command-line tools.
//! So far it only provides a function to read a line from stdin.
//! # Examples:
//! ```
//! use doc_ur_code::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//! The `read_stdin` function will panic if it fails to read a line with a message "Failed to read input line".

use std::io::{BufRead, BufReader};

pub mod colors;
pub mod config;

/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read input line".
/// # Examples:
/// ```
/// use doc_ur_code::read_stdin;
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

// private function
fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}

// test private function; cfg tells cargo to only compile this code when running tests
#[cfg(test)]
mod tests {
    use super::*;  // necessary for the code to bring all funcs in crate into scope
    use std::io::Cursor;
    use std::io::BufReader;
    use std::io::BufRead;

    #[test]
    #[ignore = "not yet reviewed by the Q.A. team"]
    fn test_read_input() {
        let input = "Hello, world!\n";
        let expected_output = "Hello, world!";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output, 
            "Output: {}, was not as expected: {}", output, expected_output);
    }

    #[test]
    fn test_read_input_empty() {
        let input = "";
        let expected_output = "";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_read_stdin_empty() {
        let input = "";
        let expected_output = "";
        let reader = Cursor::new(input);
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();
        buf_reader
            .read_line(&mut line)
            .expect("Failed to read (empty) input line");
        assert_eq!(line, expected_output);
    }

    #[test]
    #[should_panic]
    fn test_read_stdin_empty_fail() {
        let input = "hahaha!!";
        let expected_output = "";
        let reader = Cursor::new(input);
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();
        buf_reader
            .read_line(&mut line)
            .expect("Failed to read (empty) input line");
        assert_eq!(line, expected_output);
    }

    #[test]
    fn test_buf_reader() {
        let input = "Hello, world!\n";
        let expected_output = "Hello, world!\n";
        let reader = Cursor::new(input);
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();
        buf_reader
            .read_line(&mut line)
            .expect("Failed to read input line");
        assert_eq!(line, expected_output);
    }
}
