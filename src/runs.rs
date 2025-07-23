#![allow(dead_code, unused)]
//! runs crate
//!
//! This library gets a standalone filename and reads the file,
//! and generate a temporary cargo project. To run a test on a
//! stanalone doctest file.
//! Functions here gets, reads, generate, runs the cargo test --doc.
//! Then delete all the temporary projects, leaving on the standalone
//! doctest file.

use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead, Error, Read, Write};
use std::path::Path;
use std::process::Command;

/// enum type for either to create or delete used a a number type
pub(crate) enum Action {
    CREATE,
    RUN,
    DELETE,
}

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
    let file = &data[1..]; // skip the executable file

    if file.len() != 1 {
        eprintln!("Usage: runs [filename-to-create-from]");
        std::process::exit(1);
    }

    // get the filename
    let file = file.first().unwrap();

    create_run_delete(Action::CREATE, &file);
    create_run_delete(Action::RUN, &file);
    create_run_delete(Action::DELETE, &file);
}

// add specified crates indicated by the user in the
// lib.rs or initial doctest file
pub(crate) fn remake_cargo_file(file: &str) -> Vec<String> {
    // read mode activated here

    let file = File::open(file).expect("can't open file.");
    let buf_from = BufReader::new(file);
    let mut addins = Vec::<String>::new();
    for line in buf_from.lines() {
        let line = line.unwrap().clone();
        if (line.starts_with("/// use") || line.starts_with("///use")) && !line.contains("std::") {
            if let Some(stripped) = line
                .strip_prefix("/// use")
                .or_else(|| line.strip_prefix("///use"))
            {
                let trimmed = stripped.trim();
                if let Some(crate_name) = trimmed.split("::").next() {
                    if !crate_name.starts_with("std") {
                        addins.push(crate_name.to_owned());
                    }
                }
            }
        }
    }
    addins
}

pub(crate) fn copy_file(from: &str, to: &str) {
    // read mode activated here
    let file = File::open(from).expect("can't open file.");
    let buf_from = BufReader::new(file);

    // write mode activated here
    let mut file_to = File::create(to).expect("can't open file");
    for line in buf_from.lines() {
        let line = line.unwrap().clone();
        if line.starts_with("fn") {
            writeln!(file_to, "pub {}", line);
        } else {
            writeln!(file_to, "{}", line);
        }
    }
}

pub(crate) fn check_file(filename: &str) -> io::Result<bool> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    for line in buf.lines() {
        if line.unwrap().contains("/// ```") {
            return Ok(true);
        }
    }
    Ok(false)
}

/// This function strip filename extention off
///
/// ```
/// use runs::runs::remove_file_extention;
/// let file = "document.rs";
/// assert_eq!("document", remove_file_extention(file));
/// ```
///
pub fn remove_file_extention(filename: &str) -> String {
    let mut nfile = String::new();
    if let Some(filename) = String::from(filename).strip_suffix(".rs") {
        nfile = format!("{}", filename);
    }
    nfile
}

/// This function gets input from user and expect a response
///
/// ```
/// use runs::runs::user_input;
/// let answer = user_input("Answer: y/n: "); // response: y
/// assert_eq!(answer, Some("".to_string()));
/// ```

pub fn user_input(msg: &str) -> Option<String> {
    loop {
        print!("{}", msg);
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            eprintln!("Failed to read input.");
            continue;
        }

        let input = line.trim().to_lowercase();
        if input == "y" || input == "n" {
            return Some(input);
        }

        eprintln!("Input must be 'y' or 'n'.");
    }
}

/// This function takes enum Project with a string to either
/// create or delete a temporary project file.
/// It's running a shell within rust.
pub(crate) fn create_run_delete(action: Action, filename: &str) {
    match action {
        Action::CREATE => {
            let status = match check_file(filename) {
                Ok(status) => status,
                Err(err) => {
                    eprintln!("{} {:?}", filename, err.kind());
                    false
                }
            };

            if !status {
                eprintln!("{} - MUST contains doctest", filename);
                std::process::exit(1);
            }

            let _ = Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "cargo new --lib {}_proj",
                    &remove_file_extention(filename)
                ))
                .output()
                .expect("Can't create the new project.");
            // copy content
            copy_file(
                filename,
                &format!("{}_proj/src/lib.rs", &remove_file_extention(filename)),
            );

            // remake cargo file
            for file_to_add in remake_cargo_file(filename) {
                let _ = Command::new("sh")
                    .arg("-c")
                    .arg(format!(
                        "
                        cd {}_proj && cargo add {}",
                        remove_file_extention(filename),
                        file_to_add
                    ))
                    .output()
                    .expect("Can't add specified crate to project.");
            }
        }
        Action::RUN => {
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "cd {}_proj/ && cargo test --all",
                    remove_file_extention(filename)
                ))
                .output()
                .expect("Can't create the new project.");
            println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        }
        Action::DELETE => {
            if let Some(user_reponses) = user_input("Want to keep the temporary folder? [y/n]: ") {
                if user_reponses == "n".to_string() {
                    let _ = Command::new("sh")
                        .arg("-c")
                        .arg(format!("rm -rf {}_proj", remove_file_extention(filename)))
                        .output()
                        .expect("Can't delete the project.");
                }
            }
        }
    };
}

#[cfg(test)]
mod unittest;
