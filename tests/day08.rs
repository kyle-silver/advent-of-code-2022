const INPUT: &str = include_str!("res/08.txt");

struct Forest {
    trees: Vec<Vec<u8>>,
    width: isize,
    height: isize,
}

type Position = (isize, isize);
type Path = Box<dyn Iterator<Item = Position>>;

fn march_left((col, row): Position, _forest: &Forest) -> Path {
    let iter = (0..row).rev().map(move |x| (col, x));
    Box::new(iter)
}

fn march_right((col, row): Position, forest: &Forest) -> Path {
    let iter = ((row + 1)..(forest.width)).map(move |x| (col, x));
    Box::new(iter)
}

fn march_up((col, row): Position, _forest: &Forest) -> Path {
    let iter = (0..col).rev().map(move |x| (x, row));
    Box::new(iter)
}

fn march_down((col, row): Position, forest: &Forest) -> Path {
    let iter = ((col + 1)..(forest.height)).map(move |x| (x, row));
    Box::new(iter)
}

const PATHS: [fn(Position, &Forest) -> Path; 4] = [march_left, march_right, march_up, march_down];

impl Forest {
    fn parse(input: &str) -> Self {
        let trees: Vec<_> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect();
        Forest {
            width: trees[0].len() as isize,
            height: trees.len() as isize,
            trees,
        }
    }

    fn eval(&self, tree: Position) -> (bool, u32) {
        PATHS
            .iter()
            .map(|&path| self.eval_for(tree, path))
            .reduce(|(a_visible, a_score), (b_visible, b_score)| {
                (a_visible || b_visible, a_score * b_score)
            })
            .unwrap()
    }

    fn eval_for(&self, tree: Position, path: fn(Position, &Forest) -> Path) -> (bool, u32) {
        let mut count = 0;
        let current = self.get(tree).unwrap();
        for step in path(tree, &self) {
            if let Some(val) = self.get(step) {
                count += 1;
                if val >= current {
                    return (false, count);
                }
            }
        }
        (true, count)
    }

    fn get(&self, position: Position) -> Option<u8> {
        let (col, row) = position;
        self.trees
            .get(col as usize)
            .and_then(|v| v.get(row as usize))
            .map(|x| *x)
    }
}

#[test]
fn day8() {
    // parse input
    let forest = Forest::parse(INPUT);

    // assess the forest
    let (height, width) = (forest.height as usize, forest.width as usize);
    let mut evaluated: Vec<Vec<(bool, u32)>> = vec![vec![(false, 0); height]; width];
    for col in 0..forest.height {
        for row in 0..forest.width {
            evaluated[col as usize][row as usize] = forest.eval((col, row));
        }
    }

    // part 1
    let total_visible: u32 = evaluated
        .iter()
        .map(|v| v.iter().filter(|(x, _)| *x).count() as u32)
        .sum();
    println!("Day 8, part 1: {total_visible}");
    assert_eq!(1859, total_visible);

    // part 2
    let high_score = *evaluated
        .iter()
        .flat_map(|v| v.iter())
        .map(|(_, score)| score)
        .max()
        .unwrap();
    println!("Day 8, part 2: {high_score}");
    assert_eq!(332640, high_score)
}
