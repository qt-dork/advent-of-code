use std::fs;
use std::collections::HashMap;

// this is probably really inefficient
// update: hashmaps are probably more efficient idk
// the algorithm is at least O(n) (i think),
// which is better than the O(n^2) algorithm i used before
// update update: everyone else used sets. i should've used
// them too. unfortunately, i forgot about them lol
fn find_mistake((first, last): (&str, &str)) -> Vec<char> {
    let mut totals: HashMap<char, u32> = HashMap::new();
    first.chars().for_each(|c| {
        let count = totals.entry(c).or_insert(0);
        *count += 1;
    });
    // check if any character in the second half happens to in the first half
    last.chars().filter(|c| {
        let count = totals.entry(*c).or_insert(0);
        count > &mut 0
    }).collect()
}



fn find_badge(text: &[&str]) -> Option<char> {
    let part_one: Vec<String> = text.windows(2).map(|pair| {
        find_mistake((pair[0], pair[1])).into_iter().collect::<String>()
    }).collect();
    let part_two = find_mistake((&part_one[0], &part_one[1]));
    part_two.into_iter().next()
}

fn convert_to_value(c: char) -> Option<u32> {
    if c >= 'a' && c <= 'z' {
        Some(c as u32 - 'a' as u32 + 1)
    } else if c >= 'A' && c <= 'Z' {
        Some(c as u32 - 'A' as u32 + 27)
    } else {
        None
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    //testing
//     let input = String::from("vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg
// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw");

    let lines = input.lines();
    // part 1
    // the .iter().next().cloned() is just to get the only element from part 1
    // idk if i need to do that, but i don't like using [0] instead
    let result = lines.clone().map(|line| find_mistake(line.split_at(line.len() / 2)).iter().next().cloned()).filter_map(|x| convert_to_value(x?)).sum::<u32>();
    println!("{}", result);

    // part 2
    let result_two = lines.collect::<Vec<&str>>().chunks(3).map(|text| find_badge(text)).filter_map(|x| convert_to_value(x?)).sum::<u32>();
    println!("{}", result_two);

    // testing
    // assert_eq!(lines.map(|x| find_mistake(x)).collect::<Vec<_>>(), vec![
    //     Some('p'),
    //     Some('L'),
    //     Some('P'),
    //     Some('v'),
    //     Some('t'),
    //     Some('s'),
    // ]);
}
