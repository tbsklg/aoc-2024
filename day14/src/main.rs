use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let robots: HashMap<(isize, isize), usize> = create_robots(input)
        .iter()
        .map(|r| simulate(r, 100))
        .fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|c| *c += 1).or_insert(1);
            acc
        });

    println!("{:?}", robots);
    0
}

const WIDTH: isize = 11;
const HEIGHT: isize = 7;

fn create_robots(input: &str) -> Vec<Robot> {
    input.lines().map(Robot::from).collect::<Vec<Robot>>()
}

fn simulate(r: &Robot, times: isize) -> (isize, isize) {
    let (rx, ry) = r.pos;
    let (vx, vy) = r.vel;

    ((rx + 1 + vx * times).rem_euclid(WIDTH), (ry + 1 + vy * times).rem_euclid(HEIGHT))
}

#[derive(Debug, PartialEq)]
struct Robot {
    // x, y
    pos: (isize, isize),
    vel: (isize, isize),
}

impl From<&str> for Robot {
    fn from(line: &str) -> Self {
        let re = Regex::new(r"-?\d+").unwrap();
        let caps = re
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect::<Vec<isize>>();

        Self {
            pos: (caps[0], caps[1]),
            vel: (caps[2], caps[3]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Robot;

    #[test]
    fn should_create_robot_from_line() {
        let line = "p=0,4 v=3,-3";

        assert_eq!(
            Robot {
                pos: (0, 4),
                vel: (3, -3)
            },
            Robot::from(line)
        )
    }
}
