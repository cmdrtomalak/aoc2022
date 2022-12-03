use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const ELF_CALORIES: &str = "elf_calories.txt";

fn main() -> io::Result<()> {
    let file = File::open(ELF_CALORIES)?;
    let reader = BufReader::new(file);

    let mut elf_vec: Vec<i32> = vec![];

    let mut calories = 0;
    for line in reader.lines() {
        let value = line?;
        if !value.as_str().trim().is_empty() {
            calories += value.parse::<i32>().unwrap();
        } else {
            // add entry at every breakline
            elf_vec.push(calories);
            calories = 0;
        }
    }

    // sort vec highest to lowest
    elf_vec.sort_by(|a, b| b.cmp(a));
    // Part 1: How many total Calories is that Elf carrying?
    println!("Top Elf is carrying a total of {} calories", elf_vec[0]);

    // To sort, convert it to a vector

    let mut sum = 0;
    for i in 0..3 {
        sum += elf_vec[i];
    }

    println!("Total calories carried by top 3 Elfs: {sum}");

    Ok(())
}
