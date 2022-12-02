#![feature(binary_heap_drain_sorted)]

use std::collections::BinaryHeap;

const INPUT: &str = include_str!("res/01.txt");

#[test]
fn day1() {
    let mut heap: BinaryHeap<u32> = INPUT
        .split("\n\n")
        .map(|raw| {
            raw.lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    let &part1 = heap.peek().unwrap();
    assert_eq!(68802, part1);
    println!("Day 1, part 1: {part1}");

    let part2: u32 = heap.drain_sorted().take(3).sum();
    assert_eq!(205370, part2);
    println!("Day 2, part 2: {part2}");
}
