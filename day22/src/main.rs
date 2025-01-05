use std::iter::repeat_with;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .map(|sn| next_secret_numbers(sn).nth(1999).unwrap())
        .sum()
}

fn mix(x: usize, y: usize) -> usize {
    x ^ y
}

fn prune(x: usize) -> usize {
    x % 16777216
}

fn to_next_secret_number(x: usize) -> usize {
    let a = prune(mix(x, x * 64));
    let b = prune(mix(a, a / 32));
    prune(mix(b, b * 2048))
}

fn next_secret_numbers(mut x: usize) -> impl Iterator<Item = usize> {
    repeat_with(move || {
        x = to_next_secret_number(x);
        x
    })
}

#[cfg(test)]
mod tests {
    use crate::{mix, next_secret_numbers, prune, to_next_secret_number};

    #[test]
    fn should_mix() {
        assert_eq!(37, mix(42, 15));
    }

    #[test]
    fn should_prune() {
        assert_eq!(16113920, prune(100000000))
    }

    #[test]
    fn should_calc_next_secret() {
        assert_eq!(15887950, to_next_secret_number(123))
    }
    
    #[test]
    fn should_calc_next_secrets() {
        assert_eq!(15887950, next_secret_numbers(123).next().unwrap())
    }
    
    #[test]
    fn should_calc_2000_next_secrets() {
        assert_eq!(5908254, next_secret_numbers(123).nth(9).unwrap())
    }
}
