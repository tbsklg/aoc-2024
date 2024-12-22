use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    print_robots(part2(&create_robots(&input)));
}

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

fn part1(input: &str) -> usize {
    let robots: HashMap<(isize, isize), usize> = create_robots(input)
        .iter()
        .map(|r| simulate(r, 100))
        .fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|c| *c += 1).or_insert(1);
            acc
        });

    let hs = ((HEIGHT as f32 - 1f32) / 2f32).floor() as isize;
    let vs = ((WIDTH as f32 - 1f32) / 2f32).floor() as isize;

    let (q1, q2, q3, q4) = robots
        .iter()
        .fold((0, 0, 0, 0), |(q1, q2, q3, q4), (k, v)| {
            if k.0 < vs && k.1 < hs {
                return (q1 + v, q2, q3, q4);
            }

            if k.0 > vs && k.1 < hs {
                return (q1, q2 + v, q3, q4);
            }

            if k.0 < vs && k.1 > hs {
                return (q1, q2, q3 + v, q4);
            }

            if k.0 > vs && k.1 > hs {
                return (q1, q2, q3, q4 + v);
            }

            (q1, q2, q3, q4)
        });

    vec![q1, q2, q3, q4].iter().product()
}

fn part2(robots: &Vec<Robot>) -> Vec<(isize, isize)> {
    robots.iter().map(|r| simulate(r, 7051)).collect()
}

fn print_robots(points: Vec<(isize, isize)>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn create_robots(input: &str) -> Vec<Robot> {
    input.lines().map(Robot::from).collect::<Vec<Robot>>()
}

fn simulate(r: &Robot, times: isize) -> (isize, isize) {
    let (rx, ry) = r.pos;
    let (vx, vy) = r.vel;

    (
        (rx + vx * times).rem_euclid(WIDTH),
        (ry + vy * times).rem_euclid(HEIGHT),
    )
}

#[derive(Debug, PartialEq)]
struct Robot {
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
