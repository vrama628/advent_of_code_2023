use std::{io::stdin, str::FromStr};

use im::{HashMap, HashSet};
use itertools::Itertools;

type Node = String;

struct Graph {
    nodes: HashSet<Node>,
    edges: HashMap<Node, HashSet<Node>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn add_edge(&self, edge: (Node, Node)) -> Self {
        let nodes = self.nodes.update(edge.0.clone()).update(edge.1.clone());
        let new_edges = vec![
            (edge.0.clone(), HashSet::unit(edge.1.clone())),
            (edge.1.clone(), HashSet::unit(edge.0.clone())),
        ]
        .into();
        let edges = self.edges.clone().union_with(new_edges, |a, b| a.union(b));
        Self { nodes, edges }
    }

    fn edges(&self) -> Vec<(Node, Node)> {
        self.edges
            .iter()
            .flat_map(|(node, neighbors)| {
                neighbors
                    .iter()
                    .map(|neighbor| (node.clone(), neighbor.clone()))
            })
            .filter(|(a, b)| a < b)
            .collect()
    }

    fn remove_edge(&self, edge: (Node, Node)) -> Self {
        let nodes = self.nodes.clone();
        let edges = self
            .edges
            .alter(
                |nodes| nodes.map(|nodes| nodes.without(&edge.1)),
                edge.0.clone(),
            )
            .alter(
                |nodes| nodes.map(|nodes| nodes.without(&edge.0)),
                edge.1.clone(),
            );
        Self { nodes, edges }
    }

    fn component(&self) -> usize {
        let mut seen = HashSet::new();
        let mut stack = vec![];
        let first_node = self.nodes.iter().next().unwrap();
        seen.insert(first_node);
        stack.push(first_node);
        while let Some(node) = stack.pop() {
            for neighbor in self.edges.get(node).unwrap() {
                if !seen.contains(neighbor) {
                    seen.insert(neighbor);
                    stack.push(neighbor);
                }
            }
        }
        seen.len()
    }
}

struct Input(Graph);

impl Input {
    fn parse() -> Self {
        Self(
            stdin()
                .lines()
                .map(|line| line.unwrap())
                .fold(Graph::new(), |acc, line| {
                    let (node, neighbors) = line.split_once(": ").unwrap();
                    neighbors
                        .split_ascii_whitespace()
                        .fold(acc, |acc, neighbor| {
                            acc.add_edge((node.to_owned(), neighbor.to_owned()))
                        })
                }),
        )
    }

    fn solve(&self) -> usize {
        self.0
            .edges()
            .into_iter()
            .tuple_combinations()
            .into_iter()
            .find_map(|(a, b, c)| {
                let component = self
                    .0
                    .remove_edge(a)
                    .remove_edge(b)
                    .remove_edge(c)
                    .component();
                (component < self.0.nodes.len())
                    .then(|| component * (self.0.nodes.len() - component))
            })
            .unwrap()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
