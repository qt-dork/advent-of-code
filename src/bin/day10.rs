use std::str::FromStr;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag},
    character::complete::{digit1, char},
    combinator::{map, recognize, opt},
    sequence::{preceded, tuple}, IResult,
};

fn main() {
    let input = include_str!("day10_test.txt");
    let instructions = parse_input(input);
    let r = process_part1(instructions.clone());
    println!("{}", r);
    let r2 = process_part2(instructions);
    println!("{}", r2);
}

fn process_part1(v: Vec<Instruction>) -> i64 {
    let mut cpu = StackMachine::default();
    for i in v.into_iter() {
        cpu.run_instruction(i);
    }
    dbg!(cpu.clone());
    cpu.get_total_signal_strength()
}
fn process_part2(v: Vec<Instruction>) -> String {
    let mut cpu = StackMachine::default();
    for i in v.into_iter() {
        cpu.run_instruction(i);
    }
    dbg!(cpu.clone());
    let screen = cpu.screen;
    let screen = screen.chunks(40).map(|line| {
        let line = line.iter().map(|c|
            match c {
                true => '#',
                false => '.',
            }
        );
        let line = String::from_iter(line);
        format!("{}\n", line)
    });
    String::from_iter(screen)
}

fn parse_line(i: &str) -> IResult<&str, Instruction> {
    alt((
        map(
            tag("noop"),
            |_| Instruction::Noop
        ),
        map(
            preceded(tag("addx "), 
                recognize(tuple((
                    opt(char('-')),
                    digit1
                )))
            ),
            |x| Instruction::Addx(i64::from_str(x).unwrap())
        )
    ))(i)
}

fn parse_input(i: &str) -> Vec<Instruction> {
    i.lines().filter_map(|line| {
        let r = parse_line(line);
        match r {
            Ok((_, Instruction)) => Some(Instruction),
            Err(_) => None,
        }
}).collect_vec()
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i64)
}

#[derive(Debug, Clone)]
struct StackMachine {
    stack: Vec<i64>,
    screen: Vec<bool>,
    cycle: usize,
    signal_strength: Vec<i64>
}

impl StackMachine {
    fn cycle(&mut self) {
        self.cycle += 1;
        self.draw();
        if (self.cycle + 20) % 40 == 0 {
            let push = self.stack[0] * self.cycle as i64;
            self.signal_strength.push(push);
            dbg!(push);
            dbg!(self.cycle);
            dbg!(self.stack.clone());
        }
    }

    fn draw(&mut self) {
        let b = (self.stack[0]..self.stack[0]+3).contains(&((self.cycle % 40) as i64));
        self.screen.push(b);
    }
    
    /// Does nothing for 1 cycle
    fn noop(&mut self) {
        self.cycle();
    }
    
    /// Adds a number onto the current stack.
    /// Takes 2 cycles.
    fn addx(&mut self, other: i64) {
        self.cycle();
        let x = self.stack.pop().unwrap() + other;
        self.stack.push(x);
        self.cycle();
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => self.noop(),
            Instruction::Addx(x) => self.addx(x),
        }
    }

    fn get_total_signal_strength(&self) -> i64 {
        self.signal_strength.iter().sum()
    }
}

impl Default for StackMachine {
    fn default() -> Self {
        let mut cpu = Self { stack: vec![1], screen: vec![], cycle: 1, signal_strength: vec![] };
        cpu.draw();
        cpu
    }
}