use itertools::Itertools;

fn main() {
    let data = include_str!("../../input.txt");

    let part1 = data
        .as_bytes()
        .windows(4)
        .position(|window| window.iter().all_unique())
        .map(|idx| idx + 4)
        .unwrap();

    let part2 = data
        .as_bytes()
        .windows(14)
        .position(|win| win.iter().all_unique())
        .map(|idx| idx + 14)
        .unwrap();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
