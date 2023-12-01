use itertools::Itertools;
use lib::grid::*;

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: usize,
}

fn main() {
    let input = include_str!("day9_test.txt");
    let input = parse_input(input);
    dbg!(input);
}

fn parse_input(i: &str) -> Vec<Move> {
    i.lines().map(|line| {
        let mut chars = line.chars();
        let direction = Direction::try_from(chars.next().unwrap()).unwrap();
        chars.next();
        let amount = chars.next().unwrap().to_digit(10).unwrap() as usize;
        Move {
            direction,
            amount
        }
    }).collect_vec()
}