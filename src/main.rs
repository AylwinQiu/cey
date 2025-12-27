use cey;
use std::{io, fs};

// Tis function will return the 
fn read_from_file(path: &str) -> Vec<char> {
    let content = fs::read_to_string(path).expect("Cannot read the file, please check the path");
    return content.chars().collect();
}

fn main() {
    println!("Hello, world!");
}
