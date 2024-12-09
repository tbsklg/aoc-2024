use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    Map::from(input).walk()
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn next(&self, (row, col): (i32, i32)) -> (i32, i32) {
        match self {
            Dir::Up => (row - 1, col),
            Dir::Down => (row + 1, col),
            Dir::Left => (row, col - 1),
            Dir::Right => (row, col + 1),
        }
    }

    fn turn(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

#[derive(Debug)]
struct Guard {
    pos: Pos,
    dir: Dir,
}

#[derive(Debug)]
struct Map {
    guard: Guard,
    grid: Vec<Vec<char>>,
}

type Pos = (i32, i32);

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let (rows, cols) = (grid.len(), grid.first().map_or(0, |l| l.len()));

        let pos = (0..rows)
            .flat_map(|row| (0..cols).map(move |col| (row, col)))
            .find(|&(row, col)| grid[row][col] == '^')
            .map(|(row, col)| (row as i32, col as i32))
            .expect("No guard found!");

        Self {
            guard: Guard { pos, dir: Dir::Up },
            grid,
        }
    }
}

impl Map {
    fn walk(&mut self) -> usize {
        let mut points: HashSet<Pos> = HashSet::new();

        loop {
            points.insert(self.guard.pos);
            let next = self.guard.dir.next(self.guard.pos);

            match self.get(next) {
                Some('#') => self.guard.dir = self.guard.dir.turn(),
                Some(_) => self.guard.pos = next,
                None => break,
            }
        }

        points.len()
    }

    fn get(&self, (row, col): Pos) -> Option<char> {
        self.grid
            .get(row as usize)
            .and_then(|r| r.get(col as usize).copied())
    }
}
