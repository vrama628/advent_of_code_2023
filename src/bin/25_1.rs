use std::{collections::VecDeque, io::stdin, str::FromStr};

use im::{HashMap, HashSet};
use itertools::Itertools;

type Node = String;
type Edge = (Node, Node);

#[derive(Clone)]
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

    fn add_edge(&self, edge: Edge) -> Self {
        let nodes = self.nodes.update(edge.0.clone()).update(edge.1.clone());
        let new_edges = vec![
            (edge.0.clone(), HashSet::unit(edge.1.clone())),
            (edge.1.clone(), HashSet::unit(edge.0.clone())),
        ]
        .into();
        let edges = self.edges.clone().union_with(new_edges, |a, b| a.union(b));
        Self { nodes, edges }
    }

    fn find_path(&self, start: Node, end: Node) -> Vec<Edge> {
        let mut seen = HashSet::unit(start.clone());
        let mut queue: VecDeque<(Node, Vec<Edge>)> = VecDeque::from([(start.clone(), vec![])]);
        while let Some((node, path)) = queue.pop_front() {
            for neighbor in self.edges.get(&node).unwrap() {
                if !seen.contains(neighbor) {
                    let mut new_path = path.clone();
                    new_path.push((node.clone(), neighbor.clone()));
                    if neighbor == &end {
                        return new_path;
                    }
                    seen.insert(neighbor.clone());
                    queue.push_back((neighbor.clone(), new_path));
                }
            }
        }
        panic!("No path found")
    }

    fn remove_edge(&self, edge: Edge) -> Self {
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

    fn component_sizes(&self) -> Vec<usize> {
        let mut unseen = self.nodes.clone();
        let mut component_sizes = vec![];
        while let Some(start) = unseen.iter().next() {
            let mut seen = HashSet::unit(start.clone());
            let mut stack = vec![start.clone()];
            while let Some(node) = stack.pop() {
                for neighbor in self.edges.get(&node).unwrap() {
                    if !seen.contains(neighbor) {
                        seen.insert(neighbor.clone());
                        stack.push(neighbor.clone());
                    }
                }
            }
            component_sizes.push(seen.len());
            unseen = unseen.relative_complement(seen)
        }
        component_sizes
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
        for (start, end) in self
            .0
            .nodes
            .iter()
            .collect_vec()
            .into_iter()
            .tuple_combinations()
        {
            let graph = self.0.clone();
            let graph = graph
                .find_path(start.clone(), end.clone())
                .into_iter()
                .fold(graph, |acc, edge| acc.remove_edge(edge));
            let graph = graph
                .find_path(start.clone(), end.clone())
                .into_iter()
                .fold(graph, |acc, edge| acc.remove_edge(edge));
            let graph = graph
                .find_path(start.clone(), end.clone())
                .into_iter()
                .fold(graph, |acc, edge| acc.remove_edge(edge));
            let component_sizes = graph.component_sizes();
            if component_sizes.len() == 2 {
                return component_sizes[0] * component_sizes[1];
            }
        }
        panic!("No cut found")
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
