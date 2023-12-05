use std::collections::VecDeque;

use itertools::Itertools;

fn main() {
    let input = include_str!("sample.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Card {
    winners: Vec<usize>,
    picks: Vec<usize>,
}

impl Card {
    /// Scores a card by:
    ///
    /// 1. Finding winners
    fn points(&self) -> color_eyre::Result<usize> {
        let count: usize = self
            .winners
            .iter()
            .filter(|item| self.picks.iter().any(|w| &w == item))
            .count();
        Ok(count)
    }

    /// Scores a card by:
    ///
    /// 1. Finding winners
    /// 2. Doubling 1 by the number of points (2 ^ (points - 1))
    // fn score(&self) -> color_eyre::Result<usize> {
    //     let count: usize = self
    //         .winners
    //         .iter()
    //         .filter(|item| self.picks.iter().any(|w| &w == item))
    //         .count();
    //     let count: u32 = count.try_into()?; // lol panics if the number's too big
    //     Ok(2usize.pow(count) / 2)
    // }

    // Expected input:
    // -> 41 48 83 86 17 | 83 86  6 31 17  9 48 53 <-
    fn parse(i: &str) -> color_eyre::Result<Self, String> {
        let mut iterable = i.split('|').map(|side| {
            side.split_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
        });
        let winners: Vec<_> = iterable.next().ok_or("No winners found")?.collect();
        let picks: Vec<_> = iterable.next().ok_or("No picks found")?.collect();
        Ok(Self { winners, picks })
    }
}

// Expected input:
// ~~Card ~~1|:|--> 41 48 83 86 17 | 83 86  6 31 17  9 48 53 <-
//  ^trim     ^slice!    ^crammed into Card::parse()
fn parse(i: &str) -> color_eyre::Result<(usize, Card), String> {
    let mut trimmed = i
        .strip_prefix("Card ")
        .ok_or("Cannot trim 'Card'")?
        .split(':');
    let pos = trimmed
        .next()
        .ok_or("Cannot grab Card Number")?
        .trim_start();
    let pos = pos.parse().unwrap(); // lol
    let card = Card::parse(trimmed.next().ok_or("No Card contents")?)?;
    Ok((pos, card))
}

fn part1(i: &str) -> String {
    let score: Vec<_> = i
        .lines()
        .map(parse)
        .map_ok(|(_, card)| card.points())
        .flatten()
        .collect();

    let unpack_score: Result<Vec<_>, ()> = {
        match score.iter().any(|item| item.is_err()) {
            true => Err(()),
            false => Ok(score
                .iter()
                .map(|item| item.as_ref().unwrap().to_owned())
                .collect()),
        }
    };
    let unpack_score = unpack_score.unwrap(); // i give up
    let new_score: usize = unpack_score
        .iter()
        .scan(VecDeque::from([1usize]), |state, x| {
            let copies = state.pop_front().unwrap_or(1);

            let push_back = x.checked_sub(state.len()).unwrap_or(0);
            for _ in 0..push_back {
                state.push_back(1);
            }

            (0..*x).for_each(|i| {
                state[i] += copies;
            });

            Some(copies)
        })
        .sum();

    // score.to_string()
    new_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part1(input);
        assert_eq!(result, "30");
    }
}
