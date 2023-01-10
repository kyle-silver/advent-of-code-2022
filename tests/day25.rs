const INPUT: &str = include_str!("res/25.txt");

fn from_snafu(input: &str) -> i64 {
    input
        .chars()
        .rev()
        .map(|c| match c {
            '=' => -2,
            '-' => -1,
            c => c.to_digit(10).unwrap() as i64,
        })
        .enumerate()
        .map(|(i, c)| 5i64.pow(i as u32) * c)
        .sum()
}

fn to_snafu(mut value: i64) -> String {
    let mut digits = Vec::new();
    while value > 0 {
        // remainder mod 5
        let remainder = value % 5;
        let adjusted = match remainder {
            4 => -1,
            3 => -2,
            x => x,
        };
        digits.push(adjusted);
        value -= adjusted;
        value /= 5;
    }
    // println!("{digits:?}");
    digits
        .iter()
        .rev()
        .map(|d| match d {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        })
        .collect()
}

#[test]
fn part1() {
    let value = INPUT.lines().map(from_snafu).sum();
    let ans = to_snafu(value);
    println!("Day 25, part 1: {ans}");
    assert_eq!("2-21=02=1-121-2-11-0", ans);
}
