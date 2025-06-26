#![allow(dead_code, unused)]

use runs::runs::{gets_file_name, read_file};
use std::env;
fn main() {
    let filename = env::args().skip(1).next().expect("No file specified.");
    let filename = gets_file_name(&filename).expect("can't read file");
    println!("{:?}", read_file(&filename[..]));
}
