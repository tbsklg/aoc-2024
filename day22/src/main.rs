use std::{
    collections::HashMap,
    iter::{once, repeat_with},
};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    extract_numbers(input)
        .map(|sn| next_secret_numbers(sn).nth(2000).unwrap())
        .sum()
}

fn part2(input: &str) -> usize {
    extract_numbers(input)
        .flat_map(sequences)
        .fold(HashMap::new(), |mut acc, (k, v)| {
            *acc.entry(k).or_insert(0) += v;
            acc
        })
        .values()
        .max()
        .copied()
        .unwrap_or(0) // Default to
}

fn sequences(n: usize) -> HashMap<Vec<isize>, usize> {
    let prices = price(n).take(2001).collect::<Vec<_>>();

    prices
        .iter()
        .tuple_windows()
        .map(|(previous, current)| *current as isize - *previous as isize)
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
        .fold(HashMap::new(), |mut sequences, (i, w)| {
            let key = w.to_vec();
            sequences.entry(key).or_insert(prices[i + 4]);
            sequences
        })
}

fn extract_numbers(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().filter_map(|l| l.parse::<usize>().ok())
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
    once(x).chain(repeat_with(move || {
        x = to_next_secret_number(x);
        x
    }))
}

fn price(x: usize) -> impl Iterator<Item = usize> {
    next_secret_numbers(x).map(|x| x % 10)
}

#[cfg(test)]
mod tests {
    use crate::{mix, next_secret_numbers, prune, sequences, to_next_secret_number};

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
        assert_eq!(123, next_secret_numbers(123).next().unwrap())
    }

    #[test]
    fn should_calc_2000_next_secrets() {
        assert_eq!(5908254, next_secret_numbers(123).nth(10).unwrap())
    }

    #[test]
    fn should_calc_sequences() {
        let s = sequences(123);

        assert_eq!(&6, s.get(&[-1, -1, 0, 2].to_vec()).unwrap());
    }

    #[test]
    fn should_calc_another_sequences() {
        let s = sequences(1);

        assert_eq!(&7, s.get(&[-2, 1, -1, 3].to_vec()).unwrap());
    }
}
