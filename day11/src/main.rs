use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let initial_stones = &mut create_stones(input);
    blink(initial_stones, 25);
    initial_stones.values().sum()
}

fn part2(input: &str) -> usize {
    let initial_stones = &mut create_stones(input);
    blink(initial_stones, 75);
    initial_stones.values().sum()
}

fn blink(stones: &mut HashMap<usize, usize>, times: u8) {
    for _ in 0..times {
        let mut new_stones = HashMap::new();
        for (stone, count) in stones.iter() {
            for new_stone in transform_stone(*stone) {
                let new_count = new_stones.entry(new_stone).or_insert(0);
                *new_count += count;
            }
        }
        *stones = new_stones;
    }
}

fn transform_stone(stone: usize) -> Vec<usize> {
    match stone {
        0 => vec![1],
        i => {
            if has_even_digits(&i) {
                return replace_with_two(&i);
            }

            vec![i * 2024]
        }
    }
}

fn replace_with_two(i: &usize) -> Vec<usize> {
    let s = i.to_string();
    let (left, right) = s.split_at(s.len() / 2);
    vec![
        left.parse::<usize>().unwrap(),
        right.parse::<usize>().unwrap(),
    ]
}

fn has_even_digits(i: &usize) -> bool {
    is_even(i.to_string().len())
}

fn is_even(i: usize) -> bool {
    i % 2 == 0
}

fn create_stones(input: &str) -> HashMap<usize, usize> {
    input
        .split_whitespace()
        .map(|line| (line.parse::<usize>().unwrap(), 1))
        .collect::<HashMap<usize, usize>>()
}

#[cfg(test)]
mod tests {
    use crate::{has_even_digits, replace_with_two};

    #[test]
    fn should_split_stones_into_two() {
        assert_eq!(vec![10, 0], replace_with_two(&1000));
    }

    #[test]
    fn should_check_if_stone_has_even_digits() {
        assert_eq!(false, has_even_digits(&123));
        assert_eq!(true, has_even_digits(&1234));
    }
}
