use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_file(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut data = vec![];
    for line in reader.lines() {
        data.push(line?);
    }

    Ok(data)
}

fn game_logic_part1(data: &Vec<String>) -> i32 {
    let mut score = 0;
    for string in data {
        match string.as_str() {
            "A X" => {
                // Rock v Rock, Draw = 1 + 3
                score += 4;
            }
            "A Y" => {
                // Rock v Paper, Win = 2 + 6
                score += 8;
            }
            "A Z" => {
                // Rock v Sissor, Lost = 3 + 0
                score += 3;
            }
            "B X" => {
                // Paper v Rock, Lost = 1 + 0
                score += 1;
            }
            "B Y" => {
                // Paper v Paper, Draw = 2 + 3
                score += 5;
            }
            "B Z" => {
                // Paper v Sissor, Win = 3 + 6
                score += 9;
            }
            "C X" => {
                // Sissor v Rock, Win = 1 + 6
                score += 7;
            }
            "C Y" => {
                // Sissor v Paper, Lost = 2 + 0
                score += 2;
            }
            "C Z" => {
                // Sissor v Sissor, Draw = 3 + 3
                score += 6;
            }
            _ => {}
        }
    }

    score
}

fn game_logic_part2(data: &Vec<String>) -> i32 {
    let mut score = 0;
    for string in data {
        match string.as_str() {
            "A X" => {
                // Rock v Sissor, Lost = 3 + 0
                score += 3;
            }
            "A Y" => {
                // Rock v Rock, Draw = 1 + 3
                score += 4;
            }
            "A Z" => {
                // Rock v Paper, Win = 2 + 6
                score += 8;
            }
            "B X" => {
                // Paper v Rock, Lost = 1 + 0
                score += 1;
            }
            "B Y" => {
                // Paper v Paper, Draw = 2 + 3
                score += 5;
            }
            "B Z" => {
                // Paper v Sissor, Win = 3 + 6
                score += 9;
            }
            "C X" => {
                // Sissor v Paper, Lost = 2 + 0
                score += 2;
            }
            "C Y" => {
                // Sissor v Sissor, Draw = 3 + 3
                score += 6;
            }
            "C Z" => {
                // Sissor v Rock, Win = 1 + 6
                score += 7;
            }
            _ => {}
        }
    }

    score
}

fn main() {
    let data = read_file("input.txt");
    let data = data.unwrap();
    let score1 = game_logic_part1(&data);
    let score2 = game_logic_part2(&data);
    println!("Part 1: {score1}");
    println!("Part 2: {score2}");
}
