use itertools::Itertools;

fn main() {
    let input = include_str!("day6.txt");
    println!("{}", process_part1(input));
    println!("{}", process_part2(input));
}

fn process_part1(input: &str) -> usize {
    let input = input.chars().collect::<Vec<char>>();
    
    let mut answer = 0;
    for (i, chars) in input.windows(4).enumerate() {
        // let mut chars = chars.clone();
        let is_unique = chars.iter().sorted().dedup().count();

        
        if is_unique == 4 {
            answer = i + 4;
            break;
        }
    }

    answer
}

fn process_part2(input: &str) -> usize {
    let input = input.chars().collect::<Vec<char>>();
    
    let mut answer = 0;
    for (i, chars) in input.windows(14).enumerate() {
        // let mut chars = chars.clone();
        let is_unique = chars.iter().sorted().dedup().count();

        
        if is_unique == 14 {
            answer = i + 14;
            break;
        }
    }

    answer
}

#[cfg(test)]
#[test]
fn part1_works() {
    let input = include_str!("day6_test.txt");
    let mut lines = input.lines();
    
    assert_eq!(process_part1(lines.next().unwrap()), 7);
    assert_eq!(process_part1(lines.next().unwrap()), 5);
    assert_eq!(process_part1(lines.next().unwrap()), 6);
    assert_eq!(process_part1(lines.next().unwrap()), 10);
    assert_eq!(process_part1(lines.next().unwrap()), 11);
}

#[cfg(test)]
#[test]
fn part2_works() {
    let input = include_str!("day6_test.txt");
    let mut lines = input.lines();
    
    assert_eq!(process_part2(lines.next().unwrap()), 19);
    assert_eq!(process_part2(lines.next().unwrap()), 23);
    assert_eq!(process_part2(lines.next().unwrap()), 23);
    assert_eq!(process_part2(lines.next().unwrap()), 29);
    assert_eq!(process_part2(lines.next().unwrap()), 26);
}