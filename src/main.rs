#![allow(dead_code, unused)]

use runs::runs::gets_file_name;
use std::env;
fn main() {
    let filename = env::args().skip(1).next().expect("No file specified.");
    let filename = gets_file_name(&filename);
    println!("{:?}", filename);
}
