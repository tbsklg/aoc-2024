use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let grid = extract_grid(input);
    let start = find_start(&grid).unwrap();
    let path = path(&extract_grid(input), start);

    cheats(&grid, &path.unwrap()).unwrap_or(0)
}

fn positions(path: &[Pos]) -> HashMap<Pos, usize> {
    path.iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, curr)| {
            acc.insert(*curr, i);
            acc
        })
}

fn cheats(map: &[Vec<char>], path: &[Pos]) -> Option<usize> {
    let positions = positions(path);

    let mut i = 0;
    let mut cheats = HashMap::new();

    while let Some(curr) = path.get(i) {
        match path.get(i + 1..) {
            Some(tail) => {
                let cheat_positions = DIRS
                    .iter()
                    .map(|(dx, dy)| (dx + curr.0, dy + curr.1))
                    .filter(|pos| get(pos, map) == Some('#'))
                    .flat_map(|pos| {
                        DIRS.iter()
                            .map(|(dx, dy)| (dx + pos.0, dy + pos.1))
                            .filter(|pos| get(pos, map) == Some('.') || get(pos, map) == Some('E'))
                            .filter(|pos| dist(pos, curr) == Some(2))
                            .filter(|pos| tail.contains(pos))
                            .collect::<Vec<_>>()
                    })
                    .map(|pos| positions.get(&pos).unwrap() - positions.get(curr).unwrap() - 2)
                    .collect::<Vec<_>>();

                for cheat_position in cheat_positions {
                    cheats
                        .entry(cheat_position)
                        .and_modify(|r| *r += 1)
                        .or_insert(1);
                }

                i += 1;
            }
            None => todo!(),
        }
    }

    Some(
        cheats
            .iter()
            .filter(|(k, _)| k >= &&100)
            .map(|(_, v)| v)
            .sum(),
    )
}

fn dist((a, b): &(i32, i32), (c, d): &(i32, i32)) -> Option<i32> {
    if a == c {
        return Some((b - d).abs());
    }

    if b == d {
        return Some((a - c).abs());
    }

    None
}

fn extract_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
}

fn find_start(map: &[Vec<char>]) -> Option<(i32, i32)> {
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

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
type Pos = (i32, i32);

fn path(grid: &[Vec<char>], start: Pos) -> Option<Vec<Pos>> {
    let mut queue: Vec<Pos> = vec![start];
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut path: Vec<Pos> = vec![];

    while let Some(curr) = queue.pop() {
        if !visited.insert(curr) {
            continue;
        }

        path.push(curr);

        if get(&curr, grid) == Some('E') {
            return Some(path);
        }

        let next = DIRS
            .iter()
            .map(|(dx, dy)| (curr.0 + dx, curr.1 + dy))
            .filter(|pos| get(pos, grid) == Some('.') || get(pos, grid) == Some('E'))
            .collect::<Vec<Pos>>();

        queue.extend(next);
    }

    None
}

fn get((x, y): &Pos, grid: &[Vec<char>]) -> Option<char> {
    grid.get(*y as usize)
        .and_then(|r| r.get(*x as usize))
        .copied()
}
