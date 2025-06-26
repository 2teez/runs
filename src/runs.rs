#![allow(dead_code, unused)]
//! runs crate
//!
//! This library gets a standalone filename and reads the file,
//! and generate a temporary cargo project. To run a test on a
//! stanalone doctest file.
//! Functions here gets, reads, generate, runs the cargo test --doc.
//! Then delete all the temporary projects, leaving on the standalone
//! doctest file.

use std::io::{self, Error, Read, Write};
use std::path::Path;
/// This function get a valid filename, returns a String or Unit.
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

#[cfg(test)]
mod unittest;
