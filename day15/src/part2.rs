use std::collections::HashSet;

pub fn part2(input: &str) -> usize {
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
    fn move_robot(&mut self, moves: Vec<Dir>) {
        self.print_warehouse();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut stack = vec![self.robot];

        for m in moves.get(0..5).unwrap() {
            while let Some(target) = stack.pop() {
                let next = m.next(target);
                match self.get(next) {
                    Some('#') => continue,
                    Some('[') => {
                        if visited.contains(&target) {
                            continue;
                        }
                        stack.push(next);
                        stack.push((next.0 + 1, next.1));

                        visited.insert(next);
                        visited.insert((next.0 + 1, next.1));
                    }
                    Some(']') => {
                        if visited.contains(&next) {
                            continue;
                        }

                        stack.push(next);
                        stack.push((next.0 - 1, next.1));

                        visited.insert(next);
                        visited.insert((next.0 - 1, next.1));
                    }
                    _ => continue,
                }
            }

            let old_state = self.clone();
            
            let next_robot_position = m.next(self.robot);
            self.set(next_robot_position, '@');
            self.set(self.robot, '.');
            self.robot = next_robot_position;
            
            for target in &visited {
                let old = old_state.get(*target).unwrap();
                self.set(m.next(*target), old); 
            }

            visited.clear();
            stack.clear();
        }
        self.print_warehouse();
        println!("{:?}", visited);
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

    fn print_warehouse(&self) {
        for row in &self.warehouse {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    }
}
