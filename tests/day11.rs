#![feature(binary_heap_drain_sorted)]
use std::collections::{BinaryHeap, VecDeque};

const INPUT: &str = include_str!("res/11.txt");

#[derive(Debug)]
struct Test {
    divisible_by: u32,
    on_true: usize,
    on_false: usize,
}

impl Test {
    fn parse<'a>(input: &mut impl Iterator<Item = &'a str>) -> Self {
        let divisible_by = Self::parse_at(input.next().unwrap(), 3);
        let on_true = Self::parse_at(input.next().unwrap(), 5) as usize;
        let on_false = Self::parse_at(input.next().unwrap(), 5) as usize;
        Test {
            divisible_by,
            on_true,
            on_false,
        }
    }

    fn parse_at(line: &str, index: usize) -> u32 {
        let mut tokens = line.split_whitespace();
        tokens.nth(index).and_then(|x| x.parse().ok()).unwrap()
    }

    fn eval(&self, val: u32) -> usize {
        if val % &self.divisible_by == 0 {
            self.on_true
        } else {
            self.on_false
        }
    }
}

#[derive(Debug)]
enum Action {
    Add,
    Mul,
}

impl Action {
    fn parse(input: &str) -> Self {
        match input {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Arg {
    Old,
    Const(u32),
}

impl Arg {
    fn parse(val: &str) -> Self {
        match val {
            "old" => Arg::Old,
            _ => Arg::Const(val.parse().unwrap()),
        }
    }

    fn val(&self, old: u32) -> u32 {
        match self {
            Arg::Old => old,
            Arg::Const(x) => *x,
        }
    }
}

#[derive(Debug)]

struct Op {
    a: Arg,
    b: Arg,
    action: Action,
}

impl Op {
    fn parse(line: &str) -> Self {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let a = Arg::parse(tokens[3]);
        let b = Arg::parse(tokens[5]);
        let action = Action::parse(tokens[4]);
        Self { a, b, action }
    }

    fn eval(&self, old: u32) -> u32 {
        let (a, b) = (self.a.val(old), self.b.val(old));
        match self.action {
            Action::Add => a + b,
            Action::Mul => a * b,
        }
    }
}

#[derive(Debug)]

struct Monkey {
    items: VecDeque<u32>,
    op: Op,
    test: Test,
    total_inspected: u32,
}

impl Monkey {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let starting_items = lines.nth(1).unwrap();
        let (_, list) = starting_items.split_once(": ").unwrap();
        let items = list.split(", ").filter_map(|x| x.parse().ok()).collect();
        let op = Op::parse(lines.next().unwrap());
        let test = Test::parse(&mut lines);
        Self {
            items,
            op,
            test,
            total_inspected: 0,
        }
    }

    fn eval_next(&mut self, worry_reduction: bool) -> Option<(u32, usize)> {
        if let Some(item) = self.items.pop_front() {
            self.total_inspected += 1;
            let mut scored = self.op.eval(item);
            if worry_reduction {
                scored /= 3;
            }
            let recipient = self.test.eval(scored);
            Some((scored, recipient))
        } else {
            None
        }
    }

    fn receive(&mut self, item: u32) {
        self.items.push_back(item);
    }
}

fn round(monkeys: &mut [Monkey], worry_reduction: bool) {
    for i in 0..monkeys.len() {
        while let Some((item, recipient)) = monkeys[i].eval_next(worry_reduction) {
            monkeys[recipient].receive(item);
        }
    }
}

#[test]
fn part1() {
    let mut monkeys: Vec<_> = INPUT.split("\n\n").map(Monkey::parse).collect();
    for _ in 0..20 {
        round(&mut monkeys, true);
    }
    let mut activity: BinaryHeap<_> = monkeys.iter().map(|m| m.total_inspected).collect();
    let ans: u32 = activity.drain_sorted().take(2).product();
    println!("Day 11, part 1: {ans}");
    assert_eq!(58056, ans);
}

#[test]
fn part2() {
    let mut monkeys: Vec<_> = INPUT.split("\n\n").map(Monkey::parse).collect();
    for _ in 0..10_000 {
        round(&mut monkeys, false);
    }
    let mut activity: BinaryHeap<_> = monkeys.iter().map(|m| m.total_inspected).collect();
    let ans: u32 = activity.drain_sorted().take(2).product();
    println!("Day 11, part 2: {ans}");
    // assert_eq!(58056, ans);
}
