use day_02::types::Game;

use nom::{combinator::all_consuming, Finish};

fn main() {
    let input = include_str!("sample.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let games: Vec<_> = input
        .lines()
        .map(|l| {
            let l = l.trim_start_matches(' ');
            all_consuming(Game::parse)(l).finish().unwrap().1
        })
        .collect();

    let filtered_games: usize = games
        .into_iter()
        .filter_map(|game| game.pulls.into_iter().reduce(|acc, pull| acc.maximum(pull)))
        .map(|bag| bag.0.amt * bag.1.amt * bag.2.amt)
        .sum();

    filtered_games.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part2(input);
        assert_eq!(result, "2286");
    }
}
