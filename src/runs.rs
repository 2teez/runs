#![allow(dead_code, unused)]
//! runs crate
//!
//! This library gets a standalone filename and reads the file,
//! and generate a temporary cargo project. To run a test on a
//! stanalone doctest file.
//! Functions here gets, reads, generate, runs the cargo test --doc.
//! Then delete all the temporary projects, leaving on the standalone
//! doctest file.

use std::fs::{self, File};
use std::io::{self, Error, Read, Write};
use std::path::Path;

/// This function get a valid filename, returns a String or io::Error.
/// ```
/// use runs::runs::gets_file_name;
/// let filename = "src/runs.rs";
/// assert_eq!("src/runs.rs", gets_file_name(filename).expect("Invalid filename"), "Valid filename!");
/// ```
pub fn gets_file_name(filename: &str) -> Result<String, Error> {
    let file_path = Path::new(filename);
    if file_path.exists() {
        Ok(filename.to_owned())
    } else {
        Err(io::Error::last_os_error())
    }
}

/// This function read_file, read a file, and returns all the
/// lines in a vector data structure.
/// ```
/// use runs::runs::read_file;
/// let file = "../tests/t.txt";
/// let lines = read_file(file);
/// assert_eq!(lines,vec!["hello", "world", "of\n", "rubies..."]);
/// ```

pub fn read_file(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).expect("No such file or directory");
    let mut data: Vec<String> = Vec::new();
    let mut chunks = [0u8; 128];

    let mut n = file.read(&mut chunks).unwrap_or(0);
    loop {
        if n == 0 {
            break;
        }
        data.push(String::from_utf8_lossy(&chunks[..n]).to_string());
        n = file.read(&mut chunks).unwrap_or(0);
    }
    data
}

#[cfg(test)]
mod unittest;
