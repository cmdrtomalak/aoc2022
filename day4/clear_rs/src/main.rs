use anyhow::Result;
use itertools::Itertools;
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

fn parse(raw: &Vec<String>) -> (HashSet<usize>, HashSet<usize>) {
    let parsed: Vec<(usize, usize, usize, usize)> = raw
        .iter()
        .map(|l| {
            l.split(['-', ','])
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple::<(_, _, _, _)>()
                .map(|(n1, n2, n3, n4)| (n1, n2, n3, n4))
                .unwrap()
        })
        .collect();

    dbg!(parsed);
}

fn main() {
    let raw = read_file("input.txt").unwrap();
    parse(&raw);
}
