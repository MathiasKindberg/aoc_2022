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

fn priority(char: char) -> u32 {
    if char.is_uppercase() {
        // Turn 'A' which is 65 into 27.
        char as u32 - 38
    } else {
        // Turn 'a' which is 97 into 1.
        char as u32 - 96
    }
}

fn score_backpack(first: &String, second: &String) -> u32 {
    let first: std::collections::HashSet<char> = first.chars().collect();

    let matching_item = second.chars().into_iter().try_for_each(|char| {
        if first.contains(&char) {
            return ControlFlow::Break(char);
        }
        ControlFlow::Continue(())
    });

    match matching_item {
        ControlFlow::Break(matching_item) => priority(matching_item),
        ControlFlow::Continue(_) => unreachable!("We found no match....."),
    }
}

fn one(input: &[String]) {
    let score: u32 = input
        .into_iter()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            assert_eq!(first.len(), second.len());
            (first.to_string(), second.to_string())
        })
        .map(|(first, second)| score_backpack(&first, &second))
        .sum();
    println!("One: {score}");
}

fn score_badge(input: &[String]) -> u32 {
    let badge = input.into_iter().try_fold(
        std::collections::HashMap::<char, usize>::new(),
        |mut acc, backpack| {
            let mut backpack: Vec<char> = backpack.chars().collect();

            backpack.sort();
            backpack.dedup();

            for char in backpack {
                let entry = acc.entry(char).or_insert(0);
                *entry += 1;

                if *entry == 3 {
                    return ControlFlow::Break(char);
                }
            }
            ControlFlow::Continue(acc)
        },
    );

    match badge {
        ControlFlow::Break(badge) => priority(badge),
        ControlFlow::Continue(_) => unreachable!("We found no match....."),
    }
}

fn two(input: &[String]) {
    let score: u32 = input.chunks(3).map(|chunk| score_badge(chunk)).sum();
    println!("Two: {score}");
}

fn main() {
    let input = input();
    // println!("input: {input:#?}");
    one(&input);
    two(&input);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_priority() {
        let test_table = [('a', 1), ('z', 26), ('A', 27), ('Z', 52)];
        for (char, priority) in test_table {
            assert_eq!(crate::priority(char), priority, "Char tested: `{char}`");
        }
    }
}
