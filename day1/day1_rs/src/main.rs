use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const ELF_CALORIES: &str = "elf_calories.txt";

fn main() -> io::Result<()> {
    let file = File::open(ELF_CALORIES)?;
    let reader = BufReader::new(file);

    let mut records = HashMap::new();
    let mut elf_count = 1;

    for line in reader.lines() {
        let value = line?;
        if !value.as_str().trim().is_empty() {
            // println!("{value}")
            *records.entry(elf_count).or_insert(0) += value.parse::<i32>().unwrap();
        } else {
            elf_count += 1;
        }
    }

    // Part 1: How many total Calories is that Elf carrying?
    let elf_with_max_calories = records.iter().max_by_key(|entry| entry.1).unwrap();
    println!("Elf number: {}", &elf_with_max_calories.0);
    println!(
        "Is carrying a total of {} calories",
        &elf_with_max_calories.1
    );

    // To sort, convert it to a vector
    let mut hash_vec: Vec<_> = records.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut sum = 0;
    for i in 0..3 {
        sum += hash_vec[i].1;
    }

    println!("Total calories carried by top 3 Elfs: {sum}");

    Ok(())
}
