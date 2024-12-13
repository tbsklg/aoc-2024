fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let checksum = &mut Disk::from(input).compress_checksum();
    println!("{:?}", checksum);
    1
}

#[derive(Debug)]
struct Disk {
    data: Vec<Option<u32>>,
}

impl From<&str> for Disk {
    fn from(input: &str) -> Self {
        let mut data: Vec<Option<u32>> = vec![];

        let mut raw_data = input.trim_end().chars().map(|x| x.to_digit(10)).enumerate();

        let mut index = 0;
        while let Some((i, v)) = raw_data.next() {
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

        Self { data }
    }
}

impl Disk {
    fn compress_checksum(&mut self) -> usize {
        let mut x = 0;
        let mut y = self.data.len() - 1;

        while x != y {
            match self.data[x] {
                Some(_) => {
                    x += 1;
                    continue;
                },
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

        self.data.iter()
            .enumerate()
            .map(|(i, o)| i *o.unwrap_or(0) as usize)
            .sum()
    }
}

fn is_even(i: &usize) -> bool {
    i % 2 == 0
}
