use itertools::Itertools as _;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let connections = extract_connections(input);

    let graph: HashMap<&str, Vec<&str>> =
        connections.iter().fold(HashMap::new(), |mut acc, &(a, b)| {
            acc.entry(a).or_insert_with(Vec::new).push(b);
            acc.entry(b).or_insert_with(Vec::new).push(a);
            acc
        });

    find_triangles(&graph).iter().filter(|c| c.starts_with_t()).count()
}

#[derive(Debug, Eq, Hash)]
struct Clique {
    a: String,
    b: String,
    c: String,
}

impl From<(&str, &str, &str)> for Clique {
    fn from((a, b, c): (&str, &str, &str)) -> Self {
        let mut nodes = vec![a.to_string(), b.to_string(), c.to_string()];
        nodes.sort();
        Self {
            a: nodes[0].clone(),
            b: nodes[1].clone(),
            c: nodes[2].clone(),
        }
    }
}

impl Clique {
    fn starts_with_t(&self) -> bool {
        self.a.starts_with("t") || self.b.starts_with("t") || self.c.starts_with("t")
    }
}

impl PartialEq for Clique {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}

fn find_triangles(graph: &HashMap<&str, Vec<&str>>) -> HashSet<Clique> {
    graph
        .iter()
        .flat_map(|(k, neighbors)| {
            neighbors.iter().tuple_combinations().filter_map(move |(&a, &b)| {
                if graph.get(a).unwrap_or(&Vec::new()).contains(&b) {
                    Some(Clique::from((*k, a, b)))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn extract_connections(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|l| {
            let mut split_by_minus = l.split("-");
            (
                split_by_minus.next().unwrap(),
                split_by_minus.next().unwrap(),
            )
        })
        .collect()
}
