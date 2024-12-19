use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    calculate_price(Map::from(input).find_areas())
}

fn part2(input: &str) -> usize {
    calculate_price_2(Map::from(input).find_areas())
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

fn calculate_price(areas: Vec<Area>) -> usize {
    areas
        .iter()
        .fold(0, |acc, area| acc + area.points.len() * area.perimeter)
}

fn calculate_price_2(areas: Vec<Area>) -> usize {
    areas
        .iter()
        .map(|area| area.points.len() * sides(&area.points.iter().copied().collect::<Vec<_>>()))
        .sum()
}

struct Area {
    points: HashSet<(i32, i32)>,
    perimeter: usize,
}

impl Map {
    fn find_areas(&self) -> Vec<Area> {
        (0..self.rows as i32)
            .flat_map(|r| (0..self.cols as i32).map(move |c| (r, c)))
            .fold((HashSet::new(), Vec::new()), |(mut acc, mut areas), p| {
                if acc.contains(&p) {
                    return (acc, areas);
                }

                let area = self.find_area(p);
                acc.extend(area.points.iter().copied());
                areas.push(area);

                (acc, areas)
            })
            .1
    }

    fn find_area(&self, position: (i32, i32)) -> Area {
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

        Area {
            points: visited,
            perimeter,
        }
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

fn sides(points: &[(i32, i32)]) -> usize {
    let mut sides = 0;

    for point in points {
        let neighbors = [
            (point.0 + 1, point.1),
            (point.0 - 1, point.1),
            (point.0, point.1 + 1),
            (point.0, point.1 - 1),
        ]
        .into_iter()
        .filter(|p| points.contains(p))
        .collect::<Vec<_>>();

        if neighbors.is_empty() {
            sides += 4;
            continue;
        }

        if neighbors.len() == 1 {
            sides += 2;
            continue;
        }

        let right = neighbors
            .iter()
            .find(|p| p.0 == point.0 && p.1 == point.1 + 1);
        let bottom = neighbors
            .iter()
            .find(|p| p.0 == point.0 + 1 && p.1 == point.1);
        let left = neighbors
            .iter()
            .find(|p| p.0 == point.0 && p.1 == point.1 - 1);
        let top = neighbors
            .iter()
            .find(|p| p.0 == point.0 - 1 && p.1 == point.1);

        let mut sides_for_p = 0;

        match (bottom, left, right) {
            (Some(_), Some(_), Some(_)) => {
                // max 2 corner for left and right
                sides_for_p += points
                    .iter()
                    .find(|p| p.0 == point.0 + 1 && p.1 == point.1 - 1)
                    .map_or(1, |_| 0);
                sides_for_p += points
                    .iter()
                    .find(|p| p.0 == point.0 + 1 && p.1 == point.1 + 1)
                    .map_or(1, |_| 0);
            }
            (Some(_), Some(_), None) => {
                sides_for_p += top.map_or(1, |_| 0);
                sides_for_p += points
                    .iter()
                    .find(|p| p.0 == point.0 + 1 && p.1 == point.1 - 1)
                    .map_or(1, |_| 0);
            }
            (Some(_), None, Some(_)) => {
                sides_for_p += top.map_or(1, |_| 0);
                sides_for_p += points
                    .iter()
                    .find(|p| p.0 == point.0 + 1 && p.1 == point.1 + 1)
                    .map_or(1, |_| 0);
            }
            _ => (),
        };

        match (top, left, right) {
            (Some(_), Some(_), Some(_)) => {
                sides_for_p += points
                    .iter()
                    .find(|p| p.0 == point.0 - 1 && p.1 == point.1 + 1)
                    .map_or(1, |_| 0);

                sides_for_p += points
                    .iter()
                    .find(|p| p.0 == point.0 - 1 && p.1 == point.1 - 1)
                    .map_or(1, |_| 0);
            }
            (Some(_), Some(_), None) => {
                sides_for_p += bottom.map_or(1, |_| 0);
                sides_for_p += points
                    .iter()
                    .find(|p| p.0 == point.0 - 1 && p.1 == point.1 - 1)
                    .map_or(1, |_| 0);
            }
            (Some(_), None, Some(_)) => {
                sides_for_p += bottom.map_or(1, |_| 0);
                sides_for_p += points
                    .iter()
                    .find(|p| p.0 == point.0 - 1 && p.1 == point.1 + 1)
                    .map_or(1, |_| 0);
            }
            _ => (),
        }

        sides += sides_for_p;
    }
    sides
}

#[cfg(test)]
mod tests {
    use crate::sides;

    #[test]
    fn should_calculate_sides_when_no_points() {
        let points = vec![];

        assert_eq!(0, sides(&points));
    }

    #[test]
    fn should_calculate_when_only_horizontal() {
        let points = vec![(0, 0), (0, 1), (0, 2)];

        assert_eq!(4, sides(&points));
    }

    #[test]
    fn should_calculate_when_bottom_right() {
        let points = vec![(0, 0), (0, 1), (0, 2), (1, 0)];

        assert_eq!(6, sides(&points));
    }

    #[test]
    fn should_calculate_when_bottom_right_and_bottom_left() {
        let points = vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2)];

        assert_eq!(8, sides(&points));
    }

    #[test]
    fn should_calculate_sides_when_top_right() {
        let points = vec![(1, 0), (2, 0), (2, 1), (2, 2)];

        assert_eq!(6, sides(&points));
    }

    #[test]
    fn should_calculate_sides_when_top_right_and_top_left() {
        let points = vec![(1, 0), (2, 0), (2, 1), (2, 2), (1, 2)];

        assert_eq!(8, sides(&points));
    }
}
