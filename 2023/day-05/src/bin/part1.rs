use itertools::Itertools;



fn main() {
    let input = include_str!("sample.txt");
    let output = part1(input);
    dbg!(output);
}

struct Seeds(Vec<T>);

struct SeedMap<T> {
	from: T,
	to: T,
	width: T,
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
    let score: usize = i
        .lines()
        .map(parse)
        .map_ok(|(_, card)| card.score())
        .flatten()
        .process_results(|iter| iter.sum())
        .unwrap(); // all that work and then i unwrap lol

    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part1(input);
        assert_eq!(result, "13");
    }
}
