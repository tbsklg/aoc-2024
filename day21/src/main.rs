use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

type NumPad = [[char; 3]; 4];
type DirPad = [[char; 3]; 2];
type Pos = (i32, i32);
type Dir = (i32, i32);

const DIRS: [Dir; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn part1(input: &str) -> usize {
    let codes = extract_codes(input);
    let num_pad: NumPad = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        ['\0', '0', 'A'],
    ];
    let dir_pad: DirPad = [['\0', '^', 'A'], ['<', 'v', '>']];

    let paths = shortest_path(&num_pad, (2, 3), '2');

    println!("{:?}", paths);
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    dir: Dir,
    steps: usize,
}

fn shortest_paths(num_pad: &[[char; 3]; 4], start: Pos, code: &str) -> Vec<Vec<(Pos, Dir)>> {
    let mut paths = vec![];
    let mut start = start;

    paths
}

fn shortest_path(num_pad: &[[char; 3]; 4], start: Pos, end: char) -> HashMap<Pos, Vec<Vec<Dir>>> {
    let mut queue: VecDeque<(Pos, Vec<(Pos, Dir)>)> = VecDeque::from([(start, vec![])]);
    let mut paths: HashMap<Pos, Vec<Vec<Dir>>> = HashMap::new();
    let mut min_path = usize::MAX;

    while let Some((curr, path)) = queue.pop_front() {
        if get(num_pad, curr) == Some(end) {
            if path.len() > min_path {
                break;
            }

            min_path = path.len();

            let dirs = path.iter().map(|p| p.1).collect::<Vec<_>>();
            paths
                .entry(curr)
                .and_modify(|p| p.push(dirs.clone()))
                .or_insert(vec![dirs]);
            continue;
        }

        let neighbors = DIRS
            .iter()
            .filter_map(|dir| {
                let next = (curr.0 + dir.0, curr.1 + dir.1);
                let visited = path.iter().map(|p| p.0).collect::<Vec<_>>();

                if !visited.contains(&next) && get(num_pad, next).is_some_and(|c| c != '\0') {
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

fn get(num_pad: &NumPad, pos: Pos) -> Option<char> {
    num_pad
        .get(pos.1 as usize)
        .and_then(|l| l.get(pos.0 as usize))
        .copied()
}

fn extract_codes(input: &str) -> Vec<&str> {
    input.lines().collect()
}
