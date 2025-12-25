use std::{
    collections::{HashMap, HashSet},
    convert::identity,
    fs::File,
    hash::{DefaultHasher, RandomState},
    io::Write,
};

use aoc_runner_derive::{aoc, aoc_generator};
use fixedbitset::FixedBitSet;
use petgraph::{
    algo::{all_simple_paths, toposort},
    dot::{Config, Dot},
    prelude::*,
};

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> (StableDiGraph<String, ()>, HashMap<String, NodeIndex>) {
    let mut graph = StableDiGraph::new();
    let mut node_map = HashMap::new();

    for line in input.lines() {
        let (first, rest) = line.trim().split_once(':').unwrap();
        let node = *node_map
            .entry(first.to_string())
            .or_insert_with(|| graph.add_node(first.to_string()));
        for neighbor in rest.split_ascii_whitespace() {
            let neighbor_node = if node_map.contains_key(neighbor) {
                *node_map.get(neighbor).unwrap()
            } else {
                let new_node = graph.add_node(neighbor.to_string());
                node_map.insert(neighbor.to_string(), new_node);
                new_node
            };
            graph.add_edge(node, neighbor_node, ());
        }
    }

    (graph, node_map)
}

fn reachable_nodes(graph: &StableDiGraph<String, ()>, from: &[NodeIndex]) -> FixedBitSet {
    let mut dfs = Dfs::empty(&graph);
    for &node in from {
        dfs.move_to(node);
        while let Some(_) = dfs.next(&graph) {
            continue;
        }
    }
    dfs.discovered
}

fn count_paths<N, E>(graph: &StableDiGraph<N, E>, start: NodeIndex, end: NodeIndex) -> usize {
    let mut paths = HashMap::new();
    paths.insert(start, 1);

    for node in toposort(graph, None).unwrap() {
        if node == end {
            return paths[&end];
        }
        let current = *paths.get(&node).unwrap_or(&0);
        for neighbor in graph.neighbors(node) {
            paths
                .entry(neighbor)
                .and_modify(|e| *e += current)
                .or_insert(current);
        }
    }
    0
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &(StableDiGraph<String, ()>, HashMap<String, NodeIndex>)) -> usize {
    let (graph, node_map) = input;
    println!("Graph has {} nodes", graph.node_count());
    count_paths(graph, node_map["you"], node_map["out"])
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &(StableDiGraph<String, ()>, HashMap<String, NodeIndex>)) -> usize {
    let (graph, node_map) = input;
    println!("Graph has {} nodes", graph.node_count());
    let mut revgraph = graph.clone();
    revgraph.reverse();
    let mut reachable = FixedBitSet::with_capacity(graph.node_count());
    reachable.set_range(.., true);
    let from = ["fft", "dac", "out"]
        .iter()
        .flat_map(|&s| node_map.get(s).cloned())
        .collect::<Vec<_>>();
    for &node in from.iter() {
        let mut sub_reach = reachable_nodes(&graph, &[node]);
        sub_reach.union_with(&reachable_nodes(&revgraph, &[node]));
        reachable.intersect_with(&sub_reach);
    }
    println!(
        "Accessible nodes from '{:?}': {:?}",
        from,
        reachable.count_ones(..)
    );
    let graph = graph.filter_map(
        |v, w| {
            if reachable.contains(v.index()) {
                Some(w)
            } else {
                None
            }
        },
        |_, w| Some(w),
    );
    println!("Graph size after reduction: {}", graph.node_count());
    // Sanity check that the graph contains svr and out
    count_paths(
        &graph,
        *node_map.get("svr").unwrap(),
        *node_map.get("out").unwrap(),
    )
}
