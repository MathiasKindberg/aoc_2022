use std::io::BufRead;

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn extra_score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl From<&str> for Hand {
    fn from(hand: &str) -> Self {
        match hand {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Invalid rock paper scissors symbol!"),
        }
    }
}

fn input() -> Vec<(String, String)> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut input = line.split_whitespace().take(2);
            (input.next().unwrap().into(), input.next().unwrap().into())
        })
        .collect()
}

fn do_dock_paper_scissors(opponent: &Hand, you: &Hand) -> usize {
    let match_result = match (&you, &opponent) {
        (Hand::Rock, Hand::Rock)
        | (Hand::Paper, Hand::Paper)
        | (Hand::Scissors, Hand::Scissors) => 3,
        (Hand::Rock, Hand::Scissors)
        | (Hand::Scissors, Hand::Paper)
        | (Hand::Paper, Hand::Rock) => 6,
        (Hand::Rock, Hand::Paper)
        | (Hand::Paper, Hand::Scissors)
        | (Hand::Scissors, Hand::Rock) => 0,
    };

    match_result + you.extra_score()
}

fn one(input: &Vec<(String, String)>) {
    let score: usize = input
        .into_iter()
        .map(|(opponent, you)| (Hand::from(opponent.as_str()), Hand::from(you.as_str())))
        .map(|(opponent, you)| do_dock_paper_scissors(&opponent, &you))
        .sum();
    println!("One: {score}");
}

#[derive(Debug)]
enum MatchResult {
    Lose,
    Draw,
    Win,
}

impl From<&str> for MatchResult {
    fn from(hand: &str) -> Self {
        match hand {
            "X" => MatchResult::Lose,
            "Y" => MatchResult::Draw,
            "Z" => MatchResult::Win,
            _ => panic!("Invalid outcome!"),
        }
    }
}

fn choose_your_hand(opponent: &Hand, desired_outcome: MatchResult) -> Hand {
    match (opponent, desired_outcome) {
        (Hand::Rock, MatchResult::Lose) => Hand::Scissors,
        (Hand::Rock, MatchResult::Draw) => Hand::Rock,
        (Hand::Rock, MatchResult::Win) => Hand::Paper,
        (Hand::Paper, MatchResult::Lose) => Hand::Rock,
        (Hand::Paper, MatchResult::Draw) => Hand::Paper,
        (Hand::Paper, MatchResult::Win) => Hand::Scissors,
        (Hand::Scissors, MatchResult::Lose) => Hand::Paper,
        (Hand::Scissors, MatchResult::Draw) => Hand::Scissors,
        (Hand::Scissors, MatchResult::Win) => Hand::Rock,
    }
}

fn two(input: &Vec<(String, String)>) {
    let score: usize = input
        .iter()
        .map(|(opponent, you)| {
            (
                Hand::from(opponent.as_str()),
                MatchResult::from(you.as_str()),
            )
        })
        .map(|(opponent, you)| {
            let your_hand = choose_your_hand(&opponent, you);
            (opponent, your_hand)
        })
        .map(|(opponent, you)| do_dock_paper_scissors(&opponent, &you))
        .sum();
    println!("Two: {score:#?}");
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
