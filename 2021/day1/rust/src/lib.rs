use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn count_increases_part1(input: &[i64]) -> usize {
    input.windows(2).map(|w| (w[1] > w[0]) as usize).sum()
}

pub fn count_increases_part2(input: &[i64]) -> usize {
    input
        .windows(3)
        .map(|w| w.iter().sum())
        // We could use something more fancy to support this over arbitrary iterators but our data is small enough
        // that just collecting is probably faster
        .collect::<Vec<i64>>()
        .windows(2)
        .map(|w| (w[1] > w[0]) as usize)
        .sum()
}

pub fn load_input<P: AsRef<Path>>(path: P) -> Vec<i64> {
    let file = File::open(path).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect()
}
