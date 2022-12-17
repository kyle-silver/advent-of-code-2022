use std::collections::{HashMap, HashSet};
const INPUT: &str = include_str!("res/16ex.txt");

#[derive(Debug)]
struct Valve {
    flow_rate: i32,
    leads_to: Vec<String>,
}

impl Valve {
    fn new(line: &str) -> (String, Self) {
        let mut tokens = line.split_whitespace().map(|s| s.to_string());
        let name = tokens.next().unwrap();
        let flow_rate = tokens.next().and_then(|s| s.parse().ok()).unwrap();
        let leads_to = tokens.collect();
        (
            name.into(),
            Self {
                flow_rate,
                leads_to,
            },
        )
    }
}

#[derive(Debug)]
struct Volcano {
    volcano: HashMap<String, Valve>,
}

impl Volcano {
    fn new(input: &str) -> Self {
        let volcano = input.lines().map(Valve::new).collect();
        Self { volcano }
    }

    fn score(&self, path: &[String]) -> i32 {
        let mut score = 0;
        let mut open = HashSet::new();
        for valve in path {
            open.insert(valve);
            score += open
                .iter()
                .filter_map(|valve| self.volcano.get(*valve))
                .map(|valve| valve.flow_rate)
                .sum::<i32>()
        }
        score
    }
}

#[test]
fn part1() {
    let volcano = Volcano::new(INPUT);
    let path = vec!["AA".into(), "BB".into(), "CC".into(), "DD".into()];
    println!("score: {:?}", volcano.score(&path))
}
