use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let patterns = extract_patterns(parts[0]);
    parts[1]
        .lines()
        .filter(|line| {
            let mut queue = vec![line.to_string()];

            while let Some(curr) = queue.pop() {
                if curr.is_empty() {
                    return true;
                }

                let head = curr.chars().next().unwrap_or_default();
                let matching_patterns = match patterns.get(&head) {
                    Some(values) => values
                        .iter()
                        .filter(|v| curr.starts_with(*v))
                        .collect::<Vec<_>>(),
                    None => vec![],
                };

                if matching_patterns.is_empty() {
                    return false;
                }

                queue.extend(
                    matching_patterns
                        .iter()
                        .map(|v| curr.strip_prefix(*v).unwrap().to_string()),
                );
            }

            true
        })
        .count()
}

fn extract_patterns(input: &str) -> HashMap<char, Vec<String>> {
    input.split(", ").fold(HashMap::new(), |mut acc, curr| {
        acc.entry(curr.chars().next().unwrap())
            .and_modify(|e| e.push(curr.to_string()))
            .or_insert(vec![curr.to_string()]);
        acc
    })
}
