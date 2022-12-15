use std::collections::HashMap;

type Position = (i32, i32);
const INPUT: &str = include_str!("res/14.txt");

#[derive(Debug)]
enum Rock {
    Stone,
    Sand,
}

fn draw((a, b): Position, (c, d): Position) -> Box<dyn Iterator<Item = Position>> {
    if a == c {
        let (e, f) = (b.min(d), b.max(d));
        let iter = (e..=f).map(move |x| (a, x));
        Box::new(iter)
    } else {
        let (e, f) = (a.min(c), a.max(c));
        let iter = (e..=f).map(move |x| (x, b));
        Box::new(iter)
    }
}

#[derive(Debug)]
struct Cave {
    cave: HashMap<Position, Rock>,
    source: i32,
    floor: i32,
}

impl Cave {
    fn new(input: &str) -> Self {
        let cave: HashMap<Position, Rock> = input
            .lines()
            .flat_map(|line| {
                let vertices: Vec<Position> = line
                    .split(" -> ")
                    .map(|coords| {
                        let (x, y) = coords.split_once(",").unwrap();
                        (x.parse().unwrap(), y.parse().unwrap())
                    })
                    .collect();
                let stones: Vec<_> = vertices
                    .windows(2)
                    .flat_map(|window| draw(window[0], window[1]))
                    .map(|p| (p, Rock::Stone))
                    .collect();
                stones.into_iter()
            })
            .collect();
        let &floor = cave.keys().map(|(_, y)| y).max().unwrap();
        Self {
            cave,
            source: 500,
            floor,
        }
    }

    fn add_sand(&mut self) -> bool {
        match self.simulate_sand() {
            Some(sand) => {
                self.cave.insert(sand, Rock::Sand);
                true
            }
            None => false,
        }
    }

    fn simulate_sand(&mut self) -> Option<Position> {
        let mut current = (self.source, 0);
        while let Some(new) = self.next_sand_position(current) {
            if current == new {
                return Some(current);
            }
            current = new;
        }
        None
    }

    fn next_sand_position(&self, (x, y): Position) -> Option<Position> {
        // check directly beneath
        if let Some(_) = self.cave.get(&(x, y + 1)) {
            // check down and to the left
            if let None = self.cave.get(&(x - 1, y + 1)) {
                return Some((x - 1, y + 1));
            }
            if let None = self.cave.get(&(x + 1, y + 1)) {
                return Some((x + 1, y + 1));
            }
            return Some((x, y));
        }
        if y + 1 >= self.floor {
            None
        } else {
            Some((x, y + 1))
        }
    }

    fn fill_cave(&mut self) -> Position {
        let next = self.simulate_sand_with_floor();
        self.cave.insert(next, Rock::Sand);
        next
    }

    fn simulate_sand_with_floor(&mut self) -> Position {
        let mut current = (self.source, 0);
        loop {
            let new = self.next_sand_position_with_floor(current);
            if current == new {
                return current;
            }
            current = new;
        }
    }

    fn next_sand_position_with_floor(&self, (x, y): Position) -> Position {
        if y + 1 == self.floor {
            return (x, y);
        }
        // check directly beneath
        if self.cave.get(&(x, y + 1)).is_some() {
            // check down and to the left
            if let None = self.cave.get(&(x - 1, y + 1)) {
                return (x - 1, y + 1);
            }
            if let None = self.cave.get(&(x + 1, y + 1)) {
                return (x + 1, y + 1);
            }
            return (x, y);
        }
        (x, y + 1)
    }
}

#[test]
fn part1() {
    let mut cave = Cave::new(INPUT);
    while cave.add_sand() {}
    let ans = cave
        .cave
        .iter()
        .filter(|(_, rock)| matches!(rock, Rock::Sand))
        .count();
    println!("Day 14, part 1: {ans}");
}

#[test]
fn part2() {
    let mut cave = Cave::new(INPUT);
    cave.floor += 2;
    while cave.fill_cave() != (500, 0) {}
    let ans = cave
        .cave
        .iter()
        .filter(|(_, rock)| matches!(rock, Rock::Sand))
        .count();
    println!("Day 14, part 2: {ans}");
    assert_eq!(24813, ans);
}
