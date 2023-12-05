use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as cc,
    character::complete::{one_of, space1},
    combinator::{map, value},
    error::ParseError,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

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
	map(tuple((
		cc::u64,
		tag(" "),
		cc::u64,
		tag(" "),
		cc::u64,
	)), |(x, _, y, _, z)| SeedMap {
		start: y as usize,
		end: x as usize,
		width: z as usize
	})
}

pub fn parse_seedmap(i: &str) -> IResult<&str, Vec<SeedMap> {
	// Sample input:
	// seed-to-soil map:
	// 50 98 2
	// 52 50 48
	
	let (i, _) = tuple((
		alpha1,
		tag("-to-"),
		alpha1,
		tag(" map:\n")
	))(i)?;
	let (i, l) = map()(i);
}

pub fn parse_all_seedmaps(i: &str) -> Iresult<&str, Vec<Vec<Seedmap>> {
	separated_list1(tag("\n"), parse_seedmap)(i)
}