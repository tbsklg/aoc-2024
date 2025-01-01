use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let patterns = extract_patterns(parts[0]);
    parts[1]
        .lines()
        .filter(|design| is_valid(design, &patterns))
        .count()
}

fn part2(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let patterns = extract_patterns(parts[0]);
    let mut memo = HashMap::new();

    parts[1]
        .lines()
        .map(|design| number_of_ways(design, &patterns, &mut memo))
        .sum()
}

fn number_of_ways<'a>(
    design: &'a str,
    patterns: &HashSet<&str>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if memo.contains_key(design) {
        return *memo.get(&design).unwrap();
    }

    let design_count: usize = patterns
        .iter()
        .map(|p| design.strip_prefix(p))
        .map(|d| d.map_or(0, |nd| number_of_ways(nd, patterns, memo)))
        .sum();

    memo.insert(design, design_count);
    design_count
}

fn is_valid(design: &str, patterns: &HashSet<&str>) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in patterns {
        if let Some(stripped) = design.strip_prefix(pattern) {
            if is_valid(stripped, patterns) {
                return true;
            }
        }
    }

    false
}

fn extract_patterns(input: &str) -> HashSet<&str> {
    input.split(", ").collect()
}
