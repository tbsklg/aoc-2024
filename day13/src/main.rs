use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    create_claw_machines(input)
        .iter()
        .map(|machine| machine.cramers_rule().map_or(0, |(x, y)| x * 3 + y))
        .sum()
}

const PRIZE_OFFSET: isize = 10_000_000_000_000;

fn part2(input: &str) -> isize {
    create_claw_machines(input)
        .iter()
        .map(|machine| ClawMachine {
            a: machine.a,
            b: machine.b,
            prize: (machine.prize.0 + PRIZE_OFFSET, machine.prize.1 + PRIZE_OFFSET),
        })
        .map(|machine| machine.cramers_rule().map_or(0, |(x, y)| x * 3 + y))
        .sum()
}

fn create_claw_machines(input: &str) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .map(ClawMachine::from)
        .collect::<Vec<ClawMachine>>()
}

#[derive(Debug)]
struct ClawMachine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl From<&str> for ClawMachine {
    fn from(value: &str) -> Self {
        assert!(value.lines().count() == 3);

        let numbers = value
            .lines()
            .map(extract_numbers)
            .collect::<Vec<(isize, isize)>>();

        Self {
            a: numbers[0],
            b: numbers[1],
            prize: numbers[2],
        }
    }
}

impl ClawMachine {
    // see https://en.wikipedia.org/wiki/Cramer%27s_rule
    fn cramers_rule(&self) -> Option<(isize, isize)> {
        let (ax, ay) = self.a;
        let (bx, by) = self.b;
        let (px, py) = self.prize;

        let det = ax * by - ay * bx;
        if det == 0 {
            return None;
        }

        let da = px * by - py * bx;
        let db = ax * py - ay * px;

        if da % det != 0 || db % det != 0 {
            return None;
        }

        Some((da / det, db / det))
    }
}

fn extract_numbers(line: &str) -> (isize, isize) {
    let re = Regex::new(r"\b\d+\b").unwrap();
    let caps = re
        .find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect::<Vec<isize>>();

    (caps[0], caps[1])
}

#[cfg(test)]
mod tests {
    use crate::extract_numbers;

    #[test]
    fn should_extract_numbers_from_line() {
        let line = "But2ton A: X+94, Y+34";

        assert_eq!((94, 34), extract_numbers(line));
    }

    #[test]
    fn should_create_claw_machine_from_input() {
        let input = "Button A: X+94, Y+34\nButton B: X+74, Y+34\nPrize: X+104, Y+32324";

        let claw_machine = crate::ClawMachine::from(input);

        assert_eq!((94, 34), claw_machine.a);
        assert_eq!((74, 34), claw_machine.b);
        assert_eq!((104, 32324), claw_machine.prize);
    }

    #[test]
    fn should_create_claw_machines_from_input() {
        let input = "Button A: X+94, Y+34\nButton B: X+74, Y+34\nPrize: X+104, Y+32324\n\nButton A: X+94, Y+34\nButton B: X+74, Y+34\nPrize: X+104, Y+32324";

        let claw_machines = crate::create_claw_machines(input);

        assert_eq!(2, claw_machines.len());
    }
}
