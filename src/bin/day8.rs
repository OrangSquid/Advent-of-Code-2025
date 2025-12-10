use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use aoc_2025::readlines::read_lines;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

struct Node {
    x: f64,
    y: f64,
    z: f64,
}

impl Node {
    fn dist(&self, other: &Node) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0) + (self.z - other.z).powf(2.0))
            .sqrt()
    }
}

struct Edge {
    from: usize,
    to: usize,
    weight: f64,
}

impl Edge {
    fn new(from: usize, to: usize, weight: f64) -> Edge {
        Edge { from, to, weight }
    }
}

impl<'a> PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for Edge {}

impl<'a> PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<'a> Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.total_cmp(&other.weight)
    }
}

fn all_unified(union :&mut QuickUnionUf<UnionBySize>, size: usize) -> bool {
    let first_representative = union.find(0);
    (0..size)
        .all(|x| union.find(x) == first_representative)
}

fn main() {
    let lines = read_lines("input/day8.txt");

    let nodes: Vec<Node> = lines
        .map(|line| {
            let mut it = line.split(",");
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();
            let z = it.next().unwrap().parse().unwrap();
            Node { x, y, z }
        })
        .collect();

    let total_edges = nodes.len() * nodes.len();
    let mut weighted_edges = BinaryHeap::with_capacity(total_edges);
    for (idx_from, from) in nodes.iter().enumerate() {
        for (idx_to, to) in nodes[(idx_from + 1)..].iter().enumerate() {
            let weight = from.dist(to);
            weighted_edges.push(Reverse(Edge::new(idx_from, idx_to + idx_from + 1, weight)));
        }
    }

    let mut union = QuickUnionUf::<UnionBySize>::new(nodes.len());
    let mut i = 0;

    while let Some(Reverse(edge)) = weighted_edges.pop() && i < 1000 {
        union.union(edge.from, edge.to);
        i += 1;
    }

    let result_1: usize = (0..nodes.len())
        .map(|x| union.get(x).size())
        .collect::<BTreeSet<usize>>()
        .iter()
        .rev()
        .take(3)
        .product();

    let mut last_edge = Edge { from: 0, to: 0, weight: 0.0 };

    while let Some(Reverse(edge)) = weighted_edges.pop() && !all_unified(&mut union, nodes.len()) {
        union.union(edge.from, edge.to);
        last_edge = edge;
    }

    let result_2 = nodes[last_edge.from].x * nodes[last_edge.to].x;

    println!("{result_1}");
    println!("{result_2}");
}
