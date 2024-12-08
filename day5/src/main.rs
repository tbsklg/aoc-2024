fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    Manual::from(input).correct_ordered_pages();
    1
}

#[derive(Debug)]
struct Manual {
    rules: Vec<(u32, u32)>,
    pages: Vec<Vec<u32>>,
}
impl Manual {
    fn correct_ordered_pages(&self) -> usize {
        self.pages
            .iter()
            .filter(|p| self.correct_ordered(p))
            .count()
    }

    fn correct_ordered(&self, page: &Vec<u32>) -> bool {
        false
    }
}

impl From<&str> for Manual {
    fn from(input: &str) -> Self {
        let manual: Vec<&str> = input.split("\n\n").collect();
        let rules = manual[0]
            .lines()
            .map(|l| {
                let parts: Vec<u32> = l.split('|').filter_map(|s| s.parse().ok()).collect();
                (parts[0], parts[1])
            })
            .collect();

        let pages = manual[1]
            .lines()
            .map(|l| l.split(',').filter_map(|s| s.parse::<u32>().ok()).collect())
            .collect::<Vec<Vec<u32>>>();

        Manual { rules, pages }
    }
}
