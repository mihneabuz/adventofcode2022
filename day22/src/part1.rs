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

fn move_forward(map: &[Vec<u8>], pos: &mut (usize, usize), facing: &Facing, count: usize) {
    match *facing {
        Facing::Up => {
            for _ in 0..count {
                if pos.0 == 0 || pos.1 >= map[pos.0 - 1].len() || map[pos.0 - 1][pos.1] == EMPTY {
                    let mut i = pos.0;
                    while i < map.len() - 1 && pos.1 < map[i + 1].len() && map[i + 1][pos.1] != EMPTY {
                        i += 1;
                    }

                    if map[i][pos.1] == WALL {
                        return;
                    }

                    pos.0 = i;
                } else {
                    if map[pos.0 - 1][pos.1] == WALL {
                        return;
                    }

                    pos.0 -= 1;
                }
            }
        }

        Facing::Down => {
            for _ in 0..count {
                if pos.0 == map.len() - 1 || pos.1 >= map[pos.0 + 1].len() || map[pos.0 + 1][pos.1] == EMPTY {
                    let mut i = pos.0;
                    while i > 0 && pos.1 < map[i - 1].len() && map[i - 1][pos.1] != EMPTY {
                        i -= 1;
                    }

                    if map[i][pos.1] == WALL {
                        return;
                    }

                    pos.0 = i;
                } else {
                    if map[pos.0 + 1][pos.1] == WALL {
                        return;
                    }

                    pos.0 += 1;
                }
            }
        }

        Facing::Left => {
            for _ in 0..count {
                if pos.1 == 0 || map[pos.0][pos.1 - 1] == EMPTY {
                    let mut i = pos.1;
                    while i < map[pos.0].len() - 1 && map[pos.0][i + 1] != EMPTY {
                        i += 1;
                    }

                    if map[pos.0][i] == WALL {
                        return;
                    }

                    pos.1 = i;
                } else {
                    if map[pos.0][pos.1 - 1] == WALL {
                        return;
                    }

                    pos.1 -= 1;
                }
            }
        }

        Facing::Right => {
            for _ in 0..count {
                if pos.1 == map[pos.0].len() - 1 || map[pos.0][pos.1 + 1] == EMPTY {
                    let mut i = pos.1;
                    while i > 0 && map[pos.0][i - 1] != EMPTY {
                        i -= 1;
                    }

                    if map[pos.0][i] == WALL {
                        return;
                    }

                    pos.1 = i;
                } else {
                    if map[pos.0][pos.1 + 1] == WALL {
                        return;
                    }

                    pos.1 += 1;
                }
            }
        }
    }
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let (map, instructions) = content.split_once("\n\n").unwrap();

    let map = map.lines().map(parse_line).collect::<Vec<_>>();
    let mut pos = (
        0,
        map[0].iter().enumerate().find(|e| *e.1 == OPEN).unwrap().0,
    );
    let mut facing = Facing::Right;

    let mut stack = Vec::new();
    for char in instructions.trim().chars() {
        if '0' <= char && char <= '9' {
            stack.push(char);
            continue;
        }

        let forward = String::from_iter(stack.drain(..)).parse::<usize>().unwrap();
        move_forward(&map, &mut pos, &facing, forward);

        match char {
            'R' => facing = rotate_right(facing),
            'L' => facing = rotate_left(facing),
            _ => unreachable!(),
        }
    }

    println!(
        "1: {}",
        1000 * (pos.0 + 1)
            + 4 * (pos.1 + 1)
            + match facing {
                Facing::Right => 0,
                Facing::Down => 1,
                Facing::Left => 2,
                Facing::Up => 3,
            }
    );
}
