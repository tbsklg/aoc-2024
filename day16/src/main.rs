use std::collections::{HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let map = create_map(input);
    let initial_state = State {
        pos: find_start(&map).unwrap(),
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

fn dirs((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .collect()
}

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
    cost: usize,
}

fn calculate_score(steps: usize, rotations: usize) -> usize {
    steps + rotations * 1000
}

fn shortest_path(map: &Vec<Vec<char>>, start: State) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    queue.push_back(start);

    while let Some(state) = queue.pop_front() {
        if seen.contains(&state.pos) {
            continue;
        }
        seen.insert(state.pos);

        if get(map, state.pos) == 'E' {
            println!("Found exit in {} steps", state.cost);
            return Some(state.cost);
        }

        let next_states = dirs(state.pos)
            .iter()
            .filter(|pos| get(map, **pos) != '#')
            .map(|&pos| State {
                pos,
                cost: state.cost + 1,
            })
            .collect::<Vec<_>>();

        queue.extend(next_states);
    }

    None
}
