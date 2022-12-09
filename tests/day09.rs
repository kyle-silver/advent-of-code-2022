use std::{collections::HashSet, ops::Add};

const INPUT: &str = include_str!("res/09.txt");

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: u32,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let (direction, steps) = input.split_once(" ").unwrap();
        let steps = steps.parse().unwrap();
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!(),
        };
        Self { direction, steps }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
struct Point {
    col: i32,
    row: i32,
}

impl Point {
    fn move_in(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Point {
                col: self.col + 1,
                ..*self
            },
            Direction::Down => Point {
                col: self.col - 1,
                ..*self
            },
            Direction::Left => Point {
                row: self.row - 1,
                ..*self
            },
            Direction::Right => Point {
                row: self.row + 1,
                ..*self
            },
        }
    }

    // fn touching(&self, other: &Self) -> bool {
    //     // let delta_col = (self.col - other.col).abs();
    //     // let delta_row = (self.row - other.row).abs();
    //     // delta_col <= 1 && delta_row <= 1
    //     let delta = self.delta(other);
    //     delta.0 == 0 || delta.1 == 0
    // }

    fn delta(&self, other: &Self) -> ((i32, i32), bool) {
        let delta_col = self.col - other.col;
        let delta_row = self.row - other.row;
        let touching = delta_col.abs() < 2 && delta_row.abs() < 2;
        let delta = (
            Self::normalize_distance(delta_col),
            Self::normalize_distance(delta_row),
        );
        (delta, touching)
    }

    fn normalize_distance(distance: i32) -> i32 {
        if distance == 0 {
            distance
        } else {
            distance / distance.abs()
        }
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, (col, row): (i32, i32)) -> Self::Output {
        Point {
            col: self.col + col,
            row: self.row + row,
        }
    }
}

#[derive(Debug, Default)]
struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    fn move_head(&self, direction: &Direction) -> Self {
        let head = self.head.move_in(direction);
        let (delta, touching) = self.head.delta(&self.tail);
        if touching {
            return Rope { head, ..*self };
        }
        let movement = (delta.0, delta.1);
        return Rope {
            head,
            tail: self.tail + movement,
        };

        // if head.touching(&self.tail) {
        //     return Rope { head, ..*self };
        // }
        // return Rope {
        //     head,
        //     tail: self.head,
        // };
    }
}

#[derive(Debug)]
struct Grid {
    rope: Rope,
    visited: HashSet<Point>,
}

impl Grid {
    fn new() -> Self {
        let mut grid = Grid {
            rope: Rope::default(),
            visited: HashSet::new(),
        };
        grid.visited.insert(grid.rope.tail);
        grid
    }

    fn update(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.steps {
            self.rope = self.rope.move_head(&instruction.direction);
            // println!("{:?}", self.rope);
            self.visited.insert(self.rope.tail);
        }
    }

    fn visited(&self) -> usize {
        self.visited.len()
    }
}

#[test]
fn part1() {
    let instructions: Vec<_> = INPUT.lines().map(Instruction::parse).collect();
    let mut grid = Grid::new();
    for instruction in &instructions {
        grid.update(instruction);
    }
    println!("Day 9, part 1: {}", grid.visited());
    assert_eq!(6332, grid.visited())
}
