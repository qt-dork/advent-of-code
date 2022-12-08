use std::fmt;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, opt, map_res},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

// This has ended up copying almost everything from
// fasterthanli.me's solution at:
// https://fasterthanli.me/series/advent-of-code-2022/part-5
// I just used it as a reference for improvements in parsing
// but in order to make them work, I had to copy everything
// else. At the very least, it's better code.

fn main() {
    let input = include_str!("day5.txt");
    let result1 = process_part1(input);
    let result2 = process_part2(input);
    println!("{}\n{}", result1, result2);
}

#[derive(Clone, Copy)]
struct Crate(char);

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


struct Piles(Vec<Vec<Crate>>);
impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}
impl Piles {
    fn apply1(&mut self, ins: Instruction) {
        for _ in 0..ins.quantity {
            let el = self.0[ins.from].pop().unwrap();
            self.0[ins.to].push(el);
        }
    }
    fn apply(&mut self, ins: Instruction) {
        for krate in (0..ins.quantity)
            .map(|_| self.0[ins.from].pop().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            self.0[ins.to].push(krate);
        }
    }
}

// a lot of the parsing code has been borrowed from amos of
// fasterthanli.me fame. this is because i don't know how to use
// nom, and i'm using this as a way to learn about it.
fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    // apparently drop doesn't do anything on values
    // that implement copy? that's wild.
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_hole(i)?;
    let mut v = vec![c];
    
    loop {
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    /// Move `[x]` from y to z
    quantity: usize,
    /// Move x from `[y]` to z
    from: usize,
    /// Move x from y to `[z]`
    to: usize,
}


fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

// convert from 1-indexed to 0-indexed
fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(    
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, from, to)| Instruction { quantity, from
        , to },
    )(i)
}

// Stolen from stackoverflow
fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn process_part1(input: &str) -> String {
    let mut lines = input.lines();
    let mut crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();
    let mut piles = Piles(transpose_rev(crate_lines));

    assert!(lines.next().unwrap().is_empty());

    for ins in lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1) {
        println!("{ins:?}");
        piles.apply1(ins);
        println!("{piles:?}");
    }

    let result = piles.0.iter().map(|pile| pile.last().unwrap()).join("");

    result
}

fn process_part2(input: &str) -> String {
    let mut lines = input.lines();
    let mut crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();
    let mut piles = Piles(transpose_rev(crate_lines));

    assert!(lines.next().unwrap().is_empty());

    for ins in lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1) {
        println!("{ins:?}");
        piles.apply(ins);
        println!("{piles:?}");
    }

    let result = piles.0.iter().map(|pile| pile.last().unwrap()).join("");

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