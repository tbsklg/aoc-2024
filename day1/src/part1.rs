use std::iter::zip;

pub fn solve(input: &String) -> u64 {
    let l = input
        .lines()
        .map(|l| {
            l.split("   ")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>();

    let mut lhs = l.iter().map(|v| v[0]).collect::<Vec<_>>();
    let mut rhs = l.iter().map(|v| v[1]).collect::<Vec<_>>();

    lhs.sort();
    rhs.sort();

    zip(&lhs, &rhs)
        .map(|(l, r)| l.abs_diff(*r))
        .collect::<Vec<u64>>()
        .iter()
        .sum()
}
