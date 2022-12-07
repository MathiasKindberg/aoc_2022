use std::cell::RefCell;
use std::{io::BufRead, rc::Rc};

struct Directory {
    parent: Option<Rc<RefCell<Directory>>>,
    content: std::collections::HashMap<String, Content>,
}

enum Content {
    Directory(Rc<RefCell<Directory>>),
    File(usize),
}

fn input() -> Vec<String> {
    let stdin = std::io::stdin();

    let mut input: Vec<String> = stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .collect();

    input.reverse();

    input
}

fn traverse(
    mut input: Vec<String>,
    node: Rc<RefCell<Directory>>,
    root_node: Rc<RefCell<Directory>>,
) {
    let Some(action) = input.pop() else {return};

    let next_node = if action.starts_with("$ cd") {
        let target = action.split_whitespace().collect::<Vec<&str>>()[2];
        match target {
            "/" => root_node.clone(),
            ".." => node.borrow().parent.as_ref().unwrap().clone(),
            target => match node.borrow().content.get(target) {
                Some(content) => match content {
                    Content::Directory(dir) => dir.clone(),
                    Content::File(_) => unreachable!("Trying to cd into file"),
                },
                None => unreachable!("Trying to cd into something which does not exist"),
            },
        }
    } else if action.starts_with("$ ls") {
        node
    } else if action.starts_with("dir") {
        let name = action.split_whitespace().collect::<Vec<&str>>()[1];
        node.borrow_mut().content.insert(
            name.to_owned(),
            Content::Directory(Rc::new(RefCell::new(Directory {
                parent: Some(node.clone()),
                content: std::collections::HashMap::new(),
            }))),
        );
        node
    } else {
        let mut iter = action.split_whitespace();
        let (size, name) = (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap(),
        );
        node.borrow_mut()
            .content
            .insert(name.to_owned(), Content::File(size));
        node
    };

    traverse(input, next_node, root_node)
}

fn sum_size(node: Rc<RefCell<Directory>>, folder_sizes: &mut Vec<usize>) -> usize {
    let dir_size = node
        .borrow()
        .content
        .values()
        .map(|content| match content {
            Content::Directory(dir) => sum_size(dir.clone(), folder_sizes),
            Content::File(file) => *file,
        })
        .sum();

    folder_sizes.push(dir_size);

    dir_size
}

fn one(input: Vec<String>) -> usize {
    let root_node = Rc::new(RefCell::new(Directory {
        parent: None,
        content: std::collections::HashMap::new(),
    }));

    traverse(input, root_node.clone(), root_node.clone());

    let mut folder_sizes: Vec<usize> = Vec::new();
    let _size = sum_size(root_node, &mut folder_sizes);

    const CUT_OFF: usize = 100_000;

    folder_sizes.iter().filter(|elem| **elem <= CUT_OFF).sum()
}

fn two(input: Vec<String>) -> usize {
    let root_node = Rc::new(RefCell::new(Directory {
        parent: None,
        content: std::collections::HashMap::new(),
    }));

    traverse(input, root_node.clone(), root_node.clone());

    let mut folder_sizes: Vec<usize> = Vec::new();
    let total_used = sum_size(root_node, &mut folder_sizes);

    const TOTAL_SPACE: usize = 70_000_000;
    const REQUIRED: usize = 30_000_000;

    let min_to_free = REQUIRED - (TOTAL_SPACE - total_used);

    folder_sizes.iter().fold(usize::MAX, |mut acc, size| {
        if *size >= min_to_free && *size <= acc {
            acc = *size
        }
        acc
    })
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
    println!("two {two_spent:?} {two:#?}");
}
