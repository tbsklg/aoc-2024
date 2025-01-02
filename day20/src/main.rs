use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let grid = extract_grid(input);
    let path = path(&grid, find_start(&grid).unwrap());

    cheats_p1(&path.unwrap()).unwrap_or(0)
}

fn part2(input: &str) -> usize {
    let grid = extract_grid(input);
    let path = path(&grid, find_start(&grid).unwrap());

    cheats_p2(&path.unwrap()).unwrap_or(0)
}

fn cheats_p1(path: &[Pos]) -> Option<usize> {
    let distances = path
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, i))
        .collect::<Vec<_>>();

    Some(
        distances
            .iter()
            .enumerate()
            .map(|(i, (p1, d1))| {
                distances.iter().skip(i + 1).fold(0, |mut acc, (p2, d2)| {
                    let distance = dist(*p1, *p2);
                    let path_gap = *d2 - d1 - distance;

                    if distance == 2 && path_gap >= 100 {
                        acc += 1;
                    }
                    acc
                })
            })
            .sum(),
    )
}

fn cheats_p2(path: &[Pos]) -> Option<usize> {
    let distances = path
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, i))
        .collect::<Vec<_>>();

    Some(
        distances
            .iter()
            .enumerate()
            .map(|(i, (p1, d1))| {
                distances.iter().skip(i + 1).fold(0, |mut acc, (p2, d2)| {
                    let distance = dist(*p1, *p2);
                    let path_gap = *d2 - d1 - distance;

                    if distance <= 20 && path_gap >= 100 {
                        acc += 1;
                    }
                    acc
                })
            })
            .sum(),
    )
}

fn dist((a, b): (i32, i32), (c, d): (i32, i32)) -> usize {
    ((a - c).abs() + (b - d).abs()).abs().unsigned_abs() as usize
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
