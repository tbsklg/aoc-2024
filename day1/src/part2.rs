use std::collections::HashMap;

pub fn solve(input: &String) -> u64 {
    let numbers = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>();

    let counts = numbers
        .iter()
        .map(|v| v[1])
        .fold(HashMap::new(), |mut counts, c| {
            let current_count = counts.get(&c).unwrap_or(&0);
            counts.insert(c, current_count + 1);
            counts
        });

    numbers.iter()
        .map(|v| v[0])
        .map(|x| x * counts.get(&x).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::solve;
    
    #[test]
    fn should_solve() {
        assert_eq!(
            20520794,
            solve(&std::fs::read_to_string("input.txt").unwrap())
        );
    }
}

