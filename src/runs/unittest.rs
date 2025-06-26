//! runs crates unittest
//!
//! The Library runs shows all the unittest here
//!

use super::*;

#[test]
fn test_get_filename() {
    let filename = "src/runs.rs";
    assert_eq!(
        "src/runs.rs",
        gets_file_name(&filename).expect("Invalid filename"),
        "Valid filename!"
    );
}

#[test]
#[ignore = "should panic"]
#[should_panic(expected = "different file locations.")]
fn test_validate_filename() {
    let filename = "runs.rs";
    assert_eq!(
        "src/runs.rs",
        gets_file_name(&filename).expect("Invalid filename"),
        "Valid filename!"
    );
}
