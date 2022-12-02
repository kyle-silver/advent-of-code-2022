#![feature(binary_heap_drain_sorted)]

use std::collections::BinaryHeap;

const INPUT: &str = include_str!("res/01.txt");

#[test]
fn part1() {
    let answer: u32 = INPUT
        .split("\n\n")
        .map(|raw| {
            raw.lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .expect("list should have at least one item!");
    println!("Day 1, part 1: {answer}");
}

#[test]
fn part2() {
    let heap: BinaryHeap<u32> = INPUT
        .split("\n\n")
        .map(|raw| {
            raw.lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    let answer: u32 = heap.iter().take(3).sum();
    println!("Day 1, part 2: {answer}");
}

#[test]
fn day1() {
    let heap: BinaryHeap<u32> = INPUT
        .split("\n\n")
        .map(|raw| {
            raw.lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    let part1 = heap.peek().unwrap();
    println!("Day 1, part 1: {part1}");

    let part2: u32 = heap.iter().take(3).sum();
    println!("Day 2, part 2: {part2}");
}
