fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 1: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let disk = &mut Disk::from(input);
    disk.compress();
    disk.checksum_data()
}

fn part2(input: &str) -> usize {
    let disk = &mut Disk::from(input);
    disk.compress_blocks();
    disk.checksum_blocks()
}

#[derive(Debug)]
struct Disk {
    data: Vec<Option<u32>>,
    blocks: Vec<(Option<u32>, u32)>,
}

impl Disk {
    fn checksum_data(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(i, o)| i * o.unwrap_or(0) as usize)
            .sum()
    }

    fn checksum_blocks(&self) -> usize {
        let mut checksum: usize = 0;
        let mut index: usize = 0;

        for block in self.blocks.iter() {
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
}

impl From<&str> for Disk {
    fn from(input: &str) -> Self {
        let mut data: Vec<Option<u32>> = vec![];
        let mut blocks: Vec<(Option<u32>, u32)> = vec![];

        let raw_data = input.trim_end().chars().map(|x| x.to_digit(10)).enumerate();
        let mut index = 0;
        for (i, v) in raw_data {
            if is_even(&i) {
                for _ in 0..v.unwrap() {
                    data.push(Some(index));
                }
                blocks.push((Some(index), v.unwrap()));
                index += 1;
            } else {
                for _ in 0..v.unwrap() {
                    data.push(None);
                }
                blocks.push((None, v.unwrap()));
            }
        }

        Self { data, blocks }
    }
}

impl Disk {
    fn compress(&mut self) {
        let mut x = 0;
        let mut y = self.data.len() - 1;

        while x != y {
            match self.data[x] {
                Some(_) => {
                    x += 1;
                    continue;
                }
                None => match self.data[y] {
                    Some(_) => {
                        self.data[x] = self.data.remove(y);
                        y -= 1;
                    }
                    None => {
                        self.data.remove(y);
                        y -= 1;
                    }
                },
            }
        }
    }

    fn compress_blocks(&mut self) {
        let mut y = self.blocks.len() - 1;

        while y != 0 {
            let data = self.blocks[y];
            match data.0 {
                Some(_) => {
                    let block_index = self
                        .blocks
                        .iter()
                        .take(y)
                        .position(|b| b.0.is_none() && b.1 >= data.1);

                    match block_index {
                        Some(i) => {
                            let block = self.blocks[i];

                            match block.1.cmp(&data.1) {
                                std::cmp::Ordering::Equal => {
                                    self.blocks[i] = self.blocks[y];
                                    self.blocks[y] = (None, data.1);
                                    y -= 1;
                                }
                                std::cmp::Ordering::Greater => {
                                    self.blocks[i] = self.blocks[y];
                                    self.blocks[y] = (None, data.1);

                                    self.blocks.insert(i + 1, (None, block.1 - data.1));
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
}

fn is_even(i: &usize) -> bool {
    i % 2 == 0
}
