use std::io::BufRead;

#[derive(Debug, PartialEq, Clone)]
enum Action {
    Addx(isize),
    Noop,
}

#[derive(Debug, PartialEq, Clone)]
struct Instruction {
    cycles: usize,
    action: Action,
}

fn input() -> Vec<Instruction> {
    let stdin = std::io::stdin();

    let mut input: Vec<Instruction> = stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut parts = line.split_whitespace();
            match parts.next().unwrap() {
                "noop" => Instruction {
                    cycles: 1,
                    action: Action::Noop,
                },
                "addx" => Instruction {
                    cycles: 2,
                    action: Action::Addx(parts.next().unwrap().parse::<isize>().unwrap()),
                },
                invalid => unreachable!("Inavlid input {invalid}"),
            }
        })
        .collect();

    input.reverse();
    input
}

fn one(mut input: Vec<Instruction>) -> isize {
    let mut signal_strength = 0;
    let mut cycle = 0;

    let mut x = 1;
    let mut curr_instruction = input.pop().unwrap();
    loop {
        if curr_instruction.cycles == 0 {
            match curr_instruction.action {
                Action::Addx(val) => x += val,
                Action::Noop => (),
            }

            curr_instruction = match input.pop() {
                Some(curr) => curr,
                None => break,
            };
        }

        curr_instruction.cycles -= 1;
        cycle += 1;

        if cycle == 20 || cycle % 40 == 20 {
            signal_strength += cycle * x;
        }
    }

    signal_strength
}

fn two(mut input: Vec<Instruction>) -> String {
    let mut cycle = 0;

    let mut x = 1;
    let mut curr_instruction = input.pop().unwrap();

    let mut image = String::new();
    loop {
        if curr_instruction.cycles == 0 {
            match curr_instruction.action {
                Action::Addx(val) => x += val,
                Action::Noop => (),
            }

            curr_instruction = match input.pop() {
                Some(curr) => curr,
                None => break,
            };
        }

        if ((cycle % 40) - x).abs() <= 1 {
            image.push_str("#")
        } else {
            image.push_str(".")
        }

        curr_instruction.cycles -= 1;
        cycle += 1;

        if cycle % 40 == 0 {
            image.push_str("\n")
        }
    }
    image
}

fn main() {
    let input = input();

    let now = std::time::Instant::now();
    let one = one(input.clone());
    let one_spent = std::time::Instant::now() - now;

    let now = std::time::Instant::now();
    let two = two(input);
    let two_spent = std::time::Instant::now() - now;

    println!("one {one_spent:?} {one:#?}");
    println!("two {two_spent:?} \n{two}");
}
