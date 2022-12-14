use std::io::BufRead;

fn input() -> (Vec<String>, Vec<String>) {
    let stdin = std::io::stdin();

    let layout: Vec<String> = stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .map_while(|line| if !line.is_empty() { Some(line) } else { None })
        .collect();

    let moves: Vec<String> = stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .collect();
    (layout, moves)
}

#[derive(Debug)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

// Clone free transpose 💥
// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn process_layout(mut layout: Vec<String>) -> Vec<Vec<String>> {
    let _bays = layout.pop().unwrap();

    let stacks: Vec<Vec<String>> = layout
        .into_iter()
        .map(|row| {
            row.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|chunk| {
                    chunk
                        .iter()
                        .collect::<String>()
                        .trim()
                        .trim_matches('[')
                        .trim_matches(']')
                        .to_owned()
                })
                .collect()
        })
        .collect();

    transpose(stacks)
        .into_iter()
        .map(|bay| {
            bay.into_iter()
                .filter(|tier| !tier.is_empty())
                .rev()
                .collect()
        })
        .collect()
}

fn process_moves(moves: Vec<String>) -> Vec<Move> {
    moves
        .into_iter()
        .map(|row| {
            row.matches(|c: char| c.is_numeric() || c.is_whitespace())
                .collect::<String>()
                .split_whitespace()
                .map(|split| split.parse::<usize>().unwrap())
                .collect()
        })
        .map(|movement: Vec<usize>| Move {
            num: movement[0],
            // Make it zer0 indexed.
            from: movement[1] - 1,
            to: movement[2] - 1,
        })
        .collect()
}

fn one(mut stacks: Vec<Vec<String>>, moves: &[Move]) -> String {
    for Move { num, from, to } in moves {
        for _ in 0..*num {
            let item = stacks[*from].pop().unwrap();
            stacks[*to].push(item);
        }
    }

    stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().clone())
        .collect()
}

fn two(mut stacks: Vec<Vec<String>>, moves: &[Move]) -> String {
    for Move { num, from, to } in moves {
        let length = stacks[*from].len();
        let to_move = stacks[*from].split_off(length - num);
        assert_eq!(to_move.len(), *num, "Must move equal number");
        stacks[*to].extend(to_move.into_iter());
    }

    stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().clone())
        .collect()
}

fn main() {
    let (layout, moves) = input();

    let stacks = process_layout(layout);
    let moves = process_moves(moves);

    let one = one(stacks.clone(), &moves);
    let two = two(stacks, &moves);

    println!("one: {one}");
    println!("two: {two}");
}
