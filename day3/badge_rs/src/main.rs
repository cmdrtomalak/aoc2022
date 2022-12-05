use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

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

    dbg!(lower);
    dbg!(upper);

    5
}

fn main() -> Result<()> {
    // get_priority(&'a');
    let data = read_file("input.txt")?;

    Ok(())
}
