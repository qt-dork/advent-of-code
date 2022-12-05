// iliana also did tests, which i think is a good idea.
// i'll start using tests going forward.

fn main() {
    let input = include_str!("day1.txt");
    let lines = INPUT.lines();
    // i didn't need to do this in main, i should've made an answers function
    // instead so main only handled io stuff
    let greatest: usize = input
        .split("\n\n")
        // Future evie notes:
        // There's a .split_whitespace() function i could've used that would've
        // probably been better?
        .map(|elf| elf.split("\n")
            // Other people used map and .unwrap() here instead
            // but I like .filter_map()
            .filter_map(|line| line.parse().ok())
            // there's a .sum() function that does this. i should've used that
            .reduce(|accum, x| accum + x)
            // using sum would've also made the unwrap unnecessary
            .unwrap()
        // according to iliana's stuff, i could've used a binary heap which
        // would've eliminated this code and all the part 2 code, since they're
        // automatically ordered from largest to smallest
        // another solution is to use .max()
        ).reduce(|accum, x| { 
            if accum >= x { accum } else {x}
        }).unwrap();

    
    
    println!("{:?}", greatest);

    let mut list: Vec<usize> = input.split("\n\n").map(|elf| elf.split("\n").filter_map(|line| line.parse().ok()).fold(0, |accum, x: usize| accum + x)).collect();
    // another solution uses .sort_by(), which makes it avoid the need for
    // using .reverse()
    list.sort(); // I kinda gave up here and made it multiple lines
    list.reverse();
    list.truncate(3); // then uses .take(3) instead of truncate
    println!("{:?}", list);
    // once again could've used .sum() instead of .fold()
    let top_three_result = list.iter().fold(0, |accum, x| accum + x);
    println!("{:?}", top_three_result);
}

fn process_part1(input: &str) -> usize {
    input
        .lines()
}

fn process_part2(input: &str) -> usize {
    let mut list: Vec<usize> = input.split("\n\n").map(|elf| elf.split("\n").filter_map(|line| line.parse().ok()).fold(0, |accum, x: usize| accum + x)).collect();
    list.sort();
    list.reverse();
    list.truncate(3);
    list.iter().fold(0, |accum, x| accum + x)
}


#[cfg(test)]
#[test]
fn part1_works() {
    let input = include_str!("day1_test.txt");
    assert_eq!(process_part1(input), 2);
}

#[cfg(test)]
#[test]
fn part2_works() {
    let input = include_str!("day1_test.txt");
    assert_eq!(process_part2(input), 4);
}