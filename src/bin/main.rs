#![allow(dead_code, unused)]

//! # runs binary
//! This binary runs a standalone doctest file by creating a temporary cargo project.

use runs::runs::create_temp_project;
use std::env;
fn main() {
    let data = env::args().collect::<Vec<_>>();
    create_temp_project(&data);
}
