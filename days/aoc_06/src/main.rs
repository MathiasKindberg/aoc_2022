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

fn find_first_unqiue_window(input: &str, size: usize) -> usize {
    let char_indices = input.char_indices().collect::<Vec<(usize, char)>>();

    let result = char_indices
        .windows(size)
        .map(|window| {
            window.iter().fold(
                (Vec::with_capacity(size), Vec::with_capacity(size)),
                |(mut index_acc, mut char_acc), (index, char)| {
                    index_acc.push(index);
                    char_acc.push(char);
                    (index_acc, char_acc)
                },
            )
        })
        .try_for_each(|(indices, mut chars)| {
            chars.sort();
            chars.dedup();

            if indices.len() == size && chars.len() == size {
                // AoC wants 1 based indexing
                return ControlFlow::Break(*indices.last().unwrap() + 1);
            }
            std::ops::ControlFlow::Continue(())
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
        .map(|input| find_first_unqiue_window(input, 4))
        .collect();

    let now = std::time::Instant::now();
    let two: Vec<usize> = input
        .iter()
        .map(|input| find_first_unqiue_window(input, 14))
        .collect();
    let spent = std::time::Instant::now() - now;

    println!("one: \n{one:#?}");
    println!("two {spent:?}: \n{two:#?}");
}
