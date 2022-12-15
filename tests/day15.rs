use itertools::Itertools;
use std::{collections::HashSet, ops::RangeInclusive};

type Position = (i32, i32);
const INPUT: &str = include_str!("res/15.txt");

#[derive(Debug)]
struct Sensor {
    position: Position,
    beacon: Position,
}

impl Sensor {
    fn parse(input: &str) -> Self {
        // 1 2 3 4
        let coords: Vec<i32> = input
            .split_whitespace()
            .map(&str::parse)
            .flat_map(Result::ok)
            .collect();
        Self {
            position: (coords[0], coords[1]),
            beacon: (coords[2], coords[3]),
        }
    }

    fn radius(&self) -> i32 {
        let &Self {
            position: (x1, y1),
            beacon: (x2, y2),
        } = self;
        (x2 - x1).abs() + (y2 - y1).abs()
    }

    fn scan(&self, depth: i32) -> RangeInclusive<i32> {
        let &Self {
            position: (x, y), ..
        } = self;
        let budget = self.radius() - (y - depth).abs();
        println!("Budget: {budget}");
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
    let beacons_in_row = beacons
        .iter()
        .filter(|(x, y)| *y == depth && range.contains(x))
        .count();
    let ans = range.count() - beacons_in_row;
    println!("{}", ans);
    assert_eq!(5809294, ans)
}
