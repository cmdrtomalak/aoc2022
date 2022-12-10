use anyhow::Result;
use itertools::Itertools;
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

fn parse(raw: &Vec<String>) -> Vec<(usize, usize, usize, usize)> {
    raw.iter()
        .map(|l| {
            l.split(['-', ','])
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap()
        })
        .collect()
}

fn main() {
    let raw = read_file("input.txt").unwrap();
    let input = parse(&raw);
    let contain: i32 = input
        .iter()
        .map(|(a1, a2, b1, b2)| {
            if a1 <= b1 && b2 <= a2 {
                // b is a subset of a
                1
            } else if b1 <= a1 && a2 <= b2 {
                // a is a subset of b
                1
            } else {
                0
            }
        })
        .sum();

    let overlap: i32 = input
        .iter()
        .map(|(a1, a2, b1, b2)| {
            if b1 <= a2 && a1 <= b2 {
                1
            } else if a1 <= b2 && b1 <= a2 {
                1
            } else {
                0
            }
        })
        .sum();

    println!("Part 1: {contain}");
    println!("Part 2: {overlap}");
}
