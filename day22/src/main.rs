use std::{collections::{HashMap, HashSet}, iter::{once, repeat_with}};

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
    find_best_sequence_for_bananas(input)
}

fn find_best_sequence_for_bananas(input: &str) -> usize {
    todo!()
    
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

fn price_changes(x: usize) -> impl Iterator<Item = i32> {
    price(x).tuple_windows().map(|(previous, current)| {
        current as i32 - previous as i32
    })
}


#[cfg(test)]
mod tests {
    use crate::{mix, next_secret_numbers, price_changes, prune, to_next_secret_number};

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
    fn should_calc_price_changes() {
        let changes = price_changes(123).take(10).collect::<Vec<i32>>();
        assert_eq!(vec![-3, 6, -1, -1, 0, 2, -2, 0, -2, 2], changes)
    }
}
