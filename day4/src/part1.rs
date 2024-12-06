pub fn solve(input: &str) -> usize {
    find_x_positions(input)
        .into_iter()
        .map(|p| horizontal(p, input) + vertical(p, input) + diagonal(p, input))
        .sum()
}

fn find_x_positions(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'X')
                .map(move |(col, _)| (row as usize, col as usize))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>()
}

fn horizontal((r, c): (usize, usize), input: &str) -> usize {
    let line = input.lines().nth(r as usize).unwrap();

    match (
        line.get(c.saturating_sub(3)..=c),
        line.get(c..=(c.saturating_add(3))),
    ) {
        (Some("SAMX"), Some("XMAS")) => 2,
        (Some("SAMX"), _) => 1,
        (_, Some("XMAS")) => 1,
        _ => 0,
    }
}

fn vertical((r, c): (usize, usize), input: &str) -> usize {
    let line: String = input.lines().map(|l| l.chars().nth(c).unwrap()).collect();

    match (
        line.get(r.saturating_sub(3)..=r),
        line.get(r..=(r.saturating_add(3))),
    ) {
        (Some("SAMX"), Some("XMAS")) => 2,
        (Some("SAMX"), _) => 1,
        (_, Some("XMAS")) => 1,
        _ => 0,
    }
}

fn diagonal((r, c): (usize, usize), input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut left_top_bottom_right = String::new();

    for offset in -3..=3 {
        let new_r = (r as isize + offset) as usize;
        let new_c = (c as isize + offset) as usize;

        if let Some(line) = lines.get(new_r) {
            if let Some(ch) = line.chars().nth(new_c) {
                left_top_bottom_right.push(ch);
            }
        }
    }

    let mut bottom_left_to_top_right = String::new();
    for offset in -3..=3 {
        let new_r = (r as isize + offset) as usize; // Move downwards
        let new_c = (c as isize - offset) as usize; // Move leftwards

        if let Some(line) = lines.get(new_r) {
            if let Some(ch) = line.chars().nth(new_c) {
                bottom_left_to_top_right.push(ch);
            }
        }
    }

    let mut count = 0;

    if left_top_bottom_right.starts_with("SAMX") {
        count += 1;
    };

    if left_top_bottom_right.ends_with("XMAS") {
        count += 1;
    };

    if bottom_left_to_top_right.starts_with("SAMX") {
        count += 1;
    }

    if bottom_left_to_top_right.ends_with("XMAS") {
        count += 1;
    }

    count
}
