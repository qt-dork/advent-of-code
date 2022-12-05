use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("Not a valid move: {c:?}")),
        }
    }
}

struct Round {
    theirs: Move,
    ours: Move
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn try_from(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next());
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}
impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
        }
    }
}

impl Outcome {
    fn points(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Tie => 3,
        }
    }
    fn from_char(c: char) -> Outcome {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Tie,
            'Z' => Outcome::Win,
            _ => panic!("Invalid outcome"),
        }
    }
}

impl RPS {
    fn from_char(c: char) -> RPS {
        match c {
            // Opponent
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissors,

            // Player
            'X' => RPS::Rock,
            'Y' => RPS::Paper,
            'Z' => RPS::Scissors,
            
            // Just assume i'll never make any errors lol
            _ => panic!("Invalid input"),
        }
    }
    fn beats(&self, other: &RPS) -> Outcome {
        match self {
            RPS::Rock => match other {
                RPS::Rock => Outcome::Tie,
                RPS::Paper => Outcome::Lose,
                RPS::Scissors => Outcome::Win,
            },
            RPS::Paper => match other {
                RPS::Rock => Outcome::Win,
                RPS::Paper => Outcome::Tie,
                RPS::Scissors => Outcome::Lose,
            },
            RPS::Scissors => match other {
                RPS::Rock => Outcome::Lose,
                RPS::Paper => Outcome::Win,
                RPS::Scissors => Outcome::Tie,
            },
        }
    }
    fn desired_outcome(&self, outcome: Outcome) -> RPS {
        match self {
            RPS::Rock => match outcome {
                Outcome::Win => RPS::Paper,
                Outcome::Lose => RPS::Scissors,
                Outcome::Tie => RPS::Rock,
            },
            RPS::Paper => match outcome {
                Outcome::Win => RPS::Scissors,
                Outcome::Lose => RPS::Rock,
                Outcome::Tie => RPS::Paper,
            },
            RPS::Scissors => match outcome {
                Outcome::Win => RPS::Rock,
                Outcome::Lose => RPS::Paper,
                Outcome::Tie => RPS::Scissors,
            },
        }
    }
    fn points(&self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

type Game = (RPS, RPS);
type CheatGame = (RPS, Outcome);

fn score_from_games(games: &Vec<Game>) -> usize {
    games.iter().fold(0, |accum, game| {
        accum + game.1.beats(&game.0).points() + game.1.points()
    })
}

fn main() {
    // file stuff (doesn't count)
    let path = "input.txt";
    let input = fs::read_to_string(path).expect("Something went wrong reading the file");
    let input = input.trim_end();

    // part 1 
    // convert from string to games of RPS
    let games: Vec<Game> = input.split("\n").map(|line| {
        let items = line.split(" ").collect::<String>();
        let chars = items.chars();
        // just assume i'll only ever get 2 chars lol
        let rps: Vec<RPS> = chars.map(|c| RPS::from_char(c)).collect();
        (rps[0], rps[1])
    }).collect();

    // part 2
    // convert games to desired outcomes
    let cheated_games: Vec<CheatGame> = input.split("\n").map(|line| {
        let items = line.split(" ").collect::<String>();
        let mut chars = items.chars();
        // just assume i won't get any errors lol
        let rps = RPS::from_char(chars.next().unwrap());
        let outcome = Outcome::from_char(chars.next().unwrap());
        (rps, outcome)
    }).collect();

    // convert cheated games to normal game type
    let cheated_games: Vec<Game> = cheated_games.iter().map(|game| {
        let desired = game.0.desired_outcome(game.1);
        (game.0, desired)
    }).collect();

    // run games for part 1
    let score = score_from_games(&games);

    // run games for part 2
    let cheated_score = score_from_games(&cheated_games);
    
    println!("{:?}", score);
    println!("{:?}", cheated_score);
}

#[cfg(test)]
#[test]
fn part1_works() {
    let input = include_str!("day2_test.txt");
    assert_eq!(process_part1(input), 2);
}

#[cfg(test)]
#[test]
fn part2_works() {
    let input = include_str!("day2_test.txt");
    assert_eq!(process_part2(input), 4);
}