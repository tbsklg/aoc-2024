fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    Calibration::from(input)
        .equations
        .iter()
        .filter(|v| is_correct_equation(v, vec![add, multiply]))
        .map(|v| v.0)
        .sum()
}

fn part2(input: &str) -> usize {
    Calibration::from(input)
        .equations
        .iter()
        .filter(|v| is_correct_equation(v, vec![add, multiply, concat]))
        .map(|v| v.0)
        .sum()
}

fn add(a: usize, b: usize) -> usize {
    a + b
}

fn multiply(a: usize, b: usize) -> usize {
    a * b
}

fn concat(a: usize, b: usize) -> usize {
    format!("{}{}", a, b).parse().unwrap()
}

fn is_correct_equation(
    input: &(usize, Vec<usize>),
    operations: Vec<fn(usize, usize) -> usize>,
) -> bool {
    fn can_form_target(
        numbers: &[usize],
        target: usize,
        current_value: usize,
        operations: &[fn(usize, usize) -> usize],
    ) -> bool {
        if current_value > target {
            return false;
        }

        if numbers.is_empty() {
            return current_value == target;
        }

        let (first, rest) = numbers.split_first().unwrap();

        operations.iter().any(|operation| {
            can_form_target(rest, target, operation(current_value, *first), operations)
        })
    }

    let (target, numbers) = input;
    can_form_target(numbers, *target, 0, &operations)
}

type Equation = (usize, Vec<usize>);

#[derive(Debug)]
struct Calibration {
    equations: Vec<Equation>,
}

impl From<&str> for Calibration {
    fn from(input: &str) -> Self {
        let equations = input
            .lines()
            .map(|l| l.split(':').collect::<Vec<&str>>())
            .map(|l| {
                (
                    l[0].parse::<usize>().unwrap(),
                    l[1].split_whitespace()
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>(),
                )
            })
            .collect::<Vec<(usize, Vec<usize>)>>();

        Self { equations }
    }
}
