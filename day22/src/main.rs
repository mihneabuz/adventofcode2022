use std::fs;

const EMPTY: u8 = 0;
const OPEN: u8 = 1;
const WALL: u8 = 2;

enum Facing {
    Up,
    Down,
    Left,
    Right,
}

fn parse_line(s: &str) -> Vec<u8> {
    s.chars()
        .map(|c| match c {
            ' ' => EMPTY,
            '.' => OPEN,
            '#' => WALL,
            _ => unreachable!(),
        })
        .collect()
}

fn rotate_right(f: Facing) -> Facing {
    match f {
        Facing::Up => Facing::Right,
        Facing::Down => Facing::Left,
        Facing::Left => Facing::Up,
        Facing::Right => Facing::Down,
    }
}

fn rotate_left(f: Facing) -> Facing {
    match f {
        Facing::Up => Facing::Left,
        Facing::Down => Facing::Right,
        Facing::Left => Facing::Down,
        Facing::Right => Facing::Up,
    }
}

fn debug(map: &[Vec<u8>], current: (usize, usize), facing: Facing) {
    for (i, line) in map.iter().enumerate() {
        for (j, &b) in line.iter().enumerate() {
            let c = if current == (i, j) {
                match facing {
                    Facing::Up => '^',
                    Facing::Down => 'v',
                    Facing::Left => '<',
                    Facing::Right => '>',
                }
            } else {
                match b {
                    EMPTY => ' ',
                    OPEN => '.',
                    WALL => '#',
                    _ => unreachable!(),
                }
            };
            print!("{}", c);
        }
        println!()
    }
    println!()
}

fn main() {
    let content = fs::read_to_string("example").unwrap();
    let (map, instructions) = content.split_once("\n\n").unwrap();

    let map = map.lines().map(parse_line).collect::<Vec<_>>();
    let start = (0, map[0].iter().enumerate().find(|e| *e.1 == OPEN).unwrap().0);
    let facing = Facing::Right;

    debug(&map, start, facing);
    println!("{:?}", start);

    println!("{}", instructions.trim_end());

    let mut stack = Vec::new();
    for char in instructions.trim().chars() {
        if '0' <= char && char <= '9' {
            stack.push(char);
            continue;
        }

        let forward = String::from_iter(stack.drain(..)).parse::<usize>().unwrap();
        println!("{}", forward);

        match char {
            'R' => { facing = rotate_right(facing) }
            'L' => println!("left"),
            _ => unreachable!()
        }
    }
}
