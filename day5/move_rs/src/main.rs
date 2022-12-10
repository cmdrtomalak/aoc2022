use anyhow::{anyhow, Result};
use std::fs;
use std::str::FromStr;

/*
fn read_file(filename: &str) -> Result<(Vec<String>, Vec<String>)> {
    let data = fs::read_to_string(filename).unwrap();

    let mut split = data.split("\n\n");
    let stack_text = split
        .next()
        .ok_or_else(|| anyhow!("Failed to gt stack text from input file"))
        .and_then(|s| {
            s.trim()
                .split("\n")
                .map(|s| String::from_str(s.trim()))
                .collect()
        })?;
    let movements = split
        .next()
        .ok_or_else(|| anyhow!("Failed to get movements from input file"))
        .collect::<Vec<String>>();

    Ok((stack_text, movements))
}
*/

fn read_file(filename: &str) -> Result<(Vec<String>, Vec<String>)> {
    let data = fs::read_to_string(filename)?;

    let mut split = data.split("\n\n");
    let stack_text = split
        .next()
        .ok_or_else(|| anyhow!("Failed to get stack text from input file"))
        .and_then(|s| {
            Ok(s.trim()
                .split("\n")
                .map(|s| String::from_str(s.trim()))
                .collect::<Result<Vec<_>, _>>())
        })?;
    let movements = split
        .next()
        .ok_or_else(|| anyhow!("Failed to get movements from input file"))
        .and_then(|s| {
            Ok(s.trim()
                .split("\n")
                .map(|s| String::from_str(s.trim()))
                .collect::<Result<Vec<_>, _>>())
        })?;

    Ok((stack_text.unwrap(), movements.unwrap()))
}

fn main() {
    let (stack_text, movements) = read_file("input.txt").unwrap();
    dbg!(&stack_text);
    dbg!(&movements);
}
