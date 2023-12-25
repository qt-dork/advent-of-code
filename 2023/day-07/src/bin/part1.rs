use std::cmp::Ordering;

use itertools::Itertools;
use miette::{miette, Diagnostic, ErrReport, Result, SourceSpan};
use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{alphanumeric1, digit1},
    },
    combinator::map,
    sequence::tuple,
    IResult,
};
use thiserror::Error;

fn main() {
    let input = include_str!("sample.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Cards {
    Ace = 13,
    King = 12,
    Queen = 11,
    Jack = 10,
    Ten = 9,
    Nine = 8,
    Eight = 7,
    Seven = 6,
    Six = 5,
    Five = 4,
    Four = 3,
    Three = 2,
    Two = 1,
}

impl Cards {
    fn parse(i: char) -> miette::Result<Self, miette::Report> {
        match i {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),

            _ => Err(miette!("Cards fucked up")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand([Cards; 5]);

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut hand = self
            .0
            .iter()
            .dedup_with_count()
            .sorted_by(|(a, _), (b, _)| Ord::cmp(&a, &b));
        match hand.next().unwrap() {
            (5, _) => HandType::FiveOfKind,
            (4, _) => HandType::FourOfKind,
            (3, _) => match hand.next().unwrap() {
                (2, _) => HandType::FullHouse,
                (1, _) => HandType::ThreeOfKind,
            },
            (2, _) => match hand.next().unwrap() {
                (2, _) => HandType::TwoPair,
                (1, _) => HandType::OnePair,
            },
            (1, _) => HandType::HighCard,
        }
    }

    fn compare_hand(&self, other: &Self) -> Ordering {
        let ordering = Ord::cmp(&self.hand_type(), &other.hand_type());
        let orderinging;
        if ordering == Ordering::Equal {
            orderinging = self.0.iter().zip(other.0.iter()).find_map(|(x, y)| {
                let ord = Ord::cmp(x, y);
                if ord.is_eq() {
                    None
                } else {
                    Some(ord)
                }
            })
        }
        if orderinging.is_some() {
            orderinging.unwrap()
        } else {
            ordering
        }
    }
}

fn parse(i: &str) -> IResult<&str, (Hand, usize)> {
    map(
        tuple((alphanumeric1, tag(" "), character::complete::u64)),
        |(hand, _, bid): (&str, &str, u64)| {
            let hand: [Cards; 5] = hand
                .chars()
                .map(|c| Cards::parse(c).unwrap_or_else(|x| panic!("Couldn't parse cards")))
                .collect_vec()
                .try_into()
                .unwrap_or_else(|v: Vec<_>| panic!("Vec was wrong size"));
            (Hand(hand), bid as usize)
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = part1(input);
        assert_eq!(result, "288");
    }
}
