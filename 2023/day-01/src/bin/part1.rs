
fn main() {
    let input = include_str!("sample.txt");
    let output = part1(input);
    dbg!("{}", output);
}

fn parse(input: &str) -> usize {
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
    fn it_works() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        let result = part1(input);
        assert_eq!(result, "142");
    }
}