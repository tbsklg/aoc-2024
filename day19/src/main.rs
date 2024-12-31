use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let patterns = extract_patterns(parts[0]);
    parts[1]
        .lines()
        .filter(|design| is_valid(design, &patterns))
        .count()
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
