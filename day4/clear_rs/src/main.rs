use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_file(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut data = vec![];
    for line in reader.lines() {
        data.push(line?);
    }

    dbg!(&data);
    Ok(data)
}

fn main() {
    read_file("input.txt");
}
