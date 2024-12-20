use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    0
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
        
        assert_eq!(Robot { pos: (0, 4), vel: (3, -3) }, Robot::from(line))
    }
}
