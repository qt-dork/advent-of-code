use nom::{
    bytes::complete::tag,
    character::complete as cc,
    character::complete::space1,
    combinator::all_consuming,
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

fn parse(i: &str) -> IResult<&str, Vec<World>> {
    let (i, times) = delimited(
        tuple((tag("Time:"), space1)),
        separated_list1(space1, cc::u64),
        tag("\n"),
    )(i)?;

    let (i, distances) = delimited(
        tuple((tag("Distance:"), space1)),
        separated_list1(space1, cc::u64),
        tag("\n"),
    )(i)?;

    let worlds = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| World::new(*time as usize, *distance as usize))
        .collect();

    Ok((i, worlds))
}

fn part1(i: &str) -> String {
    let mut worlds = all_consuming(parse)(i).finish().unwrap().1;

    let amts: Vec<_> = worlds
        .iter_mut()
        .map(|mut world| {
            // test a world for each variation
            let amt: Vec<_> = (0..world.time)
                .flat_map(|ticks| {
                    (0..ticks).for_each(|_| {
                        world.tick();
                    });
                    world.boat.let_loose();
                    while !world.stopped() {
                        world.tick();
                    }
                    if world.has_won() {
                        let out = Some(world.clone());
                        world.reset();
                        out
                    } else {
                        world.reset();
                        None
                    }
                })
                .collect();
            dbg!(amt.as_slice(), amt.len());
            amt
        })
        .collect();
    dbg!(amts.as_slice());

    let spans = amts.iter().fold(1usize, |accum, x| accum * x.len());

    spans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part1(input);
        assert_eq!(result, "288");
    }
}
