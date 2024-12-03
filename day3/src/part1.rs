use regex::Regex;

pub fn solve(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]*,[0-9]*)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            cap[1]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .product::<usize>()
        })
        .sum::<usize>()
}
