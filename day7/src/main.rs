fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> u32 {
    Calibration::from(input)
        .equations
        .iter()
        .filter(|v| is_correct_equation(*v))
        .map(|v| v.0)
        .sum()
}

fn is_correct_equation(v: &(u32, Vec<u32>)) -> bool {
    true
}

type Equation = (u32, Vec<u32>);

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
                    l[0].parse::<u32>().unwrap(),
                    l[1].split_whitespace()
                        .map(|v| v.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                )
            })
            .collect::<Vec<(u32, Vec<u32>)>>();

        Self { equations }
    }
}
