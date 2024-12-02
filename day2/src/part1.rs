use std::iter::zip;

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(is_safe)
        .count()
}

fn is_safe(report: &Vec<u32>) -> bool {
    let s = zip(report, &report[1..]);
    let all_increasing = s.clone().all(|(f, s)| f <= s);
    let all_decreasing = s.clone().all(|(f, s)| f >= s);

    (all_increasing || all_decreasing) 
    && s.map(|(f, s)| f.abs_diff(*s)).all(|d| d > 0 && d < 4)
}
