use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> String {
    input.to_string()
}

#[aoc(day4, part1)]
fn part1(input: &String) -> u32 {
    let (i, d) = (1..1_000_000_000)
        .into_par_iter()
        .by_exponential_blocks()
        .map(|i| (i, md5::compute(format!("{}{}", input, i))))
        .find_first(|(_, d)| d.0[0] == 0 && d.0[1] == 0 && d.0[2] < 0x10)
        .expect("we know what this should be");
    println!("Input: {}{}", input, i);
    println!("Hex digest: {}", hex::encode(d.0));
    i
}

#[aoc(day4, part2)]
fn part2(input: &String) -> u32 {
    let (i, d) = (1..1_000_000_000)
        .into_par_iter()
        .by_exponential_blocks()
        .map(|i| (i, md5::compute(format!("{}{}", input, i))))
        .find_first(|(_, d)| d.0[0] == 0 && d.0[1] == 0 && d.0[2] == 0)
        .expect("we know what this should be");
    println!("Input: {}{}", input, i);
    println!("Hex digest: {}", hex::encode(d.0));
    i
}
