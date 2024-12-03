fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", day3::part1::solve(&input));
    println!("Part 2: {}", day3::part2::solve(&input));
}
