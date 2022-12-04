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

#[derive(Debug)]
struct CleaningRange {
    begin: usize,
    end: usize,
}

impl CleaningRange {
    fn new(range: &str) -> Self {
        let (begin, end) = range.split_once('-').unwrap();
        Self {
            begin: begin.parse::<usize>().unwrap(),
            end: end.parse::<usize>().unwrap(),
        }
    }
}

fn overlap(first: CleaningRange, second: CleaningRange) -> bool {
    (first.begin <= second.begin && first.end >= second.end)
        || (second.begin <= first.begin && second.end >= first.end)
}

fn one(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| line.split_once(',').unwrap())
        .map(|(first, second)| (CleaningRange::new(first), CleaningRange::new(second)))
        .map(|(first, second)| overlap(first, second) as u32)
        .sum()
}

fn any_overlap(first: CleaningRange, second: CleaningRange) -> bool {
    (first.begin <= second.begin && first.end >= second.begin)
        || (second.begin <= first.begin && second.end >= first.begin)
}

fn two(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| line.split_once(',').unwrap())
        .map(|(first, second)| (CleaningRange::new(first), CleaningRange::new(second)))
        .map(|(first, second)| any_overlap(first, second) as u32)
        .sum()
}

fn main() {
    let input = input();
    let one = one(&input);
    let two = two(&input);

    println!("one: {one}");
    println!("two: {two}");
}
