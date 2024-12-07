pub fn solve(input: &str) -> usize {
    Grid::from(input).count_xmas()
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
    fn count_xmas(&mut self) -> usize {
        self.find_positions_for('X')
            .into_iter()
            .map(|p| self.count_occurrence(p))
            .sum()
    }

    fn count_occurrence(&mut self, (row, col): (usize, usize)) -> usize {
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
        *self.grid
            .get(row as usize)
            .and_then(|r| r.get(col as usize))
            .unwrap_or(&'.')
    }

    fn find_positions_for(&mut self, c: char) -> Vec<(usize, usize)> {
        (0..self.rows)
            .flat_map(|row| (0..self.cols).map(move |col| (row, col)))
            .filter(|&(row, col)| self.grid[row][col] == c)
            .collect()
    }
}
