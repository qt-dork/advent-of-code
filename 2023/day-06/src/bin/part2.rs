use nom::{
    bytes::complete::tag,
    character::complete as cc,
    character::complete::{digit1, space1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, tuple},
    Finish, IResult,
};

fn main() {
    let input = include_str!("sample.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
struct Boat {
    speed: usize,
    distance: usize,
    let_loose: bool,
}

impl Boat {
    fn new() -> Self {
        Self {
            speed: 0,
            distance: 0,
            let_loose: false,
        }
    }

    fn charge(&mut self) {
        self.speed += 1;
    }

    fn tick(&mut self) {
        if self.let_loose {
            self.distance += self.speed
        } else {
            self.charge()
        }
    }

    fn let_loose(&mut self) {
        self.let_loose = true;
    }
}

#[derive(Debug, Clone, Copy)]
struct World {
    time: usize,
    at: usize,
    boat: Boat,
    max_distance: usize,
}

impl World {
    fn new(time: usize, max_distance: usize) -> Self {
        Self {
            time,
            at: 0,
            boat: Boat::new(),
            max_distance,
        }
    }

    fn tick(&mut self) {
        self.at += 1;
        self.boat.tick();
    }

    fn has_won(&self) -> bool {
        self.boat.distance > self.max_distance
    }

    fn stopped(&self) -> bool {
        self.time <= self.at
    }

    fn reset(&mut self) {
        self.at = 0;
        self.boat = Boat::new();
    }
}

fn parse(i: &str) -> IResult<&str, World> {
    dbg!(i);
    let (i, times) = map(
        delimited(
            tuple((tag("Time:"), space1)),
            separated_list1(space1, digit1),
            tag("\n"),
        ),
        |x| {
            let x = x.join("");
            let x: usize = x.parse().unwrap_or(0);
            x
        },
    )(i)?;
    dbg!(i);

    let (i, distances) = map(
        delimited(
            tuple((tag("Distance:"), space1)),
            separated_list1(space1, digit1),
            tag("\n"),
        ),
        |x| {
            let x = x.join("");
            let x: usize = x.parse().unwrap_or(0);
            x
        },
    )(i)?;
    dbg!(i);

    let world = World::new(times, distances);
    dbg!(world);

    Ok((i, world))
}

fn part1(i: &str) -> String {
    let mut world = all_consuming(parse)(i).finish().unwrap().1;

    // test a world for each variation
    let amt = (0..world.time).find(|ticks| {
        if ticks % 100 == 0 {
            dbg!(ticks, world.time);
        }

        world.boat.speed += ticks;
        world.at += ticks;

        world.boat.let_loose();

        // early return
        // world.tick();
        // if world.has_won() {
        //     return true;
        // }
        let remaining = world.time - world.at;
        if world.boat.speed * remaining < world.max_distance {
            world.reset();
            return false;
        }

        while !world.stopped() {
            world.tick();
        }
        if world.has_won() {
            true
        } else {
            world.reset();
            false
        }
    });
    let amt = amt.unwrap();
    dbg!(amt);

    let spans = amt..(world.time - amt + 1);

    spans.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        let result = part1(input);
        assert_eq!(result, "71503");
    }
    #[test]
    fn even() {
        let input = "Time:      7  15   31
Distance:  9  40  200
";
        let result = part1(input);
        assert_eq!(result, "71504");
    }
}
