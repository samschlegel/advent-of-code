use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::{hash::Hash, iter::from_fn};

use itertools::Itertools;
use petgraph::visit::IntoNeighbors;

use super::str::str_is_uppercase;

pub fn is_small_room(s: &str) -> bool {
    if s == "start" || s == "end" {
        false
    } else {
        s.chars().all(|c| c.is_uppercase())
    }
}

fn has_visited_small_room_twice<S: AsRef<str>>(visited: &Vec<S>) -> bool {
    visited
        .iter()
        .filter(|r| is_small_room(r.as_ref()))
        .counts_by(|r| r.as_ref())
        .values()
        .any(|&c| c == 2)
}

pub struct Paths<'a, C, G>
where
    G: IntoNeighbors,
    G::NodeId: Hash + Eq + AsRef<str>,
    C: FromIterator<G::NodeId>,
{
    graph: &'a G,
    to: G::NodeId,

    visited: Vec<G::NodeId>,
    visit_counts: HashMap<G::NodeId, (i32, i32)>,
    stack: Vec<G::Neighbors>,

    _phantom: PhantomData<C>,
}

impl<'a, C, G> Paths<'a, C, G>
where
    G: IntoNeighbors,
    G::NodeId: Hash + Eq + AsRef<str>,
    C: FromIterator<G::NodeId>,
{
    pub fn new(graph: &'a G, from: G::NodeId, to: G::NodeId) -> Self {
        let mut p = Paths {
            graph,
            visited: Vec::default(),
            visit_counts: HashMap::default(),
            stack: Vec::default(),
            max_small_node_dupes,
            to,
            _phantom: PhantomData::default(),
        };
        p.visit(from);
        p
    }

    fn node_weight(node: G::NodeId) -> f64 {
        let s = node.as_ref();
        match s {
            "start" | "end" => 1.0,
            s if str_is_uppercase(s) => f64::INFINITY,
            _ => 2.0,
        }
    }

    fn visit(&mut self, node: G::NodeId) {
        self.visited.push(node);
        self.stack.push(self.graph.neighbors(node));
    }

    fn leave(&mut self) -> Option<()> {
        let left_node = self.visited.pop()?;
        self.stack.pop();
        Some(())
    }
}

impl<'a, C, G> Iterator for Paths<'a, C, G>
where
    G: IntoNeighbors,
    G::NodeId: Hash + Eq + AsRef<str>,
    C: FromIterator<G::NodeId>,
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(children) = self.stack.last_mut() {
            if let Some(child) = children.next() {
                if child == self.to {
                    let path = self
                        .visited
                        .iter()
                        .cloned()
                        .chain(Some(self.to))
                        .collect::<C>();
                    return Some(path);
                } else if let Some((visits, max)) = self.visit_counts.get_mut(&child) {
                    if *weight > 0.0
                        && !(is_small_room(child.as_ref())
                            && has_visited_small_room_twice(&visited_vec))
                    {
                        visited_vec.push(child);
                        *weight -= 1.0;
                        stack.push(graph.neighbors(child));
                    }
                } else {
                    visited_vec.push(child);
                    visited_weights.insert(child, node_weights[&child] - 1.0);
                    stack.push(graph.neighbors(child));
                }
            } else {
                stack.pop();
                let last_node = visited_vec.pop().unwrap();
                *visited_weights.get_mut(&last_node).unwrap() += 1.0;
            }
        }
        None
    }
}

pub fn all_paths<'a, TargetColl, G>(
    graph: G,
    node_weights: HashMap<G::NodeId, f64>,
    from: G::NodeId,
    to: G::NodeId,
) -> impl Iterator<Item = TargetColl>
where
    G: IntoNeighbors,
    G::NodeId: Hash + Eq + AsRef<str>,
    TargetColl: FromIterator<G::NodeId>,
{
    // list of visited nodes
    let mut visited_vec = vec![from];
    let mut visited_weights = HashMap::new();
    visited_weights.insert(from, 0.0);
    // list of childs of currently exploring path nodes,
    // last elem is list of childs of last visited node
    let mut stack = vec![graph.neighbors(from)];

    from_fn(move || {
        while let Some(children) = stack.last_mut() {
            if let Some(child) = children.next() {
                if child == to {
                    let path = visited_vec
                        .iter()
                        .cloned()
                        .chain(Some(to))
                        .collect::<TargetColl>();
                    return Some(path);
                } else if let Some(weight) = visited_weights.get_mut(&child) {
                    if *weight > 0.0
                        && !(is_small_room(child.as_ref())
                            && has_visited_small_room_twice(&visited_vec))
                    {
                        visited_vec.push(child);
                        *weight -= 1.0;
                        stack.push(graph.neighbors(child));
                    }
                } else {
                    visited_vec.push(child);
                    visited_weights.insert(child, node_weights[&child] - 1.0);
                    stack.push(graph.neighbors(child));
                }
            } else {
                stack.pop();
                let last_node = visited_vec.pop().unwrap();
                *visited_weights.get_mut(&last_node).unwrap() += 1.0;
            }
        }
        None
    })
}
