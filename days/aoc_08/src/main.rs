use std::io::BufRead;

fn input() -> Vec<Vec<isize>> {
    let stdin = std::io::stdin();
    // Pads input with -1 all around.

    let mut input: Vec<Vec<isize>> = stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .map(|row| {
            let mut row = row
                .chars()
                .into_iter()
                .map(|char| char.to_string().parse::<isize>().unwrap())
                .collect::<Vec<isize>>();

            row.insert(0, -1);
            row.push(-1);
            row
        })
        .collect();
    input.insert(0, vec![-1; input[0].len()]);
    input.push(vec![-1; input[0].len()]);

    input
}

#[derive(Debug)]
struct Map {
    row_length: usize,
    col_length: usize,
    input: Vec<isize>,
}

impl Map {
    fn new(input: Vec<Vec<isize>>) -> Self {
        Self {
            row_length: input[0].len(),
            col_length: input.len(),
            input: input.into_iter().flatten().collect(),
        }
    }

    fn col(&self, idx: usize) -> Vec<&isize> {
        assert!(idx < self.row_length);

        self.input
            .iter()
            .skip(idx)
            .step_by(self.row_length)
            .collect()
    }
    fn row(&self, idx: usize) -> Vec<&isize> {
        assert!(idx * self.row_length < self.input.len());

        self.input
            .iter()
            .skip(self.row_length * idx)
            .take(self.row_length)
            .collect()
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        let row_data = self.row(row);
        let col_data = self.col(col);

        let height = row_data[col];
        let height_col = col_data[row];

        assert_eq!(height, height_col);

        // The visibility check is invalid if done using the padding of -1 on the edges.
        assert!(*height >= 0);

        // Ranges are [0..n).
        // Left -> Right
        if row_data[..col].iter().all(|elem| *elem < height) {
            return true;
        }

        // Right -> Left
        if row_data[col + 1..].iter().all(|elem| *elem < height) {
            return true;
        }

        // Top -> Down
        if col_data[..row].iter().all(|elem| *elem < height) {
            return true;
        }

        // Down -> Top
        if col_data[row + 1..].iter().all(|elem| *elem < height) {
            return true;
        }

        false
    }

    fn calculate_scenic_score(&self, row: usize, col: usize) -> usize {
        let row_data = self.row(row);
        let col_data = self.col(col);

        let height = row_data[col];
        let height_col = col_data[row];

        assert!(*height >= 0);
        assert_eq!(height, height_col);

        let right = visible(row_data[col + 1..].iter(), height).unwrap();
        let left = visible(row_data[..col].iter().rev(), height).unwrap();
        let up = visible(col_data[..row].iter().rev(), height).unwrap();
        let down = visible(col_data[row + 1..].iter(), height).unwrap();

        right * left * up * down
    }
}

fn visible<'a, I>(vals: I, height: &isize) -> Option<usize>
where
    I: Iterator<Item = &'a &'a isize>,
{
    let mut visible = 0;
    for tree in vals {
        visible += 1;
        if *tree >= height {
            return Some(visible);
        } else if **tree == -1 {
            return Some(visible - 1);
        }
    }
    None
}

fn one(map: &Map) -> usize {
    let mut visible = 0;
    for row in 1..map.row_length - 1 {
        for col in 1..map.col_length - 1 {
            if map.is_visible(row, col) {
                visible += 1;
            }
        }
    }
    visible
}

fn two(map: &Map) -> usize {
    let mut max_scenic_score = 0;

    for row in 1..map.row_length - 1 {
        for col in 1..map.col_length - 1 {
            max_scenic_score = max_scenic_score.max(map.calculate_scenic_score(row, col));
        }
    }
    max_scenic_score
}

fn main() {
    let input = input();

    let map = Map::new(input);

    let now = std::time::Instant::now();
    let one = one(&map);
    let one_spent = std::time::Instant::now() - now;

    let now = std::time::Instant::now();
    let two = two(&map);
    let two_spent = std::time::Instant::now() - now;

    println!("one {one_spent:?} {one:#?}");
    println!("two {two_spent:?} {two:#?}");
}
