fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    Grid::from(input).xmas_count()
}

fn part2(input: &str) -> usize {
    Grid::from(input).crossmas_count()
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

        let (rows, cols) = (grid.len(), grid.first().map_or(0, |l| l.len()));

        Self { grid, rows, cols }
    }
}

impl Grid {
    fn crossmas_count(&mut self) -> usize {
        self.find_positions_for('A')
            .into_iter()
            .filter(|p| self.crossmas_at(*p))
            .count()
    }

    fn crossmas_at(&mut self, (row, col): (usize, usize)) -> bool {
        let (row, col) = (row as isize, col as isize);
        let lt_br = [self.get(row - 1, col - 1), self.get(row + 1, col + 1)];
        let rt_bl = [self.get(row - 1, col + 1), self.get(row + 1, col - 1)];

        [lt_br, rt_bl]
            .iter()
            .all(|w| *w == ['M', 'S'] || *w == ['S', 'M'])
    }

    fn xmas_count(&mut self) -> usize {
        self.find_positions_for('X')
            .into_iter()
            .map(|p| self.xmas_counts_at(p))
            .sum()
    }

    fn xmas_counts_at(&mut self, (row, col): (usize, usize)) -> usize {
        [
            (0, 1),
            (1, 0),
            (1, 1),
            (0, -1),
            (-1, 0),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ]
        .iter()
        .filter(|(offset_row, offset_col)| {
            (0..4).all(|i| {
                let next_row = row as isize + i as isize * offset_row;
                let next_col = col as isize + i as isize * offset_col;

                self.get(next_row, next_col) == "XMAS".chars().nth(i).expect("")
            })
        })
        .count()
    }

    fn get(&mut self, row: isize, col: isize) -> char {
        *self
            .grid
            .get(row as usize)
            .and_then(|r| r.get(col as usize))
            .unwrap_or(&'#')
    }

    fn find_positions_for(&mut self, c: char) -> Vec<(usize, usize)> {
        (0..self.rows)
            .flat_map(|row| (0..self.cols).map(move |col| (row, col)))
            .filter(|&(row, col)| self.grid[row][col] == c)
            .collect()
    }
}
