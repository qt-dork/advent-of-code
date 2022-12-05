use std::vec;

use nom::*;
use nom::bytes::complete::tag;
use nom::character::complete::{
    self, alpha1, digit1, multispace1, newline, space1,
};
use nom::branch::alt;
use nom::multi::{separated_list1, many1};
use nom::sequence::{delimited, preceded};

#[derive(Debug, Clone, Copy)]
struct Instruction {
    /// Move `[x]` from y to z
    quantity: u32,
    /// Move x from `[y]` to z
    from: u32,
    /// Move x from y to `[z]`
    to: u32,
}

fn main() {
    let input = include_str!("day5.txt");
    let result1 = process_part1(input);
    let result2 = process_part2(input);
    println!("{}\n{}", result1, result2);
}

fn parse_crate(
    input: &str
) -> IResult<&str, Option<char>> {
    let (input, c) = alt((
        tag("   "),
        delimited(
            complete::char('['), 
            alpha1, 
            complete::char(']')
        )
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value.chars().next().unwrap())
    };

    Ok((input, result))
}

fn parse_line(
    input: &str
) -> IResult<&str, Vec<Option<char>>> {
    let (input, result) = 
        separated_list1(tag(" "), parse_crate)(input)?;

    Ok((input, result))
}

fn parse_instructions(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, quantity) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((input, Instruction {quantity, from, to}))
}

fn parse_crates(
    input: &str
) -> IResult<&str, (Vec<Vec<char>>, Vec<Instruction>)> {
    // parse crates
    let (input, crates_horizontal) =
        separated_list1(newline, parse_line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;

    // parse instructions
    let (input, instructions) = 
        separated_list1(newline, parse_instructions)(input)?;

    // convert crates from
    // [ , D,  ]    
    // [N, C,  ]    
    // [Z, M, P]
    // to
    // [N, Z,  ]
    // [M, C, D]
    // [P,  ,  ]
    let mut crates_vertical: Vec<Vec<Option<char>>> = vec![];
    for _ in 0..crates_horizontal[0].len() {
        crates_vertical.push(vec![])
    }
    for vec in crates_horizontal.iter().rev() {
        for (i, c) in vec.iter().enumerate() {
            crates_vertical[i].push(*c);
        }
    }
    // to
    // [N, Z]
    // [M, C, D]
    // [P]
    let crates: Vec<Vec<char>> = crates_vertical.iter().map(|c|
        c.iter().filter_map(|x| *x).collect()
    ).collect();

    let operations = (crates, instructions);

    Ok((input, operations))
}

fn process_part1(input: &str) -> String {
    let (_, (mut crates, instructions)) = parse_crates(input).unwrap();

    for instruction in instructions.iter() {
        // bad code lol
        for _ in 0..instruction.quantity {
            let pop = crates[(instruction.from - 1) as usize].pop().unwrap();
            crates[(instruction.to - 1) as usize].push(pop);
        }
    }

    let result: String = crates.iter_mut().map(|i| i.pop().unwrap_or(' ').to_string()).collect();

    result
}

fn process_part2(input: &str) -> String {
    let (_, (mut crates, instructions)) = parse_crates(input).unwrap();

    for instruction in instructions.iter() {
        let mut pop = vec![];
        // bad code lol
        for _ in 0..instruction.quantity {
            pop.push(crates[(instruction.from - 1) as usize].pop().unwrap());
        }
        for _ in 0..instruction.quantity {
            crates[(instruction.to - 1) as usize].push(pop.pop().unwrap());
        }
    }

    let result: String = crates.iter_mut().map(|i| i.pop().unwrap_or(' ').to_string()).collect();

    result
}

#[cfg(test)]
#[test]
fn part1_works() {
    let input = include_str!("day5_test.txt");
    assert_eq!(process_part1(input), "CMZ");
}

#[cfg(test)]
#[test]
fn part2_works() {
    let input = include_str!("day5_test.txt");
    assert_eq!(process_part2(input), "MCD");
}