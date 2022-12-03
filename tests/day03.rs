use std::collections::HashSet;

const INPUT: &str = include_str!("res/03.txt");

struct Rucksack(Vec<HashSet<char>>);

impl Rucksack {
    fn new(input: &str) -> Self {
        let (first, second) = input.split_at(input.len() / 2);
        let first = first.chars().collect();
        let second = second.chars().collect();
        return Rucksack(vec![first, second]);
    }

    fn from_group(items: &[&str]) -> Self {
        let items = items.iter().map(|line| line.chars().collect()).collect();
        Rucksack(items)
    }

    fn priority(self) -> u32 {
        let common = self.0.into_iter().reduce(|a, b| {
            return a.intersection(&b).cloned().collect();
        });
        let &c = common.unwrap().iter().next().unwrap();
        return Self::priority_for(c);
    }

    fn priority_for(c: char) -> u32 {
        if ('a'..='z').contains(&c) {
            1 + c as u32 - 'a' as u32
        } else {
            27 + c as u32 - 'A' as u32
        }
    }
}

#[test]
fn part1() {
    let rucksacks: Vec<_> = INPUT.lines().map(Rucksack::new).collect();
    let ans: u32 = rucksacks.into_iter().map(Rucksack::priority).sum();
    println!("Day 3, part 1: {ans}")
}

#[test]
fn part2() {
    let lines: Vec<_> = INPUT.lines().collect();
    let rucksacks: Vec<_> = lines.chunks(3).map(Rucksack::from_group).collect();
    let ans: u32 = rucksacks.into_iter().map(Rucksack::priority).sum();
    println!("Day 3, part 2: {ans}")
}
