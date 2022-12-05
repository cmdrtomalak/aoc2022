use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

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

fn main() {
    let input: Vec<&str> = INPUT.lines().collect();

    get_priority(&'a');
}
