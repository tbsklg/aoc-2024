use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

type NumPad = [[char; 3]; 4];
type DirPad = [[char; 3]; 2];
type Pos = (i32, i32);
type Dir = (i32, i32);

const DIRS: [Dir; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn part1(input: &str) -> usize {
    let codes = extract_codes(input);
    let num_pad: NumPad = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        ['\0', '0', 'A'],
    ];
    let dir_pad: DirPad = [['\0', '^', 'A'], ['<', 'v', '>']];

    let paths = shortest_paths(&num_pad, (2, 3), "029A");

    println!("{:?}", paths);
    0
}

fn shortest_paths(num_pad: &[[char; 3]; 4], start: Pos, code: &str) -> Vec<Vec<char>> {
    code.chars()
        .fold((start, vec![vec![]]), |(start, all_paths), c| {
            let (next_pos, paths) = shortest_path(num_pad, start, c);

            let new_paths = all_paths
                .into_iter()
                .flat_map(|route| {
                    paths.iter().map(move |p| {
                        let mut new_route = route.clone();
                        new_route.extend(p.clone());
                        new_route
                    })
                })
                .collect();

            (next_pos, new_paths) // Return updated state: new position and routes
        })
        .1 // Extract
}

fn shortest_path(num_pad: &[[char; 3]; 4], start: Pos, end: char) -> (Pos, Vec<Vec<char>>) {
    let mut queue: VecDeque<(Pos, Vec<(Pos, Dir)>)> = VecDeque::from([(start, vec![])]);
    let mut paths: Vec<Vec<char>> = vec![];
    let mut min_path = usize::MAX;
    let mut end_pos = start;

    while let Some((curr, path)) = queue.pop_front() {
        if get(num_pad, curr) == Some(end) {
            if path.len() > min_path {
                break;
            }

            min_path = path.len();
            end_pos = curr;

            let mut dirs = path.iter().map(|p| print_dir(p.1)).collect::<Vec<_>>();
            dirs.push('A');
            paths.push(dirs);

            continue;
        }

        let neighbors = DIRS
            .iter()
            .filter_map(|dir| {
                let next = (curr.0 + dir.0, curr.1 + dir.1);
                let visited = path.iter().map(|p| p.0).collect::<Vec<_>>();

                if !visited.contains(&next) && get(num_pad, next).is_some_and(|c| c != '\0') {
                    let mut next_path = path.clone();
                    next_path.push((next, *dir));
                    Some((next, next_path))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for neighbor in neighbors {
            queue.push_back(neighbor);
        }
    }

    (end_pos, paths)
}

fn print_dir(dir: Dir) -> char {
    match dir {
        (-1, 0) => '<',
        (1, 0) => '>',
        (0, -1) => '^',
        (0, 1) => 'v',
        _ => unreachable!(),
    }
}

fn get(num_pad: &NumPad, pos: Pos) -> Option<char> {
    num_pad
        .get(pos.1 as usize)
        .and_then(|l| l.get(pos.0 as usize))
        .copied()
}

fn extract_codes(input: &str) -> Vec<&str> {
    input.lines().collect()
}
