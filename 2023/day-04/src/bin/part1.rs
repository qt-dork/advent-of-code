use itertools::Itertools;

fn main() {
    let input = include_str!("sample.txt");
    let output = part1(input);
    dbg!(output);
}

struct Card {
	winners: Vec<usize>, 
	picks: Vec<usize>)
}

impl Card {
	/// Scores a card by:
	/// 
	/// 1. Finding winners
	/// 2. Doubling 1 by the number of points (2 ^ (points - 1))
	fn score(&self) -> usize {
		let count = c.picks.iter().filter(|item| c.winners.iter().any(|w| w == item)).sum();
		2usize.pow(count - 1)
	}
	
	// Expected input:
	// -> 41 48 83 86 17 | 83 86  6 31 17  9 48 53 <-
	fn parse(i: &str) -> Option<Self> {
		let mut iterable = i.split('|').map(|side| {
			side.split_whitespace().filter_map(|n| n.parse::<usize>().is_ok())
		});
		let winners: Vec<_> = iterable.next()?.collect();
		let picks: Vec<_> = iterable.next()?.collect();
		Some(Self { winners, picks })
	}
}

// Expected input:
// ~~Card ~~1|:|--> 41 48 83 86 17 | 83 86  6 31 17  9 48 53 <-
//  ^trim     ^slice!    ^crammed into Card::parse()
fn parse(i: &str) -> Option<(usize, Card)> {
	let mut trimmed = i.strip_prefix("Card ").split(':');
	let pos: usize = trimmed.next()?.parse()?; // god will not forgive me for this
	let card = Card::parse(trimmed.next()?)?;
	Some((pos, card))
}

fn part1(i: &str) -> String {
	let score = i.lines().filter_map(parse).map(|(_, card) card.score()).sum();
    score.to_string()
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
        assert_eq!(result, "13");
    }
}
