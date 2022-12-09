use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

const INPUT: &str = include_str!("res/09.txt");
type Delta = (i32, i32);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> Delta {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
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
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn move_in(&self, direction: &Direction) -> Self {
        *self + direction.delta()
    }

    fn delta(&self, other: &Self) -> (Delta, bool) {
        let (dx, dy) = self - other;
        let touching = dx.abs() < 2 && dy.abs() < 2;
        let normalized = (dx / dx.abs().max(1), dy / dy.abs().max(1));
        (normalized, touching)
    }

    fn follow(&self, leader: &Self) -> Self {
        let (delta, touching) = leader.delta(&self);
        if touching {
            *self
        } else {
            *self + delta
        }
    }
}

impl Add<Delta> for Knot {
    type Output = Knot;

    fn add(self, (col, row): Delta) -> Self::Output {
        Knot {
            x: self.x + col,
            y: self.y + row,
        }
    }
}

impl Sub<&Knot> for &Knot {
    type Output = Delta;

    fn sub(self, rhs: &Knot) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug)]
struct Rope(Vec<Knot>);

impl Rope {
    fn new(n: usize) -> Self {
        Self(vec![Knot::default(); n])
    }

    fn update(&mut self, direction: &Direction) {
        self.0[0] = self.0[0].move_in(direction);
        for i in 1..self.0.len() {
            // apparently having a windows_mut() method is once again something
            // that requires GATs because it depends on a Lending Iterator...
            if let [prev, current] = self.0[(i - 1)..=i].as_mut() {
                *current = current.follow(prev);
            }
        }
    }

    fn tail(&self) -> Knot {
        *self.0.last().unwrap()
    }
}

#[derive(Debug)]
struct Grid {
    rope: Rope,
    visited: HashSet<Knot>,
}

impl Grid {
    fn new(n: usize) -> Self {
        Grid {
            rope: Rope::new(n),
            visited: HashSet::new(),
        }
    }

    fn update(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.steps {
            self.rope.update(&instruction.direction);
            self.visited.insert(self.rope.tail());
        }
    }

    fn visited(&self) -> usize {
        self.visited.len()
    }
}

#[test]
fn part1() {
    let instructions: Vec<_> = INPUT.lines().map(Instruction::parse).collect();
    let mut grid = Grid::new(2);
    for instruction in &instructions {
        grid.update(instruction);
    }
    println!("Day 9, part 1: {}", grid.visited());
    assert_eq!(6332, grid.visited())
}

#[test]
fn part2() {
    let instructions: Vec<_> = INPUT.lines().map(Instruction::parse).collect();
    let mut grid = Grid::new(10);
    for instruction in &instructions {
        grid.update(instruction);
    }
    println!("Day 9, part 2: {}", grid.visited());
    assert_eq!(2511, grid.visited());
}
