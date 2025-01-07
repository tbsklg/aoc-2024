use itertools::Itertools as _;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

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

    let x = find_triangles(&graph);
        
    println!("{:?}", x.len());
    x.iter().filter(|c| c.starts_with_t()).count()
}

#[derive(Debug, Eq)]
struct Clique {
    a: String,
    b: String,
    c: String,
}

impl From<(&str, &str, &str)> for Clique {
    fn from((a, b, c): (&str, &str, &str)) -> Self {
        Self {
            a: a.to_string(),
            b: b.to_string(),
            c: c.to_string(),
        }
    }
}

impl Clique {
    fn starts_with_t(&self) -> bool {
        self.a.starts_with("t") || self.b.starts_with("t") || self.c.starts_with("t")
    }
}

impl Hash for Clique {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut fields = vec![&self.a, &self.b, &self.c];
        fields.sort();
        for field in fields {
            field.hash(state);
        }
    }
}

impl PartialEq for Clique {
    fn eq(&self, other: &Self) -> bool {
        let mut self_fields = vec![&self.a, &self.b, &self.c];
        let mut other_fields = vec![&other.a, &other.b, &other.c];
        self_fields.sort();
        other_fields.sort();
        self_fields == other_fields
    }
}

fn find_triangles(graph: &HashMap<&str, Vec<&str>>) -> HashSet<Clique> {
    graph
        .into_iter()
        .flat_map(|(k, v)| {
            v.iter().tuple_windows().filter_map(move |(a, b)| {
                if let (Some(v1), Some(v2)) = (graph.get(a), graph.get(b)) {
                    if v1.contains(b) && v2.contains(a) {
                        Some(Clique::from((*k, *a, *b)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<Clique>>()
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
        .collect::<Vec<_>>()
}
