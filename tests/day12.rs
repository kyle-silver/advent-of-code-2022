use pathfinding::prelude::bfs;
use std::collections::HashMap;

type Position = (i32, i32);
const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const INPUT: &str = include_str!("res/12.txt");

struct Grid {
    start: Position,
    end: Position,
    grid: HashMap<Position, u32>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut grid = HashMap::new();
        for (col, line) in input.lines().enumerate() {
            for (row, c) in line.chars().enumerate() {
                let (col, row) = (col as i32, row as i32);
                let elevation = match c {
                    'S' => {
                        start = (col, row);
                        0
                    }
                    'E' => {
                        end = (col, row);
                        25
                    }
                    _ => c as u32 - 'a' as u32,
                };
                grid.insert((col, row), elevation);
            }
        }
        Self { start, end, grid }
    }

    fn successors(&self, (col, row): &Position) -> Vec<Position> {
        let elevation = match self.grid.get(&(*col, *row)) {
            Some(e) => e,
            None => return Vec::new(),
        };
        NEIGHBORS
            .iter()
            .filter_map(|(delta_col, delta_row)| {
                let position = (col + delta_col, row + delta_row);
                if let Some(val) = self.grid.get(&position) {
                    if *val <= elevation + 1 {
                        return Some(position);
                    }
                }
                return None;
            })
            .collect()
    }
}

#[test]
fn day12() {
    let grid = Grid::parse(INPUT);

    // part 1
    let path = bfs(&grid.start, |p| grid.successors(p), |n| *n == grid.end).unwrap();
    println!("Day 12, part 1: {}", path.len() - 1);

    // part 2
    let starting_candidates: Vec<_> = grid
        .grid
        .iter()
        .filter_map(|(position, elevation)| match elevation {
            0 => Some(position),
            _ => None,
        })
        .collect();
    let ans = starting_candidates
        .iter()
        .filter_map(|start| bfs(*start, |p| grid.successors(p), |n| *n == grid.end))
        .map(|path| path.len())
        .min()
        .unwrap();
    println!("Day 12, part 2: {}", ans - 1);
}
