use std::{
    collections::{HashMap, VecDeque},
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
    let num_paths = shortest_paths(num_pad, (2, 3), code);

    (0..=1)
        .fold(num_paths.clone(), |acc, _| {
            let next = acc
                .iter()
                .flat_map(|p| {
                    shortest_paths(dir_pad, (2, 0), &p.iter().collect::<String>())
                })
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
) -> Vec<Vec<char>> {
    let num_map = build_pad_map(pad);

    code.chars()
        .fold((start, vec![vec![]]), |(start, all_paths), c| {
            let end = num_map.get(&c).unwrap();

            let paths = shortest_path(pad, start, *end);
            let new_paths = all_paths
                .into_iter()
                .flat_map(|route| {
                    paths.iter().map(move |p| {
                        let mut new_route = route.clone();
                        new_route.extend(p.clone());
                        new_route
                    })
                })
                .collect::<Vec<_>>();

            (*end, new_paths)
        })
        .1
}

fn shortest_path(pad: &Vec<Vec<char>>, start: Pos, end: Pos) -> Vec<Vec<char>> {
    let mut queue: VecDeque<(Pos, Vec<(Pos, Dir)>)> = VecDeque::from([(start, vec![])]);
    let mut paths: Vec<Vec<char>> = vec![];
    let mut min_path = usize::MAX;

    while let Some((curr, path)) = queue.pop_front() {
        if curr == end {
            if path.len() > min_path {
                break;
            }

            min_path = path.len();

            let mut dirs = path.iter().map(|p| print_dir(p.1)).collect::<Vec<_>>();
            dirs.push('A');
            paths.push(dirs);

            continue;
        }

        let neighbors = DIRS
            .iter()
            .filter_map(|dir| {
                let next = (curr.0 + dir.0, curr.1 + dir.1);
                let visited = path.iter().map(|p| p.0).collect::<Vec<_>>();

                if !visited.contains(&next) && get(pad, next).is_some_and(|c| c != '\0') {
                    let mut next_path = path.clone();
                    next_path.push((next, *dir));
                    Some((next, next_path))
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
