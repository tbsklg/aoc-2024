use itertools::Itertools as _;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let schematics = input.split("\n\n").map(Schematic::from).collect::<Vec<_>>();

    let keys = schematics.iter().filter_map(|s| match s {
        Schematic::Key(v) => Some(v),
        Schematic::Lock(_) => None,
    }).collect::<Vec<_>>();

    let locks = schematics.iter().filter_map(|s| match s {
        Schematic::Key(_) => None,
        Schematic::Lock(v) => Some(v),
    }).collect::<Vec<_>>();

    locks.iter().cartesian_product(keys.iter())
        .filter(|(lock, key)| fits(*lock, *key))
        .count()
}

fn fits(lock: &[u8;5], key: &[u8;5]) -> bool {
    key.iter().zip(lock.iter()).all(|(&k, l)| k < 6 - l)
}

#[derive(Debug)]
enum Schematic {
    Key([u8; 5]),
    Lock([u8; 5]),
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let first = lines.next().unwrap();
        let last = lines.last().unwrap();

        let is_lock = first.chars().all(|c| c == '#') && last.chars().all(|c| c == '.');

        if is_lock {
            Schematic::Lock(extract_schema(value))
        } else {
            Schematic::Key(extract_schema(value))
        }
    }
}

fn extract_schema(input: &str) -> [u8; 5] {
    input
        .lines()
        .skip(1)
        .take(5)
        .fold([0, 0, 0, 0, 0], |mut acc, curr| {
            curr.chars()
                .enumerate()
                .for_each(|(i, v)| acc[i] += if v == '#' { 1 } else { 0 });

            acc
        })
}
