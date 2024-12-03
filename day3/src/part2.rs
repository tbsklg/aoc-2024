use regex::Regex;

use crate::part1;

pub fn solve(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d+,\d+\))|do\(\)|don't\(\)").unwrap();

    let mut captures = vec![];
    let mut capture = true;
    for cap in re.captures_iter(input) {
        if &cap[0] == "do()" {
            capture = true;
            continue;
        }

        if &cap[0] == "don't()" {
            capture = false;
            continue;
        }

        if capture {
            captures.push(cap[0].to_owned());
        }
    }

    part1::solve(&captures.concat())
}
