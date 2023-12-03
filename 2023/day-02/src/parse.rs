use nom::{
    bytes::complete::tag,
    character::{complete::alpha1, streaming::digit1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

use crate::types;

impl types::Color {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        map(alpha1, |color: &str| match color {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,

            // Shouldn't be reachable
            &_ => Self::Red,
        })(i)
    }
}

impl types::Cubes {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        map(
            separated_pair(digit1, tag(" "), types::Color::parse),
            |(digit, color)| {
                let amt = digit.parse().unwrap();
                Self { amt, color }
            },
        )(i)
    }
}

impl types::Bag {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        map(separated_list1(tag(", "), types::Cubes::parse), |cubes| {
            let mut bag: types::Bag = Default::default();
            cubes.iter().for_each(|cube| bag.insert(cube));
            bag
        })(i)
    }
}

impl types::Game {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        // example input:
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        //
        // take "Game ", discard, store num
        // separated list, separated by ";" (inside the list of pulls)
        // separated list, separated by "," (inside each pull) (transform into tuple)
        // tuple, " ", num, " ", color
        map(
            tuple((
                preceded(tag("Game "), digit1),
                preceded(tag(": "), separated_list1(tag("; "), types::Bag::parse)),
            )),
            |(pos, pulls)| {
                let pos = pos.parse().unwrap();
                Self { pos, pulls }
            },
        )(i)
    }
}
