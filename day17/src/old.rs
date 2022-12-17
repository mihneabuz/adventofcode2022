use std::fs;

const LEFT: u8 = '<' as u8;
const RIGHT: u8 = '>' as u8;

#[derive(Clone, Copy, PartialEq)]
enum Rock {
    Horizontal,
    Plus,
    LShape,
    Vertical,
    Square,
}

fn all_pieces(p: (usize, usize), rock: Rock) -> Vec<(usize, usize)> {
    match rock {
        Rock::Horizontal => vec![(p.0 - 1, p.1), (p.0, p.1), (p.0 + 1, p.1), (p.0 + 2, p.1)],
        Rock::Plus => vec![
            (p.0, p.1),
            (p.0 - 1, p.1 + 1),
            (p.0, p.1 + 1),
            (p.0 + 1, p.1 + 1),
            (p.0, p.1 + 2),
        ],
        Rock::LShape => vec![
            (p.0 - 1, p.1),
            (p.0, p.1),
            (p.0 + 1, p.1),
            (p.0 + 1, p.1 + 1),
            (p.0 + 1, p.1 + 2),
        ],
        Rock::Vertical => vec![
            (p.0 - 1, p.1),
            (p.0 - 1, p.1 + 1),
            (p.0 - 1, p.1 + 2),
            (p.0 - 1, p.1 + 3),
        ],
        Rock::Square => vec![
            (p.0, p.1),
            (p.0, p.1 + 1),
            (p.0 - 1, p.1),
            (p.0 - 1, p.1 + 1),
        ],
    }
}

fn bottom_most_pieces(p: (usize, usize), rock: Rock) -> Vec<(usize, usize)> {
    match rock {
        Rock::Horizontal => vec![(p.0 - 1, p.1), (p.0, p.1), (p.0 + 1, p.1), (p.0 + 2, p.1)],
        Rock::Plus => vec![(p.0, p.1), (p.0 - 1, p.1 + 1), (p.0 + 1, p.1 + 1)],
        Rock::LShape => vec![(p.0 - 1, p.1), (p.0, p.1), (p.0 + 1, p.1)],
        Rock::Vertical => vec![(p.0 - 1, p.1)],
        Rock::Square => vec![(p.0, p.1), (p.0 - 1, p.1)],
    }
}

fn left_most_pieces(p: (usize, usize), rock: Rock) -> Vec<(usize, usize)> {
    match rock {
        Rock::Horizontal => vec![(p.0 - 1, p.1)],
        Rock::Plus => vec![(p.0, p.1), (p.0 - 1, p.1 + 1), (p.0, p.1 + 2)],
        Rock::LShape => vec![(p.0 - 1, p.1), (p.0 + 1, p.1 + 1), (p.0 + 1, p.1 + 2)],
        Rock::Vertical => vec![
            (p.0 - 1, p.1),
            (p.0 - 1, p.1 + 1),
            (p.0 - 1, p.1 + 2),
            (p.0 - 1, p.1 + 3),
        ],
        Rock::Square => vec![(p.0 - 1, p.1), (p.0 - 1, p.1 + 1)],
    }
}

fn right_most_pieces(p: (usize, usize), rock: Rock) -> Vec<(usize, usize)> {
    match rock {
        Rock::Horizontal => vec![(p.0 + 2, p.1)],
        Rock::Plus => vec![(p.0, p.1), (p.0 + 1, p.1 + 1), (p.0, p.1 + 2)],
        Rock::LShape => vec![(p.0 + 1, p.1), (p.0 + 1, p.1 + 1), (p.0 + 1, p.1 + 2)],
        Rock::Vertical => vec![
            (p.0 - 1, p.1),
            (p.0 - 1, p.1 + 1),
            (p.0 - 1, p.1 + 2),
            (p.0 - 1, p.1 + 3),
        ],
        Rock::Square => vec![(p.0, p.1), (p.0, p.1 + 1)],
    }
}

fn height(rock: Rock) -> usize {
    match rock {
        Rock::Horizontal => 1,
        Rock::Plus => 3,
        Rock::LShape => 3,
        Rock::Vertical => 4,
        Rock::Square => 2,
    }
}

fn drop(room: &mut Vec<u32>, rock: (usize, Rock), stream: &mut impl Iterator<Item = (usize, u8)>) {
    let mut pos = (3, room.len() - 4);

    'outer: loop {
        match stream.next().unwrap().1 {
            LEFT => {
                if !left_most_pieces(pos, rock.1)
                    .into_iter()
                    .any(|piece| piece.0 == 0 || room[piece.1] & 1 << (piece.0 - 1) != 0)
                {
                    pos.0 -= 1;
                }
            }
            RIGHT => {
                if !right_most_pieces(pos, rock.1)
                    .into_iter()
                    .any(|piece| piece.0 == 6 || room[piece.1] & 1 << (piece.0 + 1) != 0)
                {
                    pos.0 += 1;
                }
            }
            _ => unreachable!(),
        };

        if !bottom_most_pieces(pos, rock.1)
            .into_iter()
            .any(|piece| piece.1 == 0 || room[piece.1 - 1] & 1 << piece.0 != 0)
        {
            pos.1 -= 1;
            continue;
        }

        break 'outer;
    }

    for piece in all_pieces(pos, rock.1).into_iter() {
        room[piece.1] |= 1 << piece.0;
    }

    let mut to_add = 0;
    for (i, &row) in room.iter().rev().take(7).enumerate() {
        if row != 0 {
            to_add = 7 - i;
            break;
        }
    }

    for _ in 0..to_add {
        room.push(0u32);
    }
}

fn debug(room: &[u32]) {
    for line in room.iter().rev() {
        print!("#");
        for b in 0..7 {
            if (line & 1 << b) == 0 {
                print!(".");
            } else {
                print!("@");
            }
        }
        println!("#");
    }
    println!("#########");
}

fn find_cycle(room: &[u32], window: usize) -> (usize, usize) {
    let (mut ci, mut cj) = (0, 0);

    if room.len() < 2 * window {
        return (0, 0);
    }

    'outer: for i in (1..room.len() - 2 * window).rev() {
        for j in i + window..room.len() - window {
            if room[i] == room[j] && (1..window).all(|k| room[i + k] == room[j + k]) {
                ci = i;
                cj = j;
                break 'outer;
            }
        }
    }

    if ci == 0 && cj == 0 {
        return (0, 0);
    }

    for k in 2..cj - ci {
        if room[ci] == room[ci + k] && (0..cj - ci).all(|j| room[ci + j % k] == room[ci + j]) {
            cj = k;
            break;
        }
    }

    (ci, cj)
}

fn main() {
    let content = fs::read_to_string("example").unwrap();
    let mut stream = content
        .bytes()
        .filter(|&b| b == LEFT || b == RIGHT)
        .cycle()
        .enumerate()
        .map(|(i, c)| (i % content.len(), c));

    let mut room = vec![0u32; 7];

    let rocks = vec![
        Rock::Horizontal,
        Rock::Plus,
        Rock::LShape,
        Rock::Vertical,
        Rock::Square,
    ];

    let mut iter = 0;
    for (i, rock) in rocks.iter().cycle().copied().enumerate().map(|(i, r)| (i % rocks.len(), r)) {
        drop(&mut room, (i, rock), &mut stream);

        if iter == 2021 {
            println!("1: {}", room.len() - 7);
            break;
        }

        iter += 1;
    }

    let cycle = find_cycle(&room, 1000);
    println!("{:?}", cycle);
}
