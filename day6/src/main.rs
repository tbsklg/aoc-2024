fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let map = Map::from(input);
    println!("{:#?}", map);
    1
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Guard {
    pos: Pos,
    dir: Dir,
}

#[derive(Debug)]
struct Map {
    guard: Guard,
    obstructions: Vec<(usize, usize)>,
}

type Pos = (usize, usize);

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let (rows, cols) = (grid.len(), grid.first().map_or(0, |l| l.len()));

        let obstructions = (0..rows)
            .flat_map(|row| (0..cols).map(move |col| (row, col)))
            .filter(|&(row, col)| grid[row][col] == '#')
            .collect();

        let pos = (0..rows)
            .flat_map(|row| (0..cols).map(move |col| (row, col)))
            .find(|&(row, col)| grid[row][col] == '^')
            .expect("No guard found!");

        Self {
            guard: Guard { pos, dir: Dir::Up },
            obstructions,
        }
    }
}
