const INPUT: &str = include_str!("res/02.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn parse(token: &str) -> Self {
        use Shape::*;
        let char = token.chars().next().unwrap().to_ascii_lowercase();
        match char {
            'a' | 'x' => Rock,
            'b' | 'y' => Paper,
            'c' | 'z' => Scissors,
            _ => panic!("bad input value!"),
        }
    }

    fn score(&self) -> u32 {
        use Shape::*;
        match &self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn beats(&self) -> Self {
        use Shape::*;
        match &self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

struct Round(Shape, Shape);

impl Round {
    fn parse_part1(input: &str) -> Self {
        let (opponent, player) = input.split_once(" ").unwrap();
        Round(Shape::parse(opponent), Shape::parse(player))
    }

    fn parse_part2(input: &str) -> Self {
        let (opponent, player) = input.split_once(" ").unwrap();
        let opponent = Shape::parse(opponent);
        let player = match player.chars().next().unwrap().to_ascii_lowercase() {
            'x' => opponent.beats(),
            'y' => opponent,
            'z' => opponent.beats().beats(),
            _ => panic!(),
        };
        return Round(opponent, player);
    }

    fn score(&self) -> u32 {
        let outcome = if &self.0.beats() == &self.1 {
            0
        } else if &self.0 == &self.1 {
            3
        } else {
            6
        };
        let shape_bonus = &self.1.score();
        return outcome + shape_bonus;
    }
}

#[test]
fn part1() {
    let ans: u32 = INPUT
        .lines()
        .map(Round::parse_part1)
        .map(|r| r.score())
        .sum();
    println!("Day 2, part 1: {ans}");
}

#[test]
fn part2() {
    let ans: u32 = INPUT
        .lines()
        .map(Round::parse_part2)
        .map(|r| r.score())
        .sum();
    println!("Day 2, part 1: {ans}");
}
