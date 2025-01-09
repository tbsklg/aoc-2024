use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

use itertools::Itertools as _;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let mut sections = input.split("\n\n");
    let wires = &mut extract_wires(sections.next().unwrap()).unwrap();
    let connections = &mut VecDeque::from(extract_connections(sections.next().unwrap()).unwrap());

    process(wires, connections);

    usize::from_str_radix(
        &wires
            .iter()
            .filter(|(k, _)| k.starts_with("z"))
            .sorted()
            .map(|(_, &v)| if v { '1' } else { '0' })
            .rev()
            .collect::<String>(),
        2,
    )
    .unwrap()
}

fn process<'a>(wires: &mut HashMap<&'a str, bool>, connections: &mut VecDeque<Connection<'a>>) {
    while let Some(connection) = connections.pop_front() {
        let result = match connection {
            Connection::AND(a, b, r) => {
                if let (Some(&v1), Some(&v2)) = (wires.get(a), wires.get(b)) {
                    wires.insert(r, Gates::AND(v1, v2).exec());
                    None
                } else {
                    Some(connection)
                }
            }
            Connection::OR(a, b, r) => {
                if let (Some(&v1), Some(&v2)) = (wires.get(a), wires.get(b)) {
                    wires.insert(r, Gates::OR(v1, v2).exec());
                    None
                } else {
                    Some(connection)
                }
            }
            Connection::XOR(a, b, r) => {
                if let (Some(&v1), Some(&v2)) = (wires.get(a), wires.get(b)) {
                    wires.insert(r, Gates::XOR(v1, v2).exec());
                    None
                } else {
                    Some(connection)
                }
            }
        };

        if let Some(conn) = result {
            connections.push_back(conn);
        }
    }
}

fn extract_wires(input: &str) -> Result<HashMap<&str, bool>, &'static str> {
    input
        .lines()
        .map(|line| {
            let (wire, value) = line.split_once(": ").ok_or("Invalid line format")?;
            let bool_value = value.trim() == "1";
            Ok((wire, bool_value))
        })
        .collect()
}

fn extract_connections(input: &str) -> Result<Vec<Connection>, &'static str> {
    let results: Result<Vec<_>, _> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split("->").collect();
            if parts.len() != 2 {
                return Err("Invalid format: missing or too many arrows");
            }

            let left = parts[0].trim();
            let right = parts[1].trim();

            if left.contains("AND") {
                let operands: Vec<&str> = left.split("AND").collect();
                Ok(Connection::AND(
                    operands[0].trim(),
                    operands[1].trim(),
                    right,
                ))
            } else if left.contains("XOR") {
                let operands: Vec<&str> = left.split("XOR").collect();
                Ok(Connection::XOR(
                    operands[0].trim(),
                    operands[1].trim(),
                    right,
                ))
            } else if left.contains("OR") {
                let operands: Vec<&str> = left.split("OR").collect();
                Ok(Connection::OR(
                    operands[0].trim(),
                    operands[1].trim(),
                    right,
                ))
            } else {
                Err("Invalid operation")
            }
        })
        .collect();

    results
}

#[derive(Debug)]
enum Connection<'a> {
    AND(&'a str, &'a str, &'a str),
    OR(&'a str, &'a str, &'a str),
    XOR(&'a str, &'a str, &'a str),
}

enum Gates {
    AND(bool, bool),
    OR(bool, bool),
    XOR(bool, bool),
}

impl Gates {
    fn exec(&self) -> bool {
        match self {
            Gates::AND(a, b) => *a && *b,
            Gates::OR(a, b) => *a || *b,
            Gates::XOR(a, b) => *a ^ *b,
        }
    }
}
