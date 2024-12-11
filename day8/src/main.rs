use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    Map::from(input).antinodes().unwrap_or(0)
}

#[derive(Debug)]
struct Map {
    rows: usize,
    cols: usize,
    antennas: Vec<(Point, char)>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Point(i32, i32);

impl Sub<&Point> for Point {
    type Output = Self;

    fn sub(self, rhs: &Point) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<&Point> for Point {
    type Output = Self;

    fn add(self, rhs: &Point) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let (rows, cols) = (map.len(), map.first().map_or(0, |l| l.len()));

        let antennas = (0..rows)
            .flat_map(|row| (0..cols).map(move |col| (row, col)))
            .filter(|&(row, col)| map[row][col] != '.')
            .map(|(row, col)| (Point(row as i32, col as i32), map[row][col]))
            .collect::<Vec<(Point, char)>>();

        Self {
            rows,
            cols,
            antennas,
        }
    }
}

impl Map {
    fn antinodes(&self) -> Option<usize> {
        let points: HashSet<Point> = self.antennas.iter().fold(HashSet::new(), |mut points, p| {
            let same_antennas = self
                .antennas
                .iter()
                .filter(|(cp, a)| *cp != p.0 && *a == p.1)
                .flat_map(|(cp, _)| vec![*cp + cp - &p.0, p.0 + &p.0 - cp])
                .filter(|p| self.in_bounds(p))
                .collect::<Vec<_>>();

            for p in same_antennas {
                points.insert(p);
            }

            points
        });

        Some(points.len())
    }

    fn in_bounds(&self, Point(r, c): &Point) -> bool {
        *r >= 0 && *r < self.rows as i32 && *c >= 0 && *c < self.cols as i32
    }
}
