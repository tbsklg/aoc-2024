fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let map = Map::from(input);
    println!("{:?}", map.get((0,3)));
    1
}

struct Map {
    topographic: Vec<Vec<u8>>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let topographic = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        Map { topographic }
    }
}

impl Map {
    fn get(&self, (r, c): (usize, usize)) -> Option<u8> {
        self.topographic.get(r).and_then(|l| l.get(c)).copied()
    }
}
