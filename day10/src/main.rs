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
    Map::from(input).count_all_tracks()
}

struct Map {
    topographic: Vec<Vec<u8>>,
    trail_heads: Vec<(i32, i32)>,
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
            .collect::<Vec<Vec<u8>>>();

        let trail_heads = topographic
            .iter()
            .enumerate()
            .flat_map(|(i, l)| {
                l.iter()
                    .enumerate()
                    .filter(|(_, p)| **p == 0)
                    .map(move |(j, _)| (i as i32, j as i32))
            })
            .collect::<Vec<(i32, i32)>>();

        Map {
            topographic,
            trail_heads,
        }
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
        self.trail_heads.iter().map(|p| self.find_tracks(*p)).sum()
    }

    fn count_all_tracks(&self) -> usize {
        self.trail_heads
            .iter()
            .map(|p| self.find_all_tracks(p))
            .sum()
    }

    fn find_tracks(&self, position: (i32, i32)) -> usize {
        let mut stack = vec![];
        let mut visited = HashSet::new();

        let mut found = 0;
        stack.push((position, 0));

        while let Some((p, c)) = stack.pop() {
            if visited.insert(p) && c == 9 {
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

    fn find_all_tracks(&self, pos: &(i32, i32)) -> usize {
        if self.get(*pos) == Some(9) {
            return 1;
        }

        neighbors(pos)
            .iter()
            .filter(|n| match (self.get(**n), self.get(*pos)) {
                (Some(x), Some(y)) => x == y + 1,
                _ => false,
            })
            .map(|p| self.find_all_tracks(p))
            .sum::<usize>()
    }
}

fn neighbors((r, c): &(i32, i32)) -> Vec<(i32, i32)> {
    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

    dirs.iter().map(|(x, y)| (r + x, c + y)).collect()
}
