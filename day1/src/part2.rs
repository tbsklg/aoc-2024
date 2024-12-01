use std::collections::HashMap;

pub fn solve(input: &String) -> u64 {
    let l = input
        .lines()
        .map(|l| {
            l.split("   ")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>();

    let rhs = l
        .iter()
        .map(|v| v[1])
        .fold(HashMap::new(), |mut counts, c| {
            let current = counts.get(&c).unwrap_or(&0);
            counts.insert(c, current + 1);
            counts
        });

    l.iter()
        .map(|v| v[0])
        .map(|x| x * rhs.get(&x).unwrap_or(&0))
        .sum()
}
