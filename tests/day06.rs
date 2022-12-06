use std::collections::HashSet;

const INPUT: &str = include_str!("res/06.txt");

#[test]
fn part1() {
    let stream: Vec<_> = INPUT.chars().collect();
    let ans = stream
        .windows(4)
        .enumerate()
        .find_map(|(i, window)| {
            let unique: HashSet<char> = window.iter().cloned().collect();
            if unique.len() == 4 {
                Some(i + 4)
            } else {
                None
            }
        })
        .unwrap();
    println!("Day 6, part 1: {ans}");
}

#[test]
fn part2() {
    let stream: Vec<_> = INPUT.chars().collect();
    let ans = stream
        .windows(14)
        .enumerate()
        .find_map(|(i, window)| {
            let unique: HashSet<char> = window.iter().cloned().collect();
            if unique.len() == 14 {
                Some(i + 14)
            } else {
                None
            }
        })
        .unwrap();
    println!("Day 6, part 2: {ans}");
}
