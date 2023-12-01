
fn main() {
    let input = include_str!("sample.txt");

    let output = part1(input);
    println!("{}", output);
}

fn parse(input: &str) -> usize {
    let list_word = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].into_iter();
    let list_num = ["1", "2", "3", "4", "5", "6", "7", "8", "9"].into_iter();
    let list_chain = list_word.zip(list_num);
    
    let mut input = input.to_string();
    dbg!(input.clone());
    // least functional code on earth ahead
    for (from, to) in list_chain {
        while input.contains(from){
            // letters can be reused fuck me
            let from_first = from.chars().next().unwrap();
            let from_last = from.chars().next_back().unwrap();
            let new_to = format!("{}{}{}", from_first, to, from_last);

            input = input.replace(from, &new_to)
        }
    }
    dbg!(input.clone());
    let chars = input.chars();
    let mut nums = chars.filter(|x| x.is_ascii_digit());

    // I already did my error checking up here
    let first = nums.next().unwrap_or('0');
    let last = nums.next_back().unwrap_or(first);
    let mut first_last = first.to_string();
    first_last.push(last);

    // So I don't need to do it down here
    let output:usize = first_last.parse().unwrap();
    output
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let nums = lines.map(parse).clone();
    let sum:usize = nums.sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        let result = part1(input);
        assert_eq!(result, "142");
    }

    #[test]
    fn part2_test() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let result = part1(input);
        assert_eq!(result, "281");
    }
}