use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    Map::from(input).walk().len()
}

fn part2(input: &str) -> usize {
    let mut map = Map::from(input);

    map.walk().iter().filter(|&p| map.loops(*p)).count()
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn next_position(&self, (row, col): (i32, i32)) -> (i32, i32) {
        match self {
            Dir::Up => (row - 1, col),
            Dir::Down => (row + 1, col),
            Dir::Left => (row, col - 1),
            Dir::Right => (row, col + 1),
        }
    }

    fn rotate_right_90(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Guard {
    pos: Pos,
    dir: Dir,
}

#[derive(Debug)]
struct Map {
    current_guard: Guard,
    initial_guard: Guard,
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
            current_guard: Guard { pos, dir: Dir::Up },
            initial_guard: Guard { pos, dir: Dir::Up },
            grid,
        }
    }
}

impl Map {
    fn walk(&mut self) -> HashSet<Pos> {
        let mut points: HashSet<Pos> = HashSet::new();

        loop {
            points.insert(self.current_guard.pos);
            let next = self.current_guard.dir.next_position(self.current_guard.pos);

            match self.get(next) {
                Some('#') => self.current_guard.dir = self.current_guard.dir.rotate_right_90(),
                Some(_) => self.current_guard.pos = next,
                None => break,
            }
        }

        points
    }

    fn loops(&mut self, obstacle: Pos) -> bool {
        let mut points: HashSet<(Pos, Dir)> = HashSet::new();

        self.current_guard = self.initial_guard;
        self.place_obstacle(obstacle);

        let is_lopping = loop {
            if !points.insert((self.current_guard.pos, self.current_guard.dir)) {
                break true;
            }

            let next = self.current_guard.dir.next_position(self.current_guard.pos);

            match self.get(next) {
                Some('#' | 'O') => {
                    self.current_guard.dir = self.current_guard.dir.rotate_right_90()
                }
                Some(_) => self.current_guard.pos = next,
                None => break false,
            }
        };

        self.remove_obstacle(obstacle);
        is_lopping
    }

    fn get(&self, (row, col): Pos) -> Option<char> {
        self.grid
            .get(row as usize)
            .and_then(|r| r.get(col as usize).copied())
    }

    fn place_obstacle(&mut self, obstacle: Pos) {
        self.grid[obstacle.0 as usize][obstacle.1 as usize] = 'O';
    }

    fn remove_obstacle(&mut self, obstacle: Pos) {
        self.grid[obstacle.0 as usize][obstacle.1 as usize] = '.';
    }
}
