use std::collections::HashSet;

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

    fn touching(&self, other: &Self) -> bool {
        let delta_col = (self.col - other.col).abs();
        let delta_row = (self.row - other.row).abs();
        delta_col <= 1 && delta_row <= 1
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
        if head.touching(&self.tail) {
            return Rope { head, ..*self };
        }
        return Rope {
            head,
            tail: self.head,
        };
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
}
