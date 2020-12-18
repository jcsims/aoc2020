use crate::util::lines_from_path;
use std::collections::HashSet;

pub fn part1() -> i64 {
    let instructions = parse_instructions("data/d8.txt");

    if let RunState::Infinite(acc) = run_instructions(&instructions) {
        return acc;
    }

    panic!("unexpected termination in part 1!");
}

pub fn part2() -> i64 {
    let instructions = parse_instructions("data/d8.txt");

    for (i, instruction) in instructions.iter().enumerate() {
        match instruction {
            Instruction::Nop(x) => {
                let mut new = instructions.clone();
                new[i] = Instruction::Jmp(*x);
                match run_instructions(&new) {
                    RunState::Infinite(_) => continue,
                    RunState::Terminated(x) => return x,
                }
            }
            Instruction::Jmp(x) => {
                let mut new = instructions.clone();
                new[i] = Instruction::Nop(*x);
                match run_instructions(&new) {
                    RunState::Infinite(_) => continue,
                    RunState::Terminated(x) => return x,
                }
            }
            _ => continue,
        }
    }

    panic!("unexpected termination in part 2!");
}

enum RunState {
    Infinite(i64),
    Terminated(i64),
}

fn run_instructions(instructions: &[Instruction]) -> RunState {
    let mut visited = HashSet::new();
    let mut acc: i64 = 0;
    let mut pointer: usize = 0;

    let last_instruction = instructions.len();

    while !visited.contains(&pointer) {
        visited.insert(pointer);
        match instructions[pointer] {
            Instruction::Nop(_) => pointer += 1,
            Instruction::Acc(x) => {
                acc += x;
                pointer += 1
            }
            Instruction::Jmp(x) => pointer = (pointer as i64 + x) as usize,
        }
        if pointer >= last_instruction {
            return RunState::Terminated(acc);
        }
    }
    RunState::Infinite(acc)
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

fn parse_instruction(line: &str) -> Instruction {
    match &line[0..3] {
        "nop" => Instruction::Nop(line[4..].parse::<i64>().unwrap()),
        "acc" => Instruction::Acc(line[4..].parse::<i64>().unwrap()),
        "jmp" => Instruction::Jmp(line[4..].parse::<i64>().unwrap()),
        x => panic!("unknown instruction line: {}", x),
    }
}

fn parse_instructions(filepath: &str) -> Vec<Instruction> {
    lines_from_path(filepath)
        .map(|l| parse_instruction(l.as_ref().unwrap()))
        .collect::<Vec<Instruction>>()
}

#[test]
fn parse_test() {
    assert_eq!(Instruction::Nop, parse_instruction("nop 0"));
    assert_eq!(Instruction::Acc(32), parse_instruction("acc +32"));
    assert_eq!(Instruction::Jmp(-4), parse_instruction("jmp -4"));
}
