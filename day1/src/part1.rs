use std::iter::zip;

pub fn solve(input: &str) -> u64 {
    let numbers = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>();

    let mut lhs = numbers.iter().map(|v| v[0]).collect::<Vec<_>>();
    let mut rhs = numbers.iter().map(|v| v[1]).collect::<Vec<_>>();

    lhs.sort();
    rhs.sort();

    zip(&lhs, &rhs)
        .map(|(l, r)| l.abs_diff(*r))
        .collect::<Vec<u64>>()
        .iter()
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::solve;

    #[test]
    fn should_solve() {
        assert_eq!(
            1765812,
            solve(&std::fs::read_to_string("input.txt").unwrap())
        );
    }
}
