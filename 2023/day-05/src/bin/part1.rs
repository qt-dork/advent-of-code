use std::{borrow::BorrowMut, ops::Range};

use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as cc,
    character::complete::{alpha1, one_of, space1},
    combinator::{all_consuming, map, value},
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

fn main() {
    let input = include_str!("sample.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
pub struct Seeds(Vec<usize>);

enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct SeedRanges {
    seed: usize,
    range: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct SeedMap {
    from: usize,
    to: usize,
    width: usize,
}

impl SeedMap {
    /// Returns true if the value is in the range of the from seedmap
    fn in_range_of_from(&self, seed: &usize) -> bool {
        let end = self.from + self.width;
        (self.from..end).contains(seed)
    }

    /// Returns true if the value is in the range of the transformed seedmap
    fn in_range_of_to(&self, value: &usize) -> bool {
        (self.to..(self.to + self.width)).contains(value)
    }

    fn max_from(&self) -> usize {
        self.from + self.width - 1
    }

    fn max_to(&self) -> usize {
        self.to + self.width - 1
    }

    fn is_overlapping(&self, range: &Range<usize>) -> bool {
        dbg!(range.clone());
        let range_min = range.clone().min().unwrap();
        let range_max = range.clone().max().unwrap();
        (std::cmp::max(self.from, range_min) <= std::cmp::max(self.max_from(), range_max))
            || (self.from > range_min && self.max_to() < range_max)
    }

    /// Value returned is the direction you transform
    // fn overlap_direction(&self, range: &Range<usize>) -> Direction {
    //     match self.from < range.min().unwrap() {
    //         true => Direction::Left,
    //         false => Direction::Right,
    //     }
    // }

    fn overlap(&self, range: &Range<usize>) -> Vec<Range<usize>> {
        // check which kind of overlapping

        let range_min = range.clone().min().unwrap();
        let range_max = range.clone().max().unwrap();

        // case 1:
        // [4, [[ 5,6 ], 7, 8]]
        //      ^overlap

        if std::cmp::max(self.from, range_min) <= std::cmp::max(self.max_from(), range_max) {
            let amt = std::cmp::min(self.max_from(), range_max)
                .abs_diff(std::cmp::max(self.from, range_min));
            let lower = range_min..(range_min + amt);
            let higher = (range_min + amt)..range_max;
            vec![lower, higher]
        } else {
            // case 2:
            // [ 3, 4, [[ 5, 6 , 7 ]] 8, 9 ]
            //            ^overlap

            let lower = range_min..self.from;
            let middle = self.from..self.max_from();
            let higher = self.max_from()..range_max;
            vec![lower, middle, higher]
        }
    }

    fn transform_range(&self, range: &Range<usize>) -> Range<usize> {
        let offset = self.offset(&range.clone().min().unwrap());
        let to = self.offset_to(&offset);
        to..(to + range.len())
    }

    /// Returns the offset of the seed value from the from value
    ///
    /// Note: will always be positive. Always check if the seed is in range using
    /// `in_range_of_from()` before using offset
    fn offset(&self, seed: &usize) -> usize {
        seed.abs_diff(self.from)
    }

    /// Converts the value from `offset()` into a return value
    fn offset_to(&self, offset: &usize) -> usize {
        self.to + offset
    }
}

// Test case
// seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4
//
// water-to-light map:
// 88 18 7
// 18 25 70
//
// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13
//
// temperature-to-humidity map:
// 0 69 1
// 1 0 69
//
// humidity-to-location map:
// 60 56 37
// 56 93 4

pub fn parse_map(i: &str) -> IResult<&str, SeedMap> {
    map(
        tuple((cc::u64, tag(" "), cc::u64, tag(" "), cc::u64)),
        |(x, _, y, _, z)| SeedMap {
            from: y as usize,
            to: x as usize,
            width: z as usize,
        },
    )(i)
}

pub fn parse_seedmap(i: &str) -> IResult<&str, Vec<SeedMap>> {
    // Sample input:
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48

    let (i, _) = tuple((alpha1, tag("-to-"), alpha1, tag(" map:\n")))(i)?;
    separated_list1(tag("\n"), parse_map)(i)
}

pub fn parse_all_seedmaps(i: &str) -> IResult<&str, Vec<Vec<SeedMap>>> {
    separated_list1(tag("\n\n"), parse_seedmap)(i) // god i hope this works
}

fn parse(i: &str) -> IResult<&str, (Seeds, Vec<Vec<SeedMap>>)> {
    // seeds: 79 14 55 13
    //
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    let (i, seeds) = map(
        delimited(
            tag("seeds: "),
            separated_list1(tag(" "), cc::u64),
            tag("\n\n"),
        ),
        |seed_list| {
            let seed_list = seed_list.iter().map(|x| *x as usize).collect();
            Seeds(seed_list)
        },
    )(i)?;
    let (i, seedmaps) = parse_all_seedmaps(i)?;

    Ok((i, (seeds, seedmaps)))
}

fn part1(i: &str) -> String {
    let i = i.trim_end();
    let (seeds, seedmaps) = all_consuming(parse)(i).finish().unwrap().1;

    dbg!(seeds.clone());
    dbg!(seedmaps.clone());

    let seed_ranges: Vec<_> = seeds
        .0
        .chunks(2)
        .map(|seed_range| {
            let seed = seed_range.first().unwrap_or(&0);
            let range = seed_range.last().unwrap_or(&0);

            SeedRanges {
                seed: *seed,
                range: *range,
            }
        })
        .collect();
    dbg!(seed_ranges.clone());

    let seeds: Vec<_> = seed_ranges
        .iter()
        .map(|seed_range| (seed_range.seed..(seed_range.seed + seed_range.range)))
        .collect();

    // let new_seeds: Vec<_> = seeds
    //     .0
    //     .iter()
    //     .map(|seed| {
    //         let mut seed = seed;
    //         seedmaps.iter().for_each(|map| {
    //             map.iter()
    //                 .for_each(|layer| match layer.in_range_of_from(seed) {
    //                     true => {
    //                         let offset = layer.offset(seed);
    //                         let offset_to = layer.offset_to(&offset);
    //                         seed = &offset_to
    //                     }
    //                     false => {}
    //                 })
    //         });
    //         let seed = seed;
    //         seed
    //     })
    //     .collect();

    let new_seeds: Vec<_> = seedmaps.iter().fold(seeds, |accum, map| {
        let new_seeds: Vec<_> = accum
            .iter()
            .map(|seed| {
                let mut o = vec![seed.clone()];
                for layer in map.iter() {
                    if layer.is_overlapping(seed) {
                        dbg!(seed, layer);

                        let overlap = layer.overlap(seed);
                        dbg!(overlap.clone());
                        let overlap: Vec<_> = overlap
                            .iter()
                            .map(|x| {
                                dbg!(x.clone());
                                if layer.is_overlapping(x) {
                                    layer.transform_range(x).to_owned()
                                } else {
                                    x.to_owned()
                                }
                            })
                            .collect();
                        o = overlap;
                    }
                }
                o
            })
            .flatten()
            .collect();
        new_seeds
    });

    dbg!(new_seeds.clone());

    // let min = new_seeds.iter().r.unwrap_or(&0);

    "min".to_string()
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
