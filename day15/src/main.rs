fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let instructions: Vec<&str> = input.split("\n\n").collect();
    let map = &mut Map::from(instructions[0]);
    let moves: Vec<Dir> = instructions[1].trim_end().chars().map(Dir::from).collect();

    map.move_robot(moves);

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

impl Dir {
    fn next(&self, (l, t): (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => (l, t - 1),
            Dir::Left => (l - 1, t),
            Dir::Right => (l + 1, t),
            Dir::Down => (l, t + 1),
        }
    }

    fn step(&self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    warehouse: Vec<Vec<char>>,
    robot: (usize, usize),
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let warehouse: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

        let robot = warehouse.iter().enumerate().find_map(|(row, line)| {
            line.iter().enumerate().find_map(|(col, &c)| {
                if c == '@' {
                    Some((row as usize, col as usize))
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
    fn move_robot(&mut self, moves: Vec<Dir>) {
        for m in moves {
            let next_robot_position = m.next(self.robot);

            match self.get(next_robot_position) {
                Some('#') => continue,
                Some('.') => {
                    self.set(self.robot, '.');
                    self.set(next_robot_position, '@');
                    self.robot = next_robot_position;
                    println!("Robot moved to {:?}", next_robot_position);
                    self.print_warehouse();
                    continue;
                }
                Some('O') => {
                    self.try_to_move_box(next_robot_position, m.step());
                    self.print_warehouse();
                }
                _ => continue,
            }

        }
    }
    
    fn try_to_move_box(&mut self, box_position: (usize, usize), step: (i32, i32)) {
        let next_box_position = (box_position.0 as i32 + step.0, box_position.1 as i32 + step.1);
        let next_box_position = (next_box_position.0 as usize, next_box_position.1 as usize);

        loop {
            match self.get(next_box_position) {
                Some('#') => break,
                Some('.') => {
                    self.set(box_position, '.');
                    self.set(next_box_position, 'O');
                    break;
                }
                _ => {
                    self.try_to_move_box(next_box_position, step);
                    self.set(box_position, '.');
                    self.set(next_box_position, 'O');
                    break;
                }
            }
        }

    }

    fn get(&self, (l, t): (usize, usize)) -> Option<char> {
        self.warehouse.get(t).and_then(|row| row.get(l)).copied()
    }

    fn set(&mut self, (l, t): (usize, usize), c: char) {
        self.warehouse[t][l] = c;
    }

    fn print_warehouse(&self) {
        self.warehouse.iter().for_each(|row| {
            row.iter().for_each(|c| print!("{}", c));
            println!();
        });
    }
}
