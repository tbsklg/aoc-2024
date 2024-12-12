fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let disk = &mut Disk::from(input);
    disk.compress();
    println!("{:?}", disk.data);
    1
}

#[derive(Debug, Clone)]
struct Data {
    id: Option<u32>,
    offset: u32,
    len: u32,
}

impl Data {
    fn update_len(&mut self, len: u32) {
        self.len = len
    }
}

#[derive(Debug)]
struct Disk {
    data: Vec<Data>,
}

impl From<&str> for Disk {
    fn from(input: &str) -> Self {
        let mut data: Vec<Data> = vec![];

        let mut id = 0..;
        let mut raw_data = input.trim_end().chars().map(|x| x.to_digit(10)).enumerate();

        let mut offset = 0;
        while let Some((i, v)) = raw_data.next() {
            let len = v.unwrap_or(0);
            if is_even(&i) {
                data.push(Data {
                    id: Some(id.next().unwrap()),
                    offset,
                    len,
                });
            } else {
                data.push(Data {
                    id: None,
                    offset,
                    len,
                })
            }
            offset += len;
        }

        Self { data }
    }
}

impl Disk {
    fn compress(&mut self) {
        let spaces = 
            self.data.iter().filter(|d| d.id.is_none()).collect::<Vec<&Data>>();

    }
}

fn is_even(i: &usize) -> bool {
    i % 2 == 0
}
