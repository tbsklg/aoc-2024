use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let now = std::time::Instant::now();

    let cheats = part1(&input);
    println!("Part 1: {} ({:?})", cheats, now.elapsed());
}

fn part1(input: &str) -> usize {
    let grid = extract_grid(input);
    let start = find_start(&grid).unwrap();
    let path = path(&extract_grid(input), start);

    cheats(&path.unwrap()).unwrap_or(0)
}

fn distances(path: &[Pos]) -> HashMap<Pos, usize> {
    path.iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, curr)| {
            acc.insert(*curr, path.len() - i);
            acc
        })
}

fn cheats(path: &[Pos]) -> Option<usize> {
    let distances = distances(path);

    Some(
        path.iter()
            .enumerate()
            .map(|(i, curr)| {
                path.iter()
                    .skip(i)
                    .filter(|next| dist(curr, next) <= 20)
                    .filter(|pos| {
                        (distances
                            .get(&pos)
                            .unwrap()
                            .abs_diff(*distances.get(curr).unwrap()))
                            > 100
                    })
                    .count()
            })
            .sum(),
    )
}

fn dist((a, b): &(i32, i32), (c, d): &(i32, i32)) -> usize {
    ((a - c).abs() + (b - d).abs()).abs() as usize
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
