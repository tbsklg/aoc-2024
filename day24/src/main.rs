use std::collections::{HashMap, VecDeque};
use std::io::Write;

use itertools::Itertools as _;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));

    // I solved day 24 part 2 manually by identifying misconfigured full adders.
    // These can be found in adder.svg at the following positions:
    // - x06, y06: XOR wvr jgq -> fkp (should be: XOR wvr jgp -> z06)
    // - z11, y11: XOR jpp stv -> ngr (should be: XOR jpp stv -> z11)
    // - x31, y32: XOR tpf mgq -> mfm (should be: XOR tpf mgq -> z31)
    // - x38, y38: XOR y38 x38 -> krj (should be: XOR y38 x38 -> bpt)

    let _ = part2(&input);
    println!("Part 2: {}", "bpt,fkp,krj,mfm,ngr,z06,z11,z31");
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

fn part2(input: &str) -> std::io::Result<()> {
    let mut sections = input.split("\n\n");
    let connections = extract_connections(sections.nth(1).unwrap()).unwrap();
    dot(connections, "adder.dot")
}

fn dot(connections: Vec<Connection>, filename: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(filename)?;

    writeln!(file, "stateDiagram-v2")?;

    for (i, connection) in connections.iter().enumerate() {
        let (gate, a, b, out) = match connection {
            Connection::AND(a, b, out) => ("AND", a, b, out),
            Connection::OR(a, b, out) => ("OR", a, b, out),
            Connection::XOR(a, b, out) => ("XOR", a, b, out),
        };

        let gate = format!("{}_{}", gate, i);

        writeln!(file, "{} -> {}", a, gate)?;
        writeln!(file, "{} -> {}", b, gate)?;
        writeln!(file, "{} -> {}", gate, out)?;
    }
    Ok(())
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
