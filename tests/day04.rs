const INPUT: &str = include_str!("res/04.txt");

type Assignment = (u32, u32);

fn parse_line(line: &str) -> (Assignment, Assignment) {
    let (l, r) = line.split_once(",").unwrap();
    return (parse_assignment(l), parse_assignment(r));
}

fn parse_assignment(pair: &str) -> Assignment {
    let (l, r) = pair.split_once("-").unwrap();
    return (l.parse().unwrap(), r.parse().unwrap());
}

fn fully_contains(a: &Assignment, b: &Assignment) -> bool {
    if a.0 <= b.0 && a.1 >= b.1 {
        return true;
    }
    if b.0 <= a.0 && b.1 >= a.1 {
        return true;
    }
    return false;
}

fn any_overlap(a: &Assignment, b: &Assignment) -> bool {
    overlaps(a, b) || overlaps(b, a)
}

fn overlaps(a: &Assignment, b: &Assignment) -> bool {
    if a.0 <= b.0 && b.0 <= a.1 {
        return true;
    }
    if a.0 <= b.1 && b.1 <= a.1 {
        return true;
    }
    return false;
}

#[test]
fn part1() {
    let ans = INPUT
        .lines()
        .map(parse_line)
        .filter(|(p1, p2)| fully_contains(p1, p2))
        .count();
    println!("Day 4, part 1: {ans}");
}

#[test]
fn part2() {
    let ans = INPUT
        .lines()
        .map(parse_line)
        .filter(|(p1, p2)| any_overlap(p1, p2))
        .count();
    println!("Day 4, part 2: {ans}");
}
