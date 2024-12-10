fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    Calibration::from(input)
        .equations
        .iter()
        .filter(|v| is_correct_equation(*v))
        .map(|v| v.0)
        .sum()
}

fn is_correct_equation(input: &(usize, Vec<usize>)) -> bool {
    fn find(v: Vec<usize>, target: usize, current: usize) -> bool {
        if current > target {
            return false;
        }

        if v.is_empty() {
            return current == target;
        }

        let head = v.first().unwrap();
        let tail = v.iter().skip(1).map(|v| *v).collect::<Vec<usize>>();

        find(tail.clone(), target, current + head) || find(tail.clone(), target, current * head)
    }

    find(input.1.clone(), input.0, 0)
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
