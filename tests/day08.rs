const INPUT: &str = include_str!("res/08.txt");

struct Forest {
    trees: Vec<Vec<i8>>,
    width: usize,
    height: usize,
}

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
            width: trees[0].len(),
            height: trees.len(),
            trees,
        }
    }

    fn visible(&self, tree: (isize, isize)) -> (bool, usize) {
        let (l_vis, l_score) = self.visible_left(tree);
        let (r_vis, r_score) = self.visible_right(tree);
        let (t_vis, t_score) = self.visible_top(tree);
        let (b_vis, b_score) = self.visible_bottom(tree);
        println!(
            "{tree:?}: {t_score} * {l_score} * {b_score} * {r_score} ({:?})",
            self.get(tree.0, tree.1)
        );
        return (
            l_vis || r_vis || t_vis || b_vis,
            l_score * r_score * t_score * b_score,
        );
    }

    fn visible_left(&self, tree: (isize, isize)) -> (bool, usize) {
        let (col, row) = tree;
        let current = self.get(col, row).unwrap();
        let mut count = 0;
        for i in (0..row).rev() {
            if let Some(val) = self.get(col, i) {
                count += 1;
                if val >= current {
                    return (false, count);
                }
            }
        }
        (true, count)
    }

    fn visible_right(&self, tree: (isize, isize)) -> (bool, usize) {
        let (col, row) = tree;
        let current = self.get(col, row).unwrap();
        let mut count = 0;
        for i in (row as isize + 1)..(self.width as isize) {
            if let Some(val) = self.get(col, i) {
                count += 1;
                if val >= current {
                    return (false, count);
                }
            }
        }
        (true, count)
    }

    fn visible_top(&self, tree: (isize, isize)) -> (bool, usize) {
        let (col, row) = tree;
        let current = self.get(col, row).unwrap();
        let mut count = 0;
        for j in (0..col).rev() {
            if let Some(val) = self.get(j, row) {
                count += 1;
                if val >= current {
                    return (false, count);
                }
            }
        }
        (true, count)
    }

    fn visible_bottom(&self, tree: (isize, isize)) -> (bool, usize) {
        let (col, row) = tree;
        let current = self.get(col, row).unwrap();
        let mut count = 0;
        for j in (col as isize + 1)..(self.width as isize) {
            if let Some(val) = self.get(j, row) {
                count += 1;
                if val >= current {
                    return (false, count);
                }
            }
        }
        (true, count)
    }

    fn get(&self, col: isize, row: isize) -> Option<i8> {
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
    let mut visible: Vec<Vec<(bool, usize)>> = vec![vec![(false, 0); forest.height]; forest.width];
    for col in 0..forest.height {
        for row in 0..forest.width {
            visible[col][row] = forest.visible((col as isize, row as isize));
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
