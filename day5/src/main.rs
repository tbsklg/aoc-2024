fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> u32 {
    Manual::from(input).page_order_score()
}

#[derive(Debug)]
struct Manual {
    rules: Vec<(u32, u32)>,
    pages: Vec<Vec<u32>>,
}
impl Manual {
    fn page_order_score(&self) -> u32 {
        self.pages
            .iter()
            .filter(|p| self.correct_ordered(p))
            .map(|p| *p.get(p.len() / 2).unwrap())
            .sum()
    }

    fn correct_ordered(&self, page: &Vec<u32>) -> bool {
        self.rules.iter().all(|(a, b)| {
            let a = page.iter().position(|&x| x == *a);
            let b = page.iter().position(|&x| x == *b);
            match (a, b) {
                (Some(a), Some(b)) => a < b,
                _ => true,
            }
        })
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
