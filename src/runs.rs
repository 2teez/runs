#![allow(dead_code, unused)]
//! runs crate
//!
//! This library gets a standalone filename and reads the file,
//! and generate a temporary cargo project. To run a test on a
//! stanalone doctest file.
//! Functions here gets, reads, generate, runs the cargo test --doc.
//! Then delete all the temporary projects, leaving on the standalone
//! doctest file.

use std::env;
use std::fs::{self, File};
use std::io::{self, Error, Read, Write};
use std::ops::Index;
use std::path::Path;
use std::process::Command;

/// enum type for either to create or delete used a a number type
pub(crate) enum Project {
    CREATE,
    DELETE,
}

/// trait Pairs
trait Pairs<T: Clone>: Index<usize, Output = T> {
    fn first(&self) -> T {
        self[0].clone()
    }

    fn second(&self) -> T {
        self[1].clone()
    }

    fn pairs(&self) -> (T, T) {
        (self.first(), self.second())
    }
}
/// trait Pairs impl for [T]
impl<T: Clone> Pairs<T> for [T] {}

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
/// let file = "src/runs/unittest.rs";
/// let lines = read_file(file)[0].to_string();
/// assert!(lines.contains("//! runs crates unittest"));
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

/// creates the folder for the temporary project to be tested
/// using cargo test --doc

pub fn create_temp_project(data: &[String]) {
    let comb_file = data;
    let (file, action) = if comb_file.len() != 2 {
        ("tempo".to_owned(), "create".to_owned())
    } else {
        comb_file.pairs()
    };
    if action == "create".to_owned() {
        create_n_delete(Project::CREATE, &file);
    } else {
        create_n_delete(Project::DELETE, &file);
    }
}

/// This function takes enum Project with a string to either
/// create or delete a temporary project file.
/// It's running a shell within rust.
pub(crate) fn create_n_delete(action: Project, filename: &str) {
    match action {
        Project::CREATE => Command::new("sh")
            .arg("-c")
            .arg(format!("cargo new --lib {}_proj", filename))
            .output()
            .expect("Can't create the new project."),
        Project::DELETE => Command::new("sh")
            .arg("-c")
            .arg(format!("rm -rf {}", filename))
            .output()
            .expect("Can't delete the project."),
    };
}

#[cfg(test)]
mod unittest;
