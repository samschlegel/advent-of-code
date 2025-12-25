use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use fixedbitset::FixedBitSet;
use petgraph::{algo::toposort, prelude::*};

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

    count_paths_using_toposort(graph, &toposort(graph, None).unwrap(), start, end)
}

fn count_paths_using_toposort<N, E>(
    graph: &StableDiGraph<N, E>,
    toposort: &Vec<NodeIndex>,
    start: NodeIndex,
    end: NodeIndex,
) -> usize {
    let mut paths = HashMap::new();
    paths.insert(start, 1);

    for &node in toposort {
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
    count_paths(graph, node_map["you"], node_map["out"])
}

#[aoc(day11, part2, reduction)]
pub fn solve_part2(input: &(StableDiGraph<String, ()>, HashMap<String, NodeIndex>)) -> usize {
    let (graph, node_map) = input;
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
    // Sanity check that the graph contains svr and out
    count_paths(
        &graph,
        *node_map.get("svr").unwrap(),
        *node_map.get("out").unwrap(),
    )
}

#[aoc(day11, part2, mult)]
pub fn solve_part2_mult(input: &(StableDiGraph<String, ()>, HashMap<String, NodeIndex>)) -> usize {
    let (graph, node_map) = input;
    let svr_fft = count_paths(&graph, node_map["svr"], node_map["fft"]);
    let fft_dac = count_paths(&graph, node_map["fft"], node_map["dac"]);
    let dac_out = count_paths(&graph, node_map["dac"], node_map["out"]);

    svr_fft * fft_dac * dac_out
}

#[aoc(day11, part2, mult_shared_topo)]
pub fn solve_part2_mult_shared_topo(
    input: &(StableDiGraph<String, ()>, HashMap<String, NodeIndex>),
) -> usize {
    let (graph, node_map) = input;
    let topo = toposort(graph, None).unwrap();
    let svr_fft = count_paths_using_toposort(&graph, &topo, node_map["svr"], node_map["fft"]);
    let fft_dac = count_paths_using_toposort(&graph, &topo, node_map["fft"], node_map["dac"]);
    let dac_out = count_paths_using_toposort(&graph, &topo, node_map["dac"], node_map["out"]);

    svr_fft * fft_dac * dac_out
}
