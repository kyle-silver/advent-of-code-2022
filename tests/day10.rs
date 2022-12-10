const INPUT: &str = include_str!("res/10.txt");

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let tokens: Vec<_> = line.split_whitespace().collect();
        match tokens[0] {
            "noop" => Instruction::Noop,
            "addx" => {
                let x = tokens[1].parse().unwrap();
                Instruction::AddX(x)
            }
            _ => panic!(),
        }
    }

    fn eval(&self, history: &mut Vec<i32>) {
        let last = *history.last().unwrap();
        match self {
            Instruction::Noop => history.push(last),
            Instruction::AddX(x) => {
                history.push(last);
                history.push(last + x);
            }
        }
    }
}

#[test]
fn day10() {
    let instructions: Vec<_> = INPUT.lines().map(Instruction::parse).collect();
    let mut history = vec![1];
    for instruction in instructions {
        instruction.eval(&mut history);
    }
    let checks = [20, 60, 100, 140, 180, 220];
    let ans: i32 = checks.iter().map(|&i| i as i32 * history[i - 1]).sum();
    println!("Day 10, part 1: {ans}");
    assert_eq!(15680, ans);

    let mut display = [' '; 240];
    for (i, pixel) in display.iter_mut().enumerate() {
        let sprite_center = history[i];
        let i = i as i32 % 40;
        if sprite_center - 1 == i || sprite_center == i || sprite_center + 1 == i {
            *pixel = '#';
        }
    }

    // render
    println!("Day 10, part 2:");
    for pixels in display.chunks(40) {
        let scan_line: String = pixels.iter().collect();
        println!("{scan_line}");
    }
}
