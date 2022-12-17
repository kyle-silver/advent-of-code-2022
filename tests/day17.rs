use std::{collections::HashSet, fmt::Display, vec};

use itertools::Itertools;

type Position = (i64, i64);

const INPUT: &str = include_str!("res/17ex.txt");
const ORDER: [Shape; 5] = [
    Shape::Flat,
    Shape::Plus,
    Shape::Angle,
    Shape::Wall,
    Shape::Square,
];

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    fn parse(c: char) -> Self {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => Direction::Down,
        }
    }

    fn move_in(&self, (x, y): &Position) -> Position {
        match self {
            Direction::Left => (x - 1, *y),
            Direction::Right => (x + 1, *y),
            Direction::Down => (*x, y - 1),
        }
    }
}

#[derive(Debug, Clone)]
enum Shape {
    Flat,
    Plus,
    Angle,
    Wall,
    Square,
}

impl Shape {
    fn points(&self, (x, y): Position) -> Vec<Position> {
        use Shape::*;
        match self {
            Flat => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Plus => vec![
                (x, y + 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x + 1, y + 2),
                (x + 2, y + 1),
            ],
            Angle => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            Wall => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Square => vec![(x, y), (x, y + 1), (x + 1, y), (x + 1, y + 1)],
        }
    }
}

#[derive(Debug, Clone)]
struct Rock {
    shape: Shape,
    position: Position,
}

impl Rock {
    fn points(&self) -> Vec<Position> {
        self.shape.points(self.position)
    }
}

#[derive(Debug)]
struct Cavern {
    occupied: HashSet<Position>,
    width: i64,
}

impl Cavern {
    fn new(width: i64) -> Self {
        Cavern {
            width,
            occupied: HashSet::new(),
        }
    }

    fn get(&self, (x, y): Position) -> bool {
        if x < 0 || x >= self.width {
            return true;
        }
        if y < 0 {
            return true;
        }
        self.occupied.contains(&(x, y))
    }

    fn move_in(&self, rock: &Rock, direction: Direction) -> Position {
        // println!("Moving rock: {rock:?} in direction {direction:?}");
        let new_pos = direction.move_in(&rock.position);
        let can_move = rock
            .shape
            .points(new_pos)
            .iter()
            .all(|position| self.get(*position) == false);
        let next = if can_move {
            new_pos
        } else {
            rock.position.clone()
        };
        // println!("Rock {rock:?} moves in direction {direction:?} to {next:?}");
        next
    }

    fn insert(&mut self, rock: Rock) {
        for point in rock.points() {
            self.occupied.insert(point);
        }
    }

    fn height(&self) -> i64 {
        self.occupied.iter().map(|(_, y)| *y).max().unwrap_or(-1)
    }

    fn purge(&mut self) {
        let height = self.height();
        let to_purge = self
            .occupied
            .iter()
            .filter_map(|(x, y)| {
                if *y < height - 10000 {
                    Some((*x, *y))
                } else {
                    None
                }
            })
            .collect_vec();
        for item in to_purge.iter() {
            self.occupied.remove(item);
        }
    }

    fn simulate(&mut self, shape: Shape, jets: &mut impl Iterator<Item = Direction>) {
        let mut rock = Rock {
            shape,
            position: (2, self.height() + 4),
        };
        for direction in jets {
            // attempt to move laterally
            rock.position = self.move_in(&rock, direction);
            // only stop if we can't move downwards
            let next = self.move_in(&rock, Direction::Down);
            if next == rock.position {
                self.insert(rock.clone());
                return;
            } else {
                rock.position = next;
            }
        }
    }
}

impl Display for Cavern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..=self.height()).rev() {
            for x in 0..self.width {
                let c = if self.get((x, y)) { "#" } else { "." };
                write!(f, "{c}")?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[test]
fn part1() {
    let mut cavern = Cavern::new(7);
    let mut jets = INPUT.chars().map(Direction::parse).cycle();
    for (i, shape) in ORDER.into_iter().cycle().take(2022).enumerate() {
        cavern.simulate(shape, &mut jets);
        if i % 10_000 == 0 && i > 0 {
            cavern.purge()
        }
    }

    // cavern.simulate(Shape::Flat, &mut jets);
    // println!("{cavern}");
    println!("{}", cavern.height() + 1);
}
