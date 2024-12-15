fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    blink(create_stones(input), 25).len()
}

fn part2(input: &str) -> usize {
    blink(create_stones(input), 75).len()
}

fn blink(stones: Vec<usize>, times: u8) -> Vec<usize> {
    if times == 0 {
        return stones;
    }

    blink(
        stones
            .iter()
            .flat_map(|s| transform_stone(*s))
            .collect::<Vec<_>>(),
        times - 1,
    )
}

fn create_stones(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn transform_stone(stone: usize) -> Vec<usize> {
    match stone {
        0 => vec![1],
        i => {
            if has_even_digits(&i) {
                return replace_with_two(&i);
            }

            return vec![i * 2024];
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
