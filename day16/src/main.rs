use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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

fn part2(input: &str) -> usize {
    let map = create_map(input);
    let initial_state = State {
        pos: find_start(&map).unwrap(),
        dir: (1, 0),
        cost: 0,
    };
    all_path(&map, initial_state).unwrap()
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

fn all_path(map: &Vec<Vec<char>>, start: State) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut distances: HashMap<((i32, i32), (i32, i32)), usize> = HashMap::new();
    let mut backtrack: HashMap<((i32, i32), (i32, i32)), HashSet<((i32, i32), (i32, i32))>> =
        HashMap::new();
    let mut min_cost = usize::MAX;

    let mut end_states = HashSet::new();

    queue.push(start);
    distances.insert((start.pos, start.dir), 0);

    while let Some(state) = queue.pop() {
        if let Some(&dist) = distances.get(&(state.pos, state.dir)) {
            if state.cost > dist {
                continue;
            }
        }

        if get(map, state.pos) == 'E' {
            if state.cost > min_cost {
                break;
            }

            min_cost = state.cost;
            end_states.insert((state.pos, state.dir, state.cost));
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
            .filter(|state| get(map, state.pos) != '#') // Avoid walls
            .filter(|state| {
                distances
                    .get(&(state.pos, state.dir))
                    .map_or(true, |&dist| state.cost < dist)
            })
            .collect::<Vec<_>>();

        for next_state in next_states {
            distances.insert((next_state.pos, next_state.dir), next_state.cost);
            backtrack
                .entry((next_state.pos, next_state.dir))
                .or_insert(HashSet::new())
                .insert((state.pos, state.dir));
            queue.push(next_state);
        }
    }

    let mut tiles = HashSet::new();
    let mut states = end_states
        .iter()
        .map(|&(pos, dir, _)| (pos, dir))
        .collect::<Vec<_>>();

    while let Some((pos, dir)) = states.pop() {
        tiles.insert(pos);

        if let Some(back) = backtrack.get(&(pos, dir)) {
            for &prev in back {
                tiles.insert(prev.0);
                states.push(prev);
            }
        }
    }

    println!("{:?}", tiles.len());

    if min_cost == usize::MAX {
        None
    } else {
        Some(min_cost)
    }
}

fn rotation_score((dx, dy): (i32, i32), (nx, ny): (i32, i32)) -> usize {
    if dx == nx && dy == ny {
        0
    } else {
        1000
    }
}
