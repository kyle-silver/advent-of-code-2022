use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
};

type Position = (i64, i64);
const INPUT: &str = include_str!("res/15.txt");

#[derive(Debug)]
struct Sensor {
    position: Position,
    beacon: Position,
}

impl Sensor {
    fn parse(input: &str) -> Self {
        // 1 2 3 4
        let coords: Vec<i64> = input
            .split_whitespace()
            .map(&str::parse)
            .flat_map(Result::ok)
            .collect();
        Self {
            position: (coords[0], coords[1]),
            beacon: (coords[2], coords[3]),
        }
    }

    fn radius(&self) -> i64 {
        let &Self {
            position: (x1, y1),
            beacon: (x2, y2),
        } = self;
        (x2 - x1).abs() + (y2 - y1).abs()
    }

    fn scan(&self, depth: i64) -> RangeInclusive<i64> {
        let &Self {
            position: (x, y), ..
        } = self;
        let budget = self.radius() - (y - depth).abs();
        RangeInclusive::new(x - budget, x + budget)
    }
}

#[test]
fn part1() {
    let depth = 2_000_000;
    let sensors = INPUT.lines().map(Sensor::parse).collect_vec();
    let range = sensors
        .iter()
        .filter_map(|sensor| {
            let scan = sensor.scan(depth);
            if scan.is_empty() {
                None
            } else {
                Some(scan)
            }
        })
        .reduce(|a, b| {
            let &start = a.start().min(b.start());
            let &end = a.end().max(b.end());
            RangeInclusive::new(start, end)
        })
        .unwrap();
    let beacons: HashSet<_> = sensors.iter().map(|s| s.beacon).collect();
    let distress_beacons = beacons
        .iter()
        .filter(|(x, y)| *y == depth && range.contains(x))
        .collect_vec();
    println!("{distress_beacons:?}");
    let beacons_in_row = distress_beacons.len();
    let ans = range.count() - beacons_in_row;
    println!("Day 15, part 1: {}", ans);
    assert_eq!(5809294, ans);
}

fn combined(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> Option<RangeInclusive<i64>> {
    let (a, b) = if r1.start() < r2.start() {
        (r1, r2)
    } else {
        (r2, r1)
    };
    if *b.start() > a.end() + 1 {
        None
    } else {
        Some(RangeInclusive::new(
            *a.start().min(b.start()),
            *b.end().max(a.end()),
        ))
    }
}

#[test]
fn part2() {
    let sensors = INPUT.lines().map(Sensor::parse).collect_vec();
    let (lower, upper) = (0, 4_000_000);
    'outer: for depth in lower..=upper {
        let mut ranges: VecDeque<_> = sensors
            .iter()
            .filter_map(|sensor| {
                let scan = sensor.scan(depth);
                if scan.is_empty() {
                    None
                } else {
                    Some(scan)
                }
            })
            .map(|range| {
                let (start, end) = (*range.start().max(&lower), *range.end().min(&upper));
                RangeInclusive::new(start, end)
            })
            .collect();
        if depth % 100_000 == 0 {
            println!("At depth {depth}");
        }
        let mut acc = ranges.pop_front().unwrap();
        'inner: loop {
            let mut incompatible = VecDeque::new();
            while let Some(b) = ranges.pop_front() {
                let combined = combined(&acc, &b);
                // println!("{acc:?} + {b:?} = {combined:?}");
                if let Some(c) = combined {
                    acc = c;
                } else {
                    incompatible.push_back(b);
                }
            }
            if incompatible.is_empty() {
                break 'inner;
            }
            if incompatible.len() == 1 {
                if let None = combined(&acc, &incompatible[0]) {
                    let other = &incompatible[0];
                    let gap = acc.start().max(other.start()) - 1;
                    let tuning = (gap * 4_000_000) + depth;
                    println!("Day 15, part 2: {tuning}");
                    break 'outer;
                }
            }
            ranges = incompatible;
            ranges.push_back(acc.clone());
            acc = ranges.pop_front().unwrap();
        }
    }
}
