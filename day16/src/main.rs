use std::collections::{BinaryHeap, HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let map = create_map(input);
    let initial_state = State {
        pos: find_start(&map).unwrap(),
        dir: (1, 0),
        cost: 0,
    };
    shortest_path(&map, initial_state).unwrap()
}

fn create_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn find_start(map: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    map.iter().enumerate().find_map(|(row, line)| {
        line.iter().enumerate().find_map(|(col, &c)| {
            if c == 'S' {
                Some((col as i32, row as i32))
            } else {
                None
            }
        })
    })
}

fn get(map: &Vec<Vec<char>>, (x, y): (i32, i32)) -> char {
    map[y as usize][x as usize]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: (i32, i32),
    dir: (i32, i32),
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(map: &Vec<Vec<char>>, start: State) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut seen: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    queue.push(start);

    while let Some(state) = queue.pop() {
        if seen.contains(&(state.pos, state.dir)) {
            continue;
        }
        seen.insert((state.pos, state.dir));

        if get(map, state.pos) == 'E' {
            return Some(state.cost);
        }

        let (x, y) = state.pos;
        let (dx, dy) = state.dir;

        let next_states = DIRS
            .iter()
            .map(|&(nx, ny)| State {
                pos: (x + nx, y + ny),
                dir: (nx, ny),
                cost: state.cost + 1 + rotation_score((dx, dy), (nx, ny)),
            })
            .filter(|state| get(map, state.pos) != '#')
            .collect::<Vec<_>>();

        queue.extend(next_states);
    }

    None
}

fn rotation_score((dx, dy): (i32, i32), (nx, ny): (i32, i32)) -> usize {
    if dx == nx && dy == ny {
        0
    } else {
        1000
    }
}
