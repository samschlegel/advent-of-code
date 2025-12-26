use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn input_generator(input: &str) -> String {
    input.to_string()
}

#[aoc(day1, part1)]
fn part1(input: &String) -> u32 {
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

#[aoc(day1, part2)]
fn part2(input: &String) -> u32 {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return i as u32 + 1;
        }
    }
    0
}
