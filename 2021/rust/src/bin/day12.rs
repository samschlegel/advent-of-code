use std::{collections::HashMap, io};

use aoc_2021::util::all_paths;
use itertools::Itertools;
use petgraph::graphmap::UnGraphMap;

const INPUT: &str = include_str!("../../../inputs/day12_test.txt");
// const INPUT: &str = include_str!("../../../inputs/day12.txt");

fn str_is_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

fn get_weight(s: &str) -> f64 {
    match s {
        "start" | "end" => 1.0,
        s if str_is_uppercase(s) => f64::INFINITY,
        _ => 2.0,
    }
}

fn parse(s: &str) -> (UnGraphMap<&str, ()>, HashMap<&str, f64>) {
    let mut g = UnGraphMap::new();
    let mut node_weights = HashMap::new();
    for line in s.trim().lines() {
        let (from, to) = line.split_once('-').unwrap();
        node_weights.insert(from, get_weight(from));
        node_weights.insert(to, get_weight(to));
        g.add_edge(from, to, ());
    }
    (g, node_weights)
}

fn main() -> io::Result<()> {
    let (g, node_weights) = parse(INPUT);
    let mut count = 0;
    for path in all_paths::<Vec<_>, _>(&g, node_weights, "start", "end") {
        println!("{}", path.join(","));
        count += 1;
    }
    // println!("{}", all_paths.iter().map(|p| p.join(",")).join("\n"));
    println!("{}", count);

    Ok(())
}
