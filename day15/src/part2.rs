use std::collections::HashSet;

pub fn part2(input: &str) -> usize {
    let instructions: Vec<&str> = input.split("\n\n").collect();
    let moves: Vec<Dir> = instructions[1]
        .lines()
        .flat_map(|l| l.trim_end().chars().map(Dir::from))
        .collect();

    let map = &mut Map::from(instructions[0]);
    map.move_boxes(moves);
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
}

#[derive(Debug, Clone)]
struct Map {
    warehouse: Vec<Vec<char>>,
    robot: (usize, usize),
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let warehouse: Vec<Vec<char>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .flat_map(|c| match &c {
                        '#' => vec!['#', '#'],
                        'O' => vec!['[', ']'],
                        '.' => vec!['.', '.'],
                        '@' => vec!['@', '.'],
                        _ => vec![],
                    })
                    .collect()
            })
            .collect();

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
    fn move_boxes(&mut self, moves: Vec<Dir>) {
        for m in moves {
            let mut stack = vec![self.robot];
            let mut captures: HashSet<(usize, usize)> = HashSet::new();
            let mut can_move = true;

            while let Some(target) = stack.pop() {
                let next = m.next(target);
                if captures.contains(&next) {
                    continue;
                }

                match self.get(next) {
                    Some('#') => {
                        can_move = false;
                        break;
                    }
                    Some('[') => {
                        let right = (next.0 + 1, next.1);

                        if captures.insert(next) {
                            stack.push(next);
                        }

                        if captures.insert(right) {
                            stack.push(right);
                        }
                    }
                    Some(']') => {
                        let left = (next.0 - 1, next.1);

                        if captures.insert(next) {
                            stack.push(next);
                        }

                        if captures.insert(left) {
                            stack.push(left);
                        }
                    }
                    _ => continue,
                }
            }

            if !can_move {
                continue;
            }

            self.move_targets(&captures, &m);
            self.move_robot(m);
        }
    }

    fn move_targets(&mut self, captures: &HashSet<(usize, usize)>, dir: &Dir) {
        let old_state = self.clone();
        self.clear(captures);

        for capture in captures {
            let old = old_state.get(*capture).unwrap();

            self.set(dir.next(*capture), old);
        }
    }

    fn clear(&mut self, captures: &HashSet<(usize, usize)>) {
        for capture in captures {
            self.set(*capture, '.');
        }
    }

    fn move_robot(&mut self, dir: Dir) {
        self.set(self.robot, '.');
        self.set(dir.next(self.robot), '@');
        self.robot = dir.next(self.robot);
    }

    fn calculate_gps(&self) -> usize {
        self.warehouse
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(
                    move |(j, &c)| {
                        if c == '[' {
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
