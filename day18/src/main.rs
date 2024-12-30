use std::collections::{BinaryHeap, HashSet};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn part1(input: &str) -> usize {
    let byte_positions = coordinates(input);
    let start = State {
        pos: (0, 0),
        steps: 0,
    };

    shortest_path(&byte_positions[0..=1024].iter().cloned().collect(), start).unwrap()
}

fn part2(input: &str) -> Option<Pos> {
    let byte_positions = coordinates(input);
    let start = State {
        pos: (0, 0),
        steps: 0,
    };

    (1024..byte_positions.len())
        .into_par_iter()
        .find_map_first(|i| {
            match shortest_path(&byte_positions[0..=i].iter().cloned().collect(), start) {
                Some(_) => None,
                None => Some(i),
            }
        })
        .map(|i| byte_positions[i])
}

fn coordinates(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|l| {
            let c = l
                .split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (c[0], c[1])
        })
        .collect()
}

type Pos = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const GRID_SIZE: i32 = 71;

fn shortest_path(byte_positions: &HashSet<Pos>, start: State) -> Option<usize> {
    let mut queue = BinaryHeap::from([start]);
    let mut seen: HashSet<Pos> = HashSet::new();

    while let Some(state) = queue.pop() {
        if !seen.insert(state.pos) {
            continue;
        }

        if state.pos == (70, 70) {
            return Some(state.steps);
        }

        let next_states = DIRS
            .iter()
            .map(|&(nx, ny)| State {
                pos: (state.pos.0 + nx, state.pos.1 + ny),
                steps: state.steps + 1,
            })
            .filter(in_bounds)
            .filter(|s| !byte_positions.contains(&s.pos))
            .collect::<Vec<State>>();

        queue.extend(next_states);
    }

    None
}

fn in_bounds(state: &State) -> bool {
    state.pos.0 >= 0 && state.pos.0 < GRID_SIZE && state.pos.1 >= 0 && state.pos.1 < GRID_SIZE
}
