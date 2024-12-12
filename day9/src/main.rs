fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let disk = Disk::from(input);
    println!("{:?}", disk.data);
    println!("{:?}", disk.free_space);
    1
}

#[derive(Debug)]
struct Data {
    id: u32,
    offset: u32,
    len: u32,
}

#[derive(Debug)]
struct Space {
    offset: u32,
    len: u32,
}

#[derive(Debug)]
struct Disk {
    data: Vec<Data>,
    free_space: Vec<Space>,
}

impl From<&str> for Disk {
    fn from(input: &str) -> Self {
        let mut data: Vec<Data> = vec![];
        let mut free_space: Vec<Space> = vec![];

        let mut id = 0..;
        let mut raw_data = input.trim_end().chars().map(|x| x.to_digit(10)).enumerate();

        let mut offset = 0;
        while let Some((i, v)) = raw_data.next() {
            if is_even(&i) {
                data.push(Data {
                    id: id.next().unwrap(),
                    offset,
                    len: v.unwrap_or(0),
                });
            } else {
                free_space.push(Space {
                    offset,
                    len: v.unwrap_or(0),
                })
            }
            offset += v.unwrap_or(0);
        }

        Self { data, free_space }
    }
}

fn is_even(i: &usize) -> bool {
    i % 2 == 0
}
