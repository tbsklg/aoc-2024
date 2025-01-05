use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = Instant::now();
    let p1 = part1(&input);
    println!("Part 1: {} ({:?})", p1, now.elapsed());
}

type Pad = Vec<Vec<char>>;
type Pos = (i32, i32);
type Dir = (i32, i32);

const DIRS: [Dir; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn part1(input: &str) -> usize {
    let codes = extract_codes(input);
    let num_pad: Pad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['\0', '0', 'A'],
    ];
    let dir_pad: Pad = vec![vec!['\0', '^', 'A'], vec!['<', 'v', '>']];

    complexity(&num_pad, &dir_pad, codes)
}

fn build_pad_map(pad: &Pad) -> HashMap<char, Pos> {
    let mut map = HashMap::new();
    for (row_idx, row) in pad.iter().enumerate() {
        for (col_idx, &c) in row.iter().enumerate() {
            map.insert(c, (col_idx as i32, row_idx as i32));
        }
    }
    map
}

fn complexity(num_pad: &Pad, dir_pad: &Pad, codes: Vec<&str>) -> usize {
    codes
        .iter()
        .map(|code| extract_num(code) * translate_code(num_pad, dir_pad, code))
        .sum()
}

fn extract_num(code: &str) -> usize {
    let re = Regex::new(r"\d+").unwrap();

    re.captures(code)
        .and_then(|cap| cap.get(0))
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0)
}

fn translate_code(num_pad: &Pad, dir_pad: &Pad, code: &str) -> usize {
    let num_map = build_pad_map(num_pad);
    let dir_map = build_pad_map(dir_pad);

    let num_paths = shortest_paths(num_pad, (2, 3), code, &num_map);

    (0..=1)
        .fold(num_paths.clone(), |acc, _| {
            let next = acc
                .iter()
                .flat_map(|p| shortest_paths(dir_pad, (2, 0), &p, &dir_map))
                .collect();
            next
        })
        .into_iter()
        .map(|p| p.len())
        .min()
        .unwrap_or(0)
}

fn shortest_paths(
    pad: &Pad,
    start: Pos,
    code: &str,
    pad_map: &HashMap<char, Pos>,
) -> Vec<String> {
    code.chars()
        .fold((start, vec![String::new()]), |(start, all_paths), c| {
            let end = pad_map.get(&c).unwrap();

            let paths = shortest_path(pad, start, *end);

            let new_paths = all_paths
                .into_iter()
                .flat_map(|path| {
                    paths.iter().map(move |p| {
                        let mut new_path = path.clone();
                        new_path.push_str(p);
                        new_path
                    })
                })
                .collect::<Vec<String>>();

            (*end, new_paths)
        })
        .1
}

fn shortest_path(pad: &Vec<Vec<char>>, start: Pos, end: Pos) -> Vec<String> {
    let mut queue: VecDeque<(Pos, Vec<char>, HashSet<Pos>)> =
        VecDeque::from([(start, Vec::new(), HashSet::new())]);
    let mut paths: Vec<String> = vec![];
    let mut min_path = usize::MAX;

    while let Some((curr, mut path, mut visited)) = queue.pop_front() {
        if curr == end {
            if path.len() > min_path {
                break;
            }

            min_path = path.len();

            path.push('A');
            paths.push(path.iter().collect::<String>());

            continue;
        }

        if visited.contains(&curr) {
            continue;
        }
        visited.insert(curr);

        let neighbors = DIRS
            .iter()
            .filter_map(|dir| {
                let next = (curr.0 + dir.0, curr.1 + dir.1);

                if !visited.contains(&next) && get(pad, next).is_some_and(|c| c != '\0') {
                    let mut next_path = path.clone();
                    next_path.push(print_dir(*dir));
                    Some((next, next_path, visited.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for neighbor in neighbors {
            queue.push_back(neighbor);
        }
    }

    paths
}

fn print_dir(dir: Dir) -> char {
    match dir {
        (-1, 0) => '<',
        (1, 0) => '>',
        (0, -1) => '^',
        (0, 1) => 'v',
        _ => unreachable!(),
    }
}

fn get(num_pad: &Pad, pos: Pos) -> Option<char> {
    num_pad
        .get(pos.1 as usize)
        .and_then(|l| l.get(pos.0 as usize))
        .copied()
}

fn extract_codes(input: &str) -> Vec<&str> {
    input.lines().collect()
}
