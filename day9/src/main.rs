fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 1: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let disk = &mut create_disk(input);
    compress(disk);
    checksum_data(disk)
}

fn part2(input: &str) -> usize {
    let disk = &mut create_blocks(input);
    compress_blocks(disk);
    checksum_blocks(disk)
}

fn create_disk(input: &str) -> Vec<Option<u32>> {
    let mut data: Vec<Option<u32>> = vec![];

    let raw_data = input.trim_end().chars().map(|x| x.to_digit(10)).enumerate();
    let mut index = 0;
    for (i, v) in raw_data {
        if is_even(&i) {
            for _ in 0..v.unwrap() {
                data.push(Some(index));
            }
            index += 1;
        } else {
            for _ in 0..v.unwrap() {
                data.push(None);
            }
        }
    }
    data
}

fn create_blocks(input: &str) -> Vec<(Option<u32>, u32)> {
    let mut blocks: Vec<(Option<u32>, u32)> = vec![];

    let raw_data = input.trim_end().chars().map(|x| x.to_digit(10)).enumerate();
    let mut index = 0;
    for (i, v) in raw_data {
        if is_even(&i) {
            blocks.push((Some(index), v.unwrap()));
            index += 1;
        } else {
            blocks.push((None, v.unwrap()));
        }
    }

    blocks
}

fn compress(data: &mut Vec<Option<u32>>) {
    let mut x = 0;
    let mut y = data.len() - 1;

    while x != y {
        match data[x] {
            Some(_) => {
                x += 1;
                continue;
            }
            None => match data[y] {
                Some(_) => {
                    data[x] = data.remove(y);
                    y -= 1;
                }
                None => {
                    data.remove(y);
                    y -= 1;
                }
            },
        }
    }
}

fn compress_blocks(blocks: &mut Vec<(Option<u32>, u32)>) {
    let mut y = blocks.len() - 1;

    while y != 0 {
        let data = blocks[y];
        match data.0 {
            Some(_) => {
                let block_index = blocks
                    .iter()
                    .take(y)
                    .position(|b| b.0.is_none() && b.1 >= data.1);

                match block_index {
                    Some(i) => {
                        let block = blocks[i];

                        match block.1.cmp(&data.1) {
                            std::cmp::Ordering::Equal => {
                                blocks[i] = blocks[y];
                                blocks[y] = (None, data.1);
                                y -= 1;
                            }
                            std::cmp::Ordering::Greater => {
                                blocks[i] = blocks[y];
                                blocks[y] = (None, data.1);

                                blocks.insert(i + 1, (None, block.1 - data.1));
                            }
                            _ => {}
                        }
                    }
                    None => y -= 1,
                }
            }
            None => y -= 1,
        }
    }
}

fn checksum_data(data: &mut Vec<Option<u32>>) -> usize {
    data.iter()
        .enumerate()
        .map(|(i, o)| i * o.unwrap_or(0) as usize)
        .sum()
}

fn checksum_blocks(blocks: &mut Vec<(Option<u32>, u32)>) -> usize {
    let mut checksum: usize = 0;
    let mut index: usize = 0;

    for block in blocks.iter() {
        if block.0.is_none() {
            index += block.1 as usize;
        } else {
            for _ in 0..block.1 {
                checksum += index * block.0.unwrap() as usize;
                index += 1;
            }
        }
    }

    checksum
}

fn is_even(i: &usize) -> bool {
    i % 2 == 0
}
