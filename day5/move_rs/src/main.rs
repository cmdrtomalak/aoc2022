use std::collections::HashMap;
use std::fs;
use std::str::Lines;

#[derive(Debug)]
struct Move {
    n_crate: usize,
    src: usize,
    dest: usize,
}

fn parse_stack(stack_text: Lines) -> HashMap<usize, String> {
    let mut stack_map: HashMap<usize, String> = (1..10).map(|x| (x, String::new())).collect();

    for line in stack_text.into_iter().rev().skip(1) {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            let mut current = stack_map.get(&(i + 1)).unwrap().clone();
            if c != ' ' {
                current.push(c);
                stack_map.insert(i + 1, current);
            }
        }
    }
    stack_map
}

fn parse_moves(moves_as_string: Lines) -> Vec<Move> {
    let mut moves_as_vec: Vec<Move> = Vec::new();
    for line in moves_as_string {
        let mut vals = line.split_whitespace().into_iter().skip(1).step_by(2);
        let n_crate = vals.next().unwrap().parse().unwrap();
        let src = vals.next().unwrap().parse().unwrap();
        let dest = vals.next().unwrap().parse().unwrap();
        moves_as_vec.push(Move { n_crate, src, dest });
    }
    moves_as_vec
}

fn solver(stack_map: &mut HashMap<usize, String>, moves_vec: &Vec<Move>, part: u8) -> String {
    for m in moves_vec.iter() {
        let src = stack_map.get(&m.src).unwrap().clone();
        let mut dest = stack_map.get(&m.dest).unwrap().clone();

        let split_idx = src.len() - m.n_crate;
        let (new_src, tail) = src.split_at(split_idx);

        if part == 1 {
            dest = dest + &tail.chars().rev().collect::<String>();
        } else if part == 2 {
            dest = dest + tail;
        }

        stack_map.insert(m.src, new_src.to_string());
        stack_map.insert(m.dest, dest);
    }

    let mut tops = String::new();
    for i in 1..10 {
        let top = stack_map.get(&i).unwrap().clone().pop().unwrap();
        tops.push(top);
    }
    tops
}

fn main() {
    let filename = "input.txt";

    let file_as_string = fs::read_to_string(filename).unwrap();
    let file_as_vec: Vec<&str> = file_as_string.split("\n\n").collect();

    let stack_text = file_as_vec.first().unwrap().lines();
    let mut stack_map = parse_stack(stack_text);

    let moves_as_string = file_as_vec.last().unwrap().lines();
    let moves_as_vec: Vec<Move> = parse_moves(moves_as_string);

    let tops1 = solver(&mut stack_map.clone(), &moves_as_vec, 1);
    let tops2 = solver(&mut stack_map, &moves_as_vec, 2);
    println!("Part 1: {}", tops1);
    println!("Part 2: {}", tops2);
}
