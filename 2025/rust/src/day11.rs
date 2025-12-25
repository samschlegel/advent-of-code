use std::{
    collections::{HashMap, HashSet},
    convert::identity,
    hash::{DefaultHasher, RandomState},
};

use aoc_runner_derive::{aoc, aoc_generator};
use fixedbitset::FixedBitSet;
use petgraph::{
    algo::{all_simple_paths, is_cyclic_directed},
    prelude::*,
    visit::{VisitMap, Visitable, Walker},
};

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> (DiGraph<String, ()>, HashMap<String, NodeIndex>) {
    let mut graph = DiGraph::new();
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

fn reachable_nodes(graph: &DiGraph<String, ()>, from: &[NodeIndex]) -> FixedBitSet {
    let mut dfs = Dfs::empty(&graph);
    for &node in from {
        dfs.move_to(node);
        while let Some(_) = dfs.next(&graph) {
            continue;
        }
    }
    dfs.discovered
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &(DiGraph<String, ()>, HashMap<String, NodeIndex>)) -> usize {
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
        reachable.intersect_with(&reachable_nodes(&revgraph, &[node]));
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
    let min_intermediate_nodes = 0;
    let max_intermediate_nodes = None;

    let mut total_length = 0;
    let mut count = 0;

    for path in all_simple_paths::<Vec<_>, _, RandomState>(
        &graph,
        *node_map.get("svr").unwrap(),
        *node_map.get("out").unwrap(),
        min_intermediate_nodes,
        max_intermediate_nodes,
    ) {
        total_length += path.len();
        count += 1;
        if count % 1000 == 0 {
            println!("Processed {} paths", count);
            println!("Average path length: {}", total_length / count);
        }
    }
    count
}
