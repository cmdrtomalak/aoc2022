use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(filename: &str) -> (Vec<String>, Vec<String>) {
    let file = File::open(filename)?;
}

fn main() {
    println!("Hello, world!");
}
