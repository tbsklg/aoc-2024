use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::OnceLock,
};

use cached::proc_macro::cached;
use itertools::Itertools;

#[derive(Clone, Eq, Hash, PartialEq)]
enum PathType {
    Numeric,
    Directional,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            find_shortest_sequence(line.to_string(), 2, PathType::Numeric)
                * line.trim_end_matches('A').parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

// Solved with the help of  @icub3d
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            find_shortest_sequence(line.to_string(), 25, PathType::Numeric)
                * line.trim_end_matches('A').parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

fn numeric_paths() -> &'static HashMap<(char, char), Vec<String>> {
    static NUMERIC_PATHS: OnceLock<HashMap<(char, char), Vec<String>>> = OnceLock::new();
    NUMERIC_PATHS.get_or_init(|| {
        let numeric_keypad = vec![
            ('7', vec![('4', 'v'), ('8', '>')]),
            ('8', vec![('5', 'v'), ('9', '>'), ('7', '<')]),
            ('9', vec![('6', 'v'), ('8', '<')]),
            ('4', vec![('1', 'v'), ('5', '>'), ('7', '^')]),
            ('5', vec![('2', 'v'), ('6', '>'), ('4', '<'), ('8', '^')]),
            ('6', vec![('3', 'v'), ('5', '<'), ('9', '^')]),
            ('1', vec![('2', '>'), ('4', '^')]),
            ('2', vec![('3', '>'), ('5', '^'), ('1', '<'), ('0', 'v')]),
            ('0', vec![('2', '^'), ('A', '>')]),
            ('3', vec![('6', '^'), ('2', '<'), ('A', 'v')]),
            ('A', vec![('0', '<'), ('3', '^')]),
        ]
        .into_iter()
        .collect::<HashMap<char, Vec<(char, char)>>>();

        numeric_keypad
            .keys()
            .cartesian_product(numeric_keypad.keys())
            .map(|(&a, &b)| ((a, b), shortest_path(&numeric_keypad, a, b)))
            .collect()
    })
}

fn direction_paths() -> &'static HashMap<(char, char), Vec<String>> {
    static DIRECTION_PATHS: OnceLock<HashMap<(char, char), Vec<String>>> = OnceLock::new();
    DIRECTION_PATHS.get_or_init(|| {
        let direction_keypad = vec![
            ('^', vec![('A', '>'), ('v', 'v')]),
            ('A', vec![('^', '<'), ('>', 'v')]),
            ('>', vec![('A', '^'), ('v', '<')]),
            ('<', vec![('v', '>')]),
            ('v', vec![('<', '<'), ('^', '^'), ('>', '>')]),
        ]
        .into_iter()
        .collect::<HashMap<char, Vec<(char, char)>>>();

        direction_keypad
            .keys()
            .cartesian_product(direction_keypad.keys())
            .map(|(&a, &b)| ((a, b), shortest_path(&direction_keypad, a, b)))
            .collect()
    })
}

fn shortest_path(
    neighbors: &HashMap<char, Vec<(char, char)>>,
    start: char,
    end: char,
) -> Vec<String> {
    let mut queue = VecDeque::new();
    queue.push_back((start, Vec::new(), HashSet::new()));

    let mut paths = Vec::new();
    let mut min_path = usize::MAX;

    while let Some((curr, path, mut visited)) = queue.pop_front() {
        if curr == end {
            if path.len() <= min_path {
                min_path = path.len();
                paths.push(path.iter().collect::<String>());
            }
            continue;
        }

        if visited.contains(&curr) {
            continue;
        }

        visited.insert(curr);

        for (next, dir) in neighbors.get(&curr).unwrap() {
            let mut path = path.clone();
            path.push(*dir);
            queue.push_back((*next, path, visited.clone()));
        }
    }

    paths
}

#[cached]
fn find_shortest_sequence(sequence: String, depth: usize, path_type: PathType) -> usize {
    let paths = match path_type {
        PathType::Numeric => numeric_paths(),
        PathType::Directional => direction_paths(),
    };

    ("A".to_string() + &sequence)
        .chars()
        .tuple_windows()
        .map(|(a, b)| {
            let shortest_paths = paths.get(&(a, b)).unwrap();
            match depth {
                0 => shortest_paths[0].len() + 1,
                _ => shortest_paths
                    .iter()
                    .cloned()
                    .map(|mut path| {
                        path.push('A');
                        find_shortest_sequence(path, depth - 1, PathType::Directional)
                    })
                    .min()
                    .unwrap(),
            }
        })
        .sum::<usize>()
}
