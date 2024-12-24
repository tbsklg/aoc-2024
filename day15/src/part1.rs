pub fn part1(input: &str) -> usize {
    let instructions: Vec<&str> = input.split("\n\n").collect();
    let moves: Vec<Dir> = instructions[1]
        .lines()
        .flat_map(|l| l.trim_end().chars().map(Dir::from))
        .collect();

    let map = &mut Map::from(instructions[0]);
    map.move_robot(moves);
    map.calculate_gps()
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
            line.iter().enumerate().find_map(
                |(col, &c)| {
                    if c == '@' {
                        Some((col, row))
                    } else {
                        None
                    }
                },
            )
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
                    continue;
                }
                Some('O') => match self.try_to_move_box(next_robot_position, m.step()) {
                    Some(free_position) => {
                        self.set(self.robot, '.');
                        self.set(free_position, '@');
                        self.robot = free_position;
                    }
                    None => continue,
                },
                _ => continue,
            }
        }
    }

    fn try_to_move_box(
        &mut self,
        box_position: (usize, usize),
        step: (i32, i32),
    ) -> Option<(usize, usize)> {
        let next_box_position = (
            (box_position.0 as i32 + step.0) as usize,
            (box_position.1 as i32 + step.1) as usize,
        );

        match self.get(next_box_position) {
            Some('#') => None,
            Some('.') => {
                self.set(box_position, '.');
                self.set(next_box_position, 'O');
                Some(box_position)
            }
            Some('O') => match self.try_to_move_box(next_box_position, step) {
                Some(free_position) => {
                    self.set(box_position, '.');
                    self.set(free_position, 'O');
                    Some(box_position)
                }
                None => None,
            },
            _ => None,
        }
    }

    fn calculate_gps(&self) -> usize {
        self.warehouse
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(
                    move |(j, &c)| {
                        if c == 'O' {
                            Some(i * 100 + j)
                        } else {
                            None
                        }
                    },
                )
            })
            .sum()
    }

    fn get(&self, (l, t): (usize, usize)) -> Option<char> {
        self.warehouse.get(t).and_then(|row| row.get(l)).copied()
    }

    fn set(&mut self, (l, t): (usize, usize), c: char) {
        self.warehouse[t][l] = c;
    }
}

#[cfg(test)]
mod tests {
    use crate::part1::part1;

    #[test]
    fn should_calculate_gps() {
        let input = "########\n#..O.O.#\n#.#.O..#\n#..@.O.#\n#OO....#\n########\n\n<^^>>>vv<v>>v<<";

        let score = part1(input);

        assert_eq!(1521, score);
    }
}
