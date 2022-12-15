#![feature(iter_array_chunks)]

use std::io::BufRead;

#[derive(Debug, Clone)]
struct Monkey {
    items: std::collections::VecDeque<u128>,
    operation: Operation,
    operation_target: OperationTarget,
    divisor: u128,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiplicate,
}

#[derive(Debug, Clone)]
enum OperationTarget {
    Old,
    Num(u128),
}

fn input() -> Vec<Monkey> {
    let stdin = std::io::stdin();

    let input: Vec<String> = stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_owned())
        .collect();

    let input: Vec<&[String]> = input.split(|row| row.is_empty()).collect();

    input
        .into_iter()
        .map(|chunk| {
            let mut iter = chunk.into_iter().skip(1);

            let items = iter
                .next()
                .unwrap()
                .trim_start_matches("Starting items:")
                .split(',')
                .map(|item| item.trim().parse::<u128>().unwrap())
                .collect();

            let mut operation_iter = iter
                .next()
                .unwrap()
                .trim_start_matches("Operation: new = old ")
                .split_whitespace();

            let operation = match operation_iter.next().unwrap() {
                "*" => Operation::Multiplicate,
                "+" => Operation::Add,
                _ => unreachable!("invalid operation"),
            };

            let operation_target = match operation_iter.next().unwrap() {
                "old" => OperationTarget::Old,
                num => OperationTarget::Num(num.parse::<u128>().unwrap()),
            };

            let divisor = iter
                .next()
                .unwrap()
                .trim_start_matches("Test: divisible by ")
                .parse::<u128>()
                .unwrap();

            let true_monkey = iter
                .next()
                .unwrap()
                .trim_start_matches("If true: throw to monkey ")
                .parse::<usize>()
                .unwrap();

            let false_monkey = iter
                .next()
                .unwrap()
                .trim_start_matches("If false: throw to monkey ")
                .parse::<usize>()
                .unwrap();

            Monkey {
                items,
                operation,
                operation_target,
                divisor,
                true_monkey,
                false_monkey,
                inspections: 0,
            }
        })
        .collect()
}

fn one(mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            while let Some(item) = monkeys[idx].items.pop_front() {
                let item = match (&monkeys[idx].operation, &monkeys[idx].operation_target) {
                    (Operation::Add, OperationTarget::Old) => item + item,
                    (Operation::Add, OperationTarget::Num(num)) => item + num,
                    (Operation::Multiplicate, OperationTarget::Old) => item * item,
                    (Operation::Multiplicate, OperationTarget::Num(num)) => item * num,
                };
                let item = item / 3;
                if item % monkeys[idx].divisor == 0 {
                    let monkey_idx = monkeys[idx].true_monkey;
                    monkeys[monkey_idx].items.push_back(item);
                } else {
                    let monkey_idx = monkeys[idx].false_monkey;
                    monkeys[monkey_idx].items.push_back(item);
                }

                monkeys[idx].inspections += 1;
            }
        }
    }
    let mut inspections: Vec<usize> = monkeys
        .into_iter()
        .map(|monkey| monkey.inspections)
        .collect();
    inspections.sort();

    inspections.into_iter().rev().take(2).product()
}

fn two(mut monkeys: Vec<Monkey>) -> usize {
    let prime_product: u128 = monkeys
        .iter()
        .map(|monkey| monkey.divisor)
        .collect::<std::collections::HashSet<u128>>()
        .iter()
        .product();

    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            while let Some(item) = monkeys[idx].items.pop_front() {
                let item = match (&monkeys[idx].operation, &monkeys[idx].operation_target) {
                    (Operation::Add, OperationTarget::Old) => item + item,
                    (Operation::Add, OperationTarget::Num(num)) => item + num,
                    (Operation::Multiplicate, OperationTarget::Old) => item * item,
                    (Operation::Multiplicate, OperationTarget::Num(num)) => item * num,
                };

                let item = item % prime_product;

                if item % monkeys[idx].divisor == 0 {
                    let monkey_idx = monkeys[idx].true_monkey;
                    monkeys[monkey_idx].items.push_back(item);
                } else {
                    let monkey_idx = monkeys[idx].false_monkey;
                    monkeys[monkey_idx].items.push_back(item);
                }

                monkeys[idx].inspections += 1;
            }
        }
    }

    let mut inspections: Vec<usize> = monkeys
        .into_iter()
        .map(|monkey| monkey.inspections)
        .collect();
    inspections.sort();

    inspections.into_iter().rev().take(2).product()
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
    println!("two {two_spent:?} {two}");
}
