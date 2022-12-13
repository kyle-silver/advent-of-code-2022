use std::cmp::Ordering;

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

const INPUT: &str = include_str!("res/13.txt");

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Number(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(line: &str) -> Self {
        let mut tokens: Vec<String> = Vec::new();
        let mut current: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '[' => tokens.push("[".into()),
                ']' => {
                    if current.len() > 0 {
                        let value = current.iter().collect();
                        tokens.push(value);
                        current = Vec::new();
                    }
                    tokens.push("]".into())
                }
                ',' => {
                    if current.len() > 0 {
                        let value = current.iter().collect();
                        tokens.push(value);
                        current = Vec::new();
                    }
                }
                _ => current.push(c),
            }
        }
        return Self::parse_list(&mut tokens.iter().skip(1));
    }

    fn parse_list<'a>(tokens: &mut impl Iterator<Item = &'a String>) -> Self {
        let mut list = Vec::new();
        while let Some(token) = tokens.next() {
            match token.as_str() {
                "[" => {
                    let sub_list = Self::parse_list(tokens);
                    list.push(sub_list);
                }
                "]" => return Self::List(list),
                _ => {
                    let number = token.parse().unwrap();
                    list.push(Self::Number(number));
                }
            }
        }
        panic!("incomplete list")
    }
}

impl Ord for Packet {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self {
            Packet::Number(left) => match rhs {
                Packet::Number(right) => {
                    return left.cmp(right);
                }
                Packet::List(_) => {
                    let lhs = Packet::List(vec![Packet::Number(*left)]);
                    lhs.cmp(rhs)
                }
            },
            Packet::List(left) => match rhs {
                Packet::Number(right) => {
                    let right = Packet::List(vec![Packet::Number(*right)]);
                    self.cmp(&right)
                }
                Packet::List(right) => {
                    for pair in left.iter().zip_longest(right.iter()) {
                        match pair {
                            Left(_) => return Ordering::Greater,
                            Right(_) => return Ordering::Less,
                            Both(left, right) => match left.cmp(&right) {
                                Ordering::Equal => continue,
                                ordering => return ordering,
                            },
                        }
                    }
                    Ordering::Equal
                }
            },
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn part1() {
    let ans: u32 = INPUT
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, pair)| {
            let (left, right) = pair.split_once("\n").unwrap();
            let (left, right) = (Packet::parse(left), Packet::parse(right));
            match left.cmp(&right) {
                Ordering::Less => Some(i as u32 + 1),
                _ => None,
            }
        })
        .sum();
    println!("Day 13, part 1: {ans}");
    assert_eq!(5198, ans);
}

fn find(packets: &[Packet], x: &Packet) -> usize {
    packets
        .iter()
        .enumerate()
        .filter_map(|(i, packet)| match x == packet {
            true => Some(i + 1),
            false => None,
        })
        .next()
        .unwrap()
}

#[test]
fn part2() {
    let mut packets: Vec<_> = INPUT
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(Packet::parse)
        .collect();
    let div1 = Packet::parse("[[2]]");
    let div2 = Packet::parse("[[6]]");
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort_unstable();
    let a = find(&packets, &div1);
    let b = find(&packets, &div2);
    let ans = a * b;
    println!("Day 13, part 2: {ans}");
    assert_eq!(22344, ans);
}
