use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    part1(&input);
}

fn part1(input: &str) -> usize {
    let machines = create_claw_machines(input);

    println!("{:#?}", machines);
    2
}

fn create_claw_machines(input: &str) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .map(ClawMachine::from)
        .collect::<Vec<ClawMachine>>()
}

#[derive(Debug)]
struct ClawMachine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

impl From<&str> for ClawMachine {
    fn from(value: &str) -> Self {
        assert!(value.lines().count() == 3);

        let numbers = value
            .lines()
            .map(extract_numbers)
            .collect::<Vec<(usize, usize)>>();

        Self {
            a: numbers[0],
            b: numbers[1],
            prize: numbers[2],
        }
    }
}

fn extract_numbers(line: &str) -> (usize, usize) {
    let re = Regex::new(r"\b\d+\b").unwrap();
    let caps = re
        .find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect::<Vec<usize>>();

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
