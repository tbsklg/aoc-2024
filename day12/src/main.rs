use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    calculate_price(Map::from(input).find_areas())
}

struct Map {
    area: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let area = value
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let rows = area.len();
        let cols = area.first().map_or(0, |l| l.len());

        Self { area, rows, cols }
    }
}

fn calculate_price(areas: Vec<(char, usize, usize)>) -> usize {
    areas
        .iter()
        .fold(0, |acc, (_, size, perimeter)| acc + size * perimeter)
}

impl Map {
    fn find_areas(&self) -> Vec<(char, usize, usize)> {
        (0..self.rows as i32)
            .flat_map(|r| (0..self.cols as i32).map(move |c| (r, c)))
            .fold((HashSet::new(), Vec::new()), |(mut acc, mut areas), p| {
                if acc.contains(&p) {
                    return (acc, areas);
                }

                let area = self.find_area(p);
                acc.extend(area.0.iter().copied());
                areas.push((self.get(&p).unwrap(), area.0.len(), area.1));

                (acc, areas)
            })
            .1
    }

    fn find_area(&self, position: (i32, i32)) -> (HashSet<(i32, i32)>, usize) {
        let mut visited = HashSet::new();
        let mut stack = vec![position];
        let mut perimeter: usize = 0;

        while let Some(p) = stack.pop() {
            if !visited.insert(p) {
                continue;
            }

            let neighbors = self.neighbors(&p);

            perimeter += 4 - neighbors.len();

            for n in neighbors {
                stack.push(n)
            }
        }

        (visited, perimeter)
    }

    fn neighbors(&self, (r, c): &(i32, i32)) -> Vec<(i32, i32)> {
        let area = self.get(&(*r, *c)).unwrap();
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(dr, dc)| (dr + r, dc + c))
            .filter(|p| self.in_bounds(p))
            .filter(|(nr, nc)| {
                let n_area = self.get(&(*nr, *nc)).unwrap();
                area == n_area
            })
            .collect()
    }

    fn get(&self, p: &(i32, i32)) -> Option<char> {
        self.area
            .get(p.0 as usize)
            .and_then(|l| l.get(p.1 as usize))
            .copied()
    }

    fn in_bounds(&self, (r, c): &(i32, i32)) -> bool {
        *r >= 0 && *r < self.rows as i32 && *c >= 0 && *c < self.cols as i32
    }
}
