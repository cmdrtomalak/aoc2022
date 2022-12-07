use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_file(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut data = vec![];
    for line in reader.lines() {
        data.push(line?);
    }

    Ok(data)
}

fn parse(raw: &Vec<String>) {
    for line in raw {}
}

fn main() {
    let raw = read_file("input.txt").unwrap();
    parse(&raw);
}
