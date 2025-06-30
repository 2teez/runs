#![allow(dead_code, unused)]

use runs::runs::create_temp_project;
use std::env;
fn main() {
    let data = env::args().collect::<Vec<_>>();
    create_temp_project(&data);
}
