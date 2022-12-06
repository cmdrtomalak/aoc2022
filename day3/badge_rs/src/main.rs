use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let data = reader.lines().collect::<Result<_, _>>().unwrap();

    Ok(data)
}

fn get_priority(item: &char) -> usize {
    let lower: HashMap<char, usize> = ('a'..='z')
        .into_iter()
        .map(|c| (c, c as usize - 96))
        .collect();

    let upper: HashMap<char, usize> = ('A'..='Z')
        .into_iter()
        .map(|c| (c, c as usize - 38))
        .collect();

    if item.is_lowercase() {
        return *lower.get(item).unwrap();
    } else {
        return *upper.get(item).unwrap();
    }
}

fn main() -> Result<()> {
    let data = read_file("input.txt")?;

    println!(
        "Part 1: {:?}",
        data.iter()
            .map(|r| {
                let (comp1, comp2) = r.split_at(r.len() / 2);
                for c in comp1.chars() {
                    if comp2.contains(c) {
                        return get_priority(&c);
                    }
                }
                panic!("Couldn't find a pair.");
            })
            .sum::<usize>()
    );

    println!(
        "Part 2: {:?}",
        data.iter()
            .tuples()
            .map(|(a, b, c)| {
                for ch in a.chars() {
                    if b.contains(ch) && c.contains(ch) {
                        return get_priority(&ch);
                    }
                }
                panic!("Couldn't find common item.");
            })
            .sum::<usize>()
    );

    Ok(())
}
