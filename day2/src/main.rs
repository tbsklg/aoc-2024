fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", day2::part1::solve(&input));
    println!("Part 2: {}", day2::part2::solve(&input));
}
