const STACKS: &str = include_str!("res/05/stacks.txt");
const PROCEDURE: &str = include_str!("res/05/proc.txt");

struct Instruction {
    quantity: usize,
    source: usize,
    destination: usize,
}

impl Instruction {
    fn parse(instr: &str) -> Self {
        let tokens: Vec<_> = instr
            .split_whitespace()
            .filter_map(|t| match t.parse::<usize>() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect();
        return Instruction {
            quantity: tokens[0],
            source: tokens[1] - 1,
            destination: tokens[2] - 1,
        };
    }

    fn move_single(&self, stacks: &mut [Vec<char>]) {
        for _ in 0..self.quantity {
            let popped = stacks[self.source].pop().unwrap();
            stacks[self.destination].push(popped);
        }
    }

    fn move_bulk(&self, stacks: &mut [Vec<char>]) {
        let index = stacks[self.source].len() - self.quantity;
        let mut to_move = stacks[self.source].drain(index..).collect();
        stacks[self.destination].append(&mut to_move);
    }
}

#[test]
fn day5() {
    let mut stacks: Vec<_> = STACKS
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let instructions: Vec<_> = PROCEDURE.lines().map(Instruction::parse).collect();

    // part 1
    let mut stack_copy = stacks.clone();
    instructions
        .iter()
        .for_each(|instr| instr.move_single(&mut stack_copy));
    let part1: String = stack_copy
        .iter()
        .map(|stack| *stack.last().unwrap())
        .collect();
    println!("Day 5, part 1: {part1}");

    // part 2
    instructions
        .iter()
        .for_each(|instr| instr.move_bulk(&mut stacks));
    let part2: String = stacks.iter().map(|stack| *stack.last().unwrap()).collect();
    println!("Day 5, part 2: {part2}");
}
