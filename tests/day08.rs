const INPUT: &str = include_str!("res/08.txt");

struct Forest {
    trees: Vec<Vec<i8>>,
    width: isize,
    height: isize,
}

type Position = (isize, isize);

impl Forest {
    fn parse(input: &str) -> Self {
        let trees: Vec<_> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i8)
                    .collect::<Vec<i8>>()
            })
            .collect();
        Forest {
            width: trees[0].len() as isize,
            height: trees.len() as isize,
            trees,
        }
    }

    fn eval(&self, tree: Position) -> (bool, u32) {
        let (l_vis, l_score) = self.eval_for_direction(
            tree,
            |(_, row), _| Box::new((0..row).rev()), // march left
            |(col, _), x| (col, x),
        );
        let (r_vis, r_score) = self.eval_for_direction(
            tree,
            |(_, row), forest| Box::new((row + 1)..(forest.width)), // march right
            |(col, _), x| (col, x),
        );
        let (t_vis, t_score) = self.eval_for_direction(
            tree,
            |(col, _), _| Box::new((0..col).rev()), // march up
            |(_, row), x| (x, row),
        );
        let (b_vis, b_score) = self.eval_for_direction(
            tree,
            |(col, _), forest| Box::new((col + 1)..(forest.width)), // march down
            |(_, row), x| (x, row),
        );
        return (
            l_vis || r_vis || t_vis || b_vis,
            l_score * r_score * t_score * b_score,
        );
    }

    fn eval_for_direction(
        &self,
        tree: Position,
        range: fn(Position, &Forest) -> Box<dyn Iterator<Item = isize>>,
        to_get: fn(Position, isize) -> Position,
    ) -> (bool, u32) {
        let current = self.get(tree).unwrap();
        let mut count = 0;
        for x in range(tree, &self) {
            if let Some(val) = self.get(to_get(tree, x)) {
                count += 1;
                if val >= current {
                    return (false, count);
                }
            }
        }
        (true, count)
    }

    fn get(&self, position: Position) -> Option<i8> {
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
    let mut visible: Vec<Vec<(bool, u32)>> = vec![vec![(false, 0); height]; width];
    for col in 0..forest.height {
        for row in 0..forest.width {
            // visible[col][row] = forest.visible((col as isize, row as isize));
            visible[col as usize][row as usize] = forest.eval((col as isize, row as isize));
        }
    }

    // part 1
    let total_visible: u32 = visible
        .iter()
        .map(|v| v.iter().filter(|(x, _)| *x).count() as u32)
        .sum();
    println!("Day 8, part 1: {total_visible}");

    // part 2
    let high_score = *visible
        .iter()
        .flat_map(|v| v.iter())
        .map(|(_, score)| score)
        .max()
        .unwrap();
    println!("Day 8, part 2: {high_score}");
}
