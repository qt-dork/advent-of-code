use std::fs;

fn main() {
    // file stuff (doesn't count)
    let path = "input.txt";
    let input = fs::read_to_string(path).expect("Something went wrong reading the file");

    let greatest: usize = input.split("\n\n").map(|elf| elf.split("\n").filter_map(|line| line.parse().ok()).reduce(|accum, x| accum + x).unwrap()).reduce(|accum, x| { if accum >= x { accum } else {x}}).unwrap();
    
    println!("{:?}", greatest);

    let mut list: Vec<usize> = input.split("\n\n").map(|elf| elf.split("\n").filter_map(|line| line.parse().ok()).fold(0, |accum, x: usize| accum + x)).collect();
    list.sort(); // I kinda gave up here and made it multiple lines
    list.reverse();
    list.truncate(3);
    println!("{:?}", list);
    let top_three_result = list.iter().fold(0, |accum, x| accum + x);
    println!("{:?}", top_three_result);
}
