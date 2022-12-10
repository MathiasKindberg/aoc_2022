use std::collections::HashSet;
use std::io::BufRead;

use lending_iterator::lending_iterator::constructors::windows_mut;
use lending_iterator::LendingIterator;

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn input() -> Vec<(Direction, usize)> {
    let stdin = std::io::stdin();

    stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let direction = match parts.next().unwrap() {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                invalid => unreachable!("Inavlid input {invalid}"),
            };
            let steps = parts.next().unwrap().parse::<usize>().unwrap();
            (direction, steps)
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Location {
    idx: usize,
    x: isize, // Right positive direction
    y: isize, // Up positive direction
}

fn one(inputs: &Vec<(Direction, usize)>) -> usize {
    let mut head = Location { x: 0, y: 0, idx: 0 };
    let mut tail = Location { x: 0, y: 0, idx: 1 };
    let mut visited: HashSet<Location> = HashSet::new();

    for input in inputs {
        let (direction, steps) = input;
        for _ in 0..*steps {
            visited.insert(tail.clone());

            match direction {
                Direction::Right => head.x += 1,
                Direction::Left => head.x -= 1,
                Direction::Up => head.y += 1,
                Direction::Down => head.y -= 1,
            }
            tail = move_tail(&head, tail);
        }
    }
    visited.insert(tail);

    visited.len()
}

fn move_tail(head: &Location, mut tail: Location) -> Location {
    let diff_x: isize = head.x - tail.x;
    let diff_y: isize = head.y - tail.y;

    let step_x = diff_x / 2;
    let step_y = diff_y / 2;

    tail.x += step_x;
    tail.y += step_y;

    // Handle diagonal catchup.
    if diff_y.abs() == 2 && diff_x.abs() == 1 {
        tail.x += diff_x;
    } else if diff_x.abs() == 2 && diff_y.abs() == 1 {
        tail.y += diff_y;
    }
    tail
}

// Rust is not smart enough to handle an immutable reference to location[0] and a mutable to location[1]
// yet. Would need to take ownership and destructure and then rebuild or something similar to do that
// so we instead take an array.
fn move_tail_2(locations: &mut [Location; 2]) {
    let diff_x: isize = locations[0].x - locations[1].x;
    let diff_y: isize = locations[0].y - locations[1].y;

    let step_x = diff_x / 2;
    let step_y = diff_y / 2;

    locations[1].x += step_x;
    locations[1].y += step_y;

    // Handle diagonal catchup.
    if diff_y.abs() == 2 && diff_x.abs() == 1 {
        locations[1].x += diff_x;
    } else if diff_x.abs() == 2 && diff_y.abs() == 1 {
        locations[1].y += diff_y;
    }
}

fn _print_rope(locs: &Vec<Location>) {
    const MAX: isize = 10;
    println!("============================================");
    for y in (-MAX..MAX).into_iter().rev() {
        print!("{y:3} ");
        for x in -MAX..MAX {
            let mut loc_found = false;
            for loc in locs {
                if loc.x == x && loc.y == y {
                    print!("{}", loc.idx);
                    loc_found = true;
                    break;
                }
            }
            if !loc_found {
                print!(".");
            }
        }
        println!();
    }
}

fn two(inputs: &Vec<(Direction, usize)>) -> usize {
    let mut parts: Vec<Location> = (0..10)
        .into_iter()
        .map(|idx| Location { x: 0, y: 0, idx })
        .collect();
    assert_eq!(parts.len(), 10);
    let mut visited: HashSet<Location> = HashSet::new();

    for input in inputs {
        let (direction, steps) = input;
        for _ in 0..*steps {
            visited.insert(parts.last().unwrap().clone());

            match direction {
                Direction::Right => parts[0].x += 1,
                Direction::Left => parts[0].x -= 1,
                Direction::Up => parts[0].y += 1,
                Direction::Down => parts[0].y -= 1,
            }

            // One day we will get LendingIterator and windows_mut through GATs in std....
            let mut iter = parts.windows_mut::<2>();
            while let Some(window) = iter.next() {
                move_tail_2(window);
            }
        }
    }
    visited.insert(parts.last().unwrap().clone());

    visited.len()
}

fn main() {
    let input = input();

    let now = std::time::Instant::now();
    let one = one(&input);
    let one_spent = std::time::Instant::now() - now;

    let now = std::time::Instant::now();
    let two = two(&input);
    let two_spent = std::time::Instant::now() - now;

    println!("one {one_spent:?} {one:#?}");
    println!("two {two_spent:?} {two:#?}");
}
