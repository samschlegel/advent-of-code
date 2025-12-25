use std::{
    collections::{HashMap, HashSet},
    hash::DefaultHasher,
};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> HashMap<String, HashSet<String>> {
    input
        .lines()
        .map(|line| {
            let (first, rest) = line.trim().split_once(':').unwrap();
            (
                first.to_string(),
                rest.split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &HashMap<String, HashSet<String>>) -> usize {
    // Count total number of paths from 'you' to 'out'
    let mut visited = HashSet::<String>::new();
    let mut count = 0;

    fn dfs(
        input: &HashMap<String, HashSet<String>>,
        node: &str,
        visited: &mut HashSet<String>,
        count: &mut usize,
    ) {
        if node == "out" {
            *count += 1;
            return;
        }
        if visited.contains(node) {
            return;
        }
        visited.insert(node.to_string());
        for neighbor in input.get(node).unwrap() {
            dfs(input, neighbor, visited, count);
        }
        visited.remove(node);
    }

    dfs(input, "you", &mut visited, &mut count);

    count
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &HashMap<String, HashSet<String>>) -> String {
    String::new()
}
