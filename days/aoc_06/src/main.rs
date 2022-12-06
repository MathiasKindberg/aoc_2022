use std::io::BufRead;
use std::ops::ControlFlow;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();

    stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .collect()
}

fn find_first_unqiue_window<const WINDOW_SIZE: usize>(input: &str) -> usize {
    let char_indices = input.char_indices().collect::<Vec<(usize, char)>>();

    let result = char_indices.windows(WINDOW_SIZE).try_for_each(|window| {
        let mut chars: arrayvec::ArrayVec<_, WINDOW_SIZE> =
            window.iter().map(|item| item.1).collect();

        chars.sort();

        if window.len() != WINDOW_SIZE {
            return std::ops::ControlFlow::Continue(());
        }

        for char in chars.windows(2) {
            if char[0] == char[1] {
                return std::ops::ControlFlow::Continue(());
            }
        }

        std::ops::ControlFlow::Break(window.last().unwrap().0 + 1)
    });

    match result {
        ControlFlow::Break(res) => res,
        ControlFlow::Continue(_) => unreachable!("We found no match....."),
    }
}

fn main() {
    let input = input();

    let one: Vec<usize> = input
        .iter()
        .map(|input| find_first_unqiue_window::<4>(input))
        .collect();

    let now = std::time::Instant::now();
    let two: Vec<usize> = input
        .iter()
        .map(|input| find_first_unqiue_window::<14>(input))
        .collect();
    let spent = std::time::Instant::now() - now;

    println!("one: \n{one:#?}");
    println!("two {spent:?}: \n{two:#?}");
}
