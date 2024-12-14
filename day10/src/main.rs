use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    Map::from(input).count_tracks()
}

fn part2(input: &str) -> usize {
    Map::from(input).count_distinct_tracks()
}

struct Map {
    topographic: Vec<Vec<u8>>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let topographic = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        Map { topographic }
    }
}

impl Map {
    fn get(&self, (r, c): (i32, i32)) -> Option<u8> {
        self.topographic
            .get(r as usize)
            .and_then(|l| l.get(c as usize))
            .copied()
    }

    fn count_tracks(&self) -> usize {
        self.topographic
            .iter()
            .enumerate()
            .map(|(i, l)| {
                l.iter()
                    .enumerate()
                    .filter(|(_, p)| **p == 0)
                    .map(move |(j, _)| (i as i32, j as i32))
                    .map(|p| self.find_tracks(p))
                    .sum::<usize>()
            })
            .sum()
    }

    fn count_distinct_tracks(&self) -> usize {
        self.topographic
            .iter()
            .enumerate()
            .map(|(i, l)| {
                l.iter()
                    .enumerate()
                    .filter(|(_, p)| **p == 0)
                    .map(move |(j, _)| (i as i32, j as i32))
                    .map(|p| self.find_distinct_tracks(p))
                    .sum::<usize>()
            })
            .sum()
    }

    fn find_distinct_tracks(&self, position: (i32, i32)) -> usize {
        let mut stack = vec![];

        let mut found = 0;
        stack.push((position, 0));

        while let Some((p, c)) = stack.pop() {
            if c == 9 {
                found += 1;
                continue;
            }

            for n in neighbors(&p) {
                match self.get(n) {
                    Some(x) => {
                        if x == self.get(p).unwrap() + 1 {
                            stack.push((n, c + 1))
                        }
                    }
                    None => continue,
                }
            }
        }

        found
    }

    fn find_tracks(&self, position: (i32, i32)) -> usize {
        let mut stack = vec![];
        let mut visited = HashSet::new();

        let mut found = 0;
        stack.push((position, 0));

        while let Some((p, c)) = stack.pop() {
            if visited.insert(p) {
                if c == 9 {
                    found += 1;
                    continue;
                }
            }

            for n in neighbors(&p) {
                match self.get(n) {
                    Some(x) => {
                        if x == self.get(p).unwrap() + 1 {
                            stack.push((n, c + 1))
                        }
                    }
                    None => continue,
                }
            }
        }

        found
    }
}

fn neighbors((r, c): &(i32, i32)) -> Vec<(i32, i32)> {
    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

    dirs.iter().map(|(x, y)| (r + x, c + y)).collect()
}
