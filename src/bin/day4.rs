use std::str::FromStr;

const DAY: &str = "day4";

const INPUT: &str = include_str!("day4.txt");

#[cfg(test)]
const TEST_INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

// in retrospect, i don't like part 1 and 2 being in the same function.
// i'm going back to a part 1 and part 2 function.
fn answer(input: &str) -> (u32, u32) {
    input
        .split_whitespace()
        .map(|line| {
            let ranges = line
                .split(',')
                .map(|range| {
                    let mut range = range.split('-');
                    let start = u32::from_str(range.next().unwrap()).unwrap();
                    let end = u32::from_str(range.next().unwrap()).unwrap();
                    (start..=end).collect::<Vec<u32>>()
                }).collect::<Vec<Vec<u32>>>();
            // part 1
            // i have given up
            let left = (ranges[0].iter().all(|x| ranges[1].contains(x)) | ranges[1].iter().all(|x| ranges[0].contains(x))) as u32;
            // part 2
            let right = ranges[0].iter().any(|x| ranges[1].contains(x)) as u32;
            (left, right)
        }).fold((0,0), |accum, x| (accum.0 + x.0, accum.1 + x.1))
}

fn main() {
    println!("{:?}", answer(INPUT));
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(answer(TEST_INPUT), (2, 4));
}