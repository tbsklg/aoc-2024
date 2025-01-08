use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let mut sections = input.split("\n\n");
    let wires = extract_wires(sections.next().unwrap());
    let connections = extract_connections(sections.next().unwrap());

    println!("{:?}", connections);
    0
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
