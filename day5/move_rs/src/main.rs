use anyhow::Result;
use std::fs;

fn read_file(filename: &str) -> Result<(Vec<String>, Vec<String>)> {
    let data = fs::read_to_string(filename).unwrap();

    let mut split = data.split("\n\n");
    let stack_text = split
        .next()
        .ok_or("")
        .to_owned()
        .trim()
        .split("\n")
        .collect::<Vec<String>>();
    let movements = split
        .next()
        .ok_or("")
        .to_owned()
        .trim()
        .split("\n")
        .collect::<Vec<String>>();

    Ok((stack_text, movements))
}

fn main() {
    read_file("input.txt");
}
