fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let instructions: Vec<&str> = input.split("\n\n").collect();
    let map = Map::from(instructions[0]);
    let moves: Vec<Dir> = instructions[1].trim_end().chars().map(Dir::from).collect();

    println!("{:?}", map);
    println!("{:?}", moves);


    0
}

#[derive(Debug)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '<' => Dir::Left,
            '>' => Dir::Right,
            '^' => Dir::Up,
            'v' => Dir::Down,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Map {
    warehouse: Vec<Vec<char>>,
    robot: (isize, isize),
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let warehouse: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

        let robot = warehouse.iter().enumerate().find_map(|(row, line)| {
            line.iter().enumerate().find_map(|(col, &c)| {
                if c == '@' {
                    Some((row as isize, col as isize))
                } else {
                    None
                }
            })
        });

        Self {
            warehouse,
            robot: robot.unwrap(),
        }
    }
}

impl Map {
    fn move_robot(&self, moves: Vec<Dir>) -> Vec<char> {
        todo!();
    }
}
