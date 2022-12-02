use std::io::BufRead;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .collect()
}

fn one(input: &[String]) {
    let max: usize = input
        .iter()
        .fold(vec![0], |mut acc, line| {
            if line.is_empty() {
                acc.push(0);
            } else {
                *acc.last_mut().unwrap() += line.parse::<usize>().unwrap();
            };
            acc
        })
        .into_iter()
        .max()
        .unwrap();
    println!("One: {max}");
}

fn two(input: &[String]) {
    let mut calories: Vec<usize> = input.iter().fold(vec![0], |mut acc, line| {
        if line.is_empty() {
            acc.push(0);
        } else {
            *acc.last_mut().unwrap() += line.parse::<usize>().unwrap();
        };
        acc
    });
    calories.sort();

    let max_three: usize = calories.into_iter().rev().take(3).sum();

    println!("Two: {max_three:?}");
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
