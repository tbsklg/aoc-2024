use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn part1(input: &str) -> String {
    let register = &mut extract_register(input);
    let program = extract_program(input);

    run(register, &program, 0, &mut vec![])
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

// Thanks to https://github.com/hyperneutrino for providing a solution
// see https://www.youtube.com/watch?v=y-UPxMAh2N8
fn part2(input: &str) -> usize {
    fn find_a(program: &[usize], ans: usize) -> Option<usize> {
        if program.is_empty() {
            return Some(ans);
        }

        let last = *program.last().unwrap();

        for i in 0..8 {
            let a = ans << 3 | i;
            let mut b = a % 8;
            b ^= 1;
            let c = a >> b;
            b ^= 5;
            b ^= c;
            if b % 8 == last {
                let next_program = &program[..program.len() - 1];
                if let Some(result) = find_a(next_program, a) {
                    return Some(result);
                }
            }
        }

        None
    }

    let program = extract_program(input);
    find_a(&program, 0).unwrap()
}

fn extract_register(input: &str) -> Register {
    let re = Regex::new(r"\d+").unwrap();

    let lines = &mut input.lines();
    let a = re.captures(lines.next().unwrap()).unwrap();
    let b = re.captures(lines.next().unwrap()).unwrap();
    let c = re.captures(lines.next().unwrap()).unwrap();

    HashMap::from([
        ('A', a.get(0).unwrap().as_str().parse().unwrap()),
        ('B', b.get(0).unwrap().as_str().parse().unwrap()),
        ('C', c.get(0).unwrap().as_str().parse().unwrap()),
    ])
}

fn extract_program(input: &str) -> Vec<usize> {
    let re = Regex::new(r"\d+").unwrap();

    let input = input.split("\n\n").collect::<Vec<&str>>();

    re.captures_iter(input[1])
        .map(|c| c.get(0).unwrap().as_str().parse().unwrap())
        .collect()
}

fn run(
    register: &mut Register,
    program: &Vec<usize>,
    pointer: usize,
    output: &mut Vec<usize>,
) -> Vec<usize> {
    let opcode = program.get(pointer).unwrap();
    let operand = program.get(pointer + 1).unwrap();

    let mut jumper = None;
    match opcode {
        0 => adv(register, combo(register, *operand)),
        1 => bxl(register, *operand),
        2 => bst(register, combo(register, *operand)),
        3 => jumper = jnz(register, *operand),
        4 => bxc(register),
        5 => output.push(out(combo(register, *operand))),
        6 => bdv(register, combo(register, *operand)),
        7 => cdv(register, combo(register, *operand)),
        _ => todo!(),
    }

    let next_pointer = jumper.map_or(pointer + 2, |j| j);

    if next_pointer < program.len() {
        run(register, program, next_pointer, output)
    } else {
        output.to_vec()
    }
}

type Register = HashMap<char, usize>;

fn combo(register: &Register, operand: usize) -> usize {
    match operand {
        4 => *register.get(&'A').unwrap(),
        5 => *register.get(&'B').unwrap(),
        6 => *register.get(&'C').unwrap(),
        _ => operand,
    }
}

fn adv(register: &mut Register, operand: usize) {
    let result = register.get(&'A').unwrap() / (2_usize.pow(operand as u32));
    register.insert('A', result);
}

fn bdv(register: &mut Register, operand: usize) {
    let result = register.get(&'A').unwrap() / (2_usize.pow(operand as u32));
    register.insert('B', result);
}

fn cdv(register: &mut Register, operand: usize) {
    let result = register.get(&'A').unwrap() / (2_usize.pow(operand as u32));
    register.insert('C', result);
}

fn bxl(register: &mut Register, operand: usize) {
    let b = register.get(&'B').unwrap();
    register.insert('B', b ^ operand);
}

fn bst(register: &mut Register, operand: usize) {
    register.insert('B', operand & 7);
}

fn jnz(register: &mut Register, operand: usize) -> Option<usize> {
    let a = register.get(&'A').unwrap();

    if *a == 0 {
        None
    } else {
        Some(operand)
    }
}

fn bxc(register: &mut Register) {
    let b = register.get(&'B').unwrap();
    let c = register.get(&'C').unwrap();

    register.insert('B', b ^ c);
}

fn out(operand: usize) -> usize {
    operand & 7
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::run;

    #[test]
    fn should_set_register_b_to_1() {
        let register = &mut HashMap::from([('C', 9)]);
        let program = vec![2, 6];

        run(register, program, 0, &mut vec![]);
        assert_eq!(Some(&1), register.get(&'B'));
    }

    #[test]
    fn should_print_output() {
        let register = &mut HashMap::from([('A', 10)]);
        let program = vec![5, 0, 5, 1, 5, 4];

        let output = run(register, program, 0, &mut vec![]);

        assert_eq!(Some("0,1,2"), output.as_deref());
    }

    #[test]
    fn should_output_larger_example() {
        let register = &mut HashMap::from([('A', 2024)]);
        let program = vec![0, 1, 5, 4, 3, 0];

        let output = run(register, program, 0, &mut vec![]);

        assert_eq!(Some("4,2,5,6,7,7,7,7,3,1,0"), output.as_deref());
        assert_eq!(Some(&0), register.get(&'A'));
    }

    #[test]
    fn should_check_register_b() {
        let register = &mut HashMap::from([('B', 29)]);
        let program = vec![1, 7];

        run(register, program, 0, &mut vec![]);

        assert_eq!(Some(&26), register.get(&'B'));
    }

    #[test]
    fn should_check_register_b_and_c() {
        let register = &mut HashMap::from([('B', 2024), ('C', 43690)]);
        let program = vec![4, 0];

        run(register, program, 0, &mut vec![]);

        assert_eq!(Some(&44354), register.get(&'B'));
    }
}
