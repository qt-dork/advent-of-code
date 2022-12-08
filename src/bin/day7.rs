use nom::{
    *,
    branch::alt,
    bytes::complete::{tag, take, take_while1, take_until1},
    character::complete::char,
    combinator::{all_consuming, map, opt, map_res},
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult,
};

fn main() {
    let input = include_str!("day7.txt");
}

// ===============
// Data Structures
// ===============

// Probably useless? Can be derived 
#[derive(Debug, Clone, Copy)]
enum FileKind {
    File,
    Folder
}

// Maybe a file type?
struct File<'a> {
    name: &'a str,
    parent: Option<Id>,
    size: Option<usize>,
    kind: FileKind,
}

struct Folder<'a> {
    name: &'a str,
    parent: Option<Id>,
}

type Id = usize;

#[derive(Debug, Clone)]
struct FileTree {
    id: Vec<Id>,
    next_id: Id,
    // Only the root doesn't have a parent
    parent: Vec<Option<Id>>,
    // I should use a &str, but I'm not going to
    // stress out this much about the least important
    // part of the file
    name: Vec<String>,
    kind: Vec<FileKind>,
    // All files should eventually have a size, 
    // but folders can only determine their size
    // once the entire file tree is built.
    size: Vec<Option<usize>>
}

impl FileTree {
    fn new() -> Self {
        FileTree { 
            id: Vec::new(), 
            next_id: 0, 
            parent: Vec::new(),
            name: Vec::new(), 
            kind: Vec::new(), 
            size: Vec::new() 
        }
    }

    fn insert(
        &mut self,
        (name, parent, kind, size): (&str, Option<Id>, FileKind, Option<usize>)
    ) -> Id {
        let id = self.next_id;
        self.next_id += 1;
        self.id.push(id);
        self.name.push(name.to_string());
        self.parent.push(parent);
        self.kind.push(kind);
        self.size.push(size);

        id
    }
}

// ============
// Text Parsing
// ============

// Breaking it down
// 
// ┌── Parse a command (command branch)
// │     ┌── Next one is parent folder name
// │     │   (use to find parent id for FileTree)
// │     │      ┌── Parent folder name
// │     │      │
// [ $ ] [ cd ] [ / ]
// [ $ ] [ ls ]  
//       └── Skip
// 
// ┌── Skip
// [ dir ] [ a ]
// [ 14848514 ] [ b.txt ]
// │            └── File Name
// └── Size is `Some(size)`
// 
// [ $ ] [ cd ]  [ .. ]
//               └── Skip

// ------------
// File Parsing
// ------------
fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

fn parse_name(
    i: &str
) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphabetic() || c.is_ascii_punctuation())(i)
}

fn parse_file_or_folder(
    (i, parent): (&str, Id)
) -> IResult<&str, Option<File>> {
    alt((
        map(
            preceded(tag("dir "), parse_name),
            |_| None
        ),
        map(
            separated_pair(
                parse_number,
                char(' '),
                parse_name
            ),
            |(size, name)| {
                Some(File { 
                    name, 
                    parent: Some(parent), 
                    size: Some(size), 
                    kind: FileKind::File 
                })
            }
        )
    ))(i)
}

// -------------------
// Instruction Parsing
// -------------------

fn parse_back_or_name(i: &str) -> IResult<&str, Option<&str>> {
    alt((map(tag(".."), |_| None), map(parse_name, Some)))(i)
}

fn parse_cd(i: &str) -> IResult<&str, Option<&str>> {
    preceded(tag("cd "), parse_back_or_name)(i)
}

// I could generalize `parse_dir()` instead of this
// but it's 5 am.
fn parse_ls(i: &str) -> IResult<&str, ()> {
    map(tag("ls"), drop)(i)
}

// Once again I could generalize `parse_size_or_dir`,
// but once again, it's 5 am
/// Parses a `cd` or `ls`.
/// Returns `Some(folder_name)` if the command is `cd`
/// and [`None`] if the command is ls
fn parse_cd_or_ls(i: &str) -> IResult<&str, Option<&str>> {
    alt((parse_cd, map(parse_ls, |_| None)))(i)
}

// this could all be so much more concise aaaaaa
fn parse_instruction((i, parent): (&str, Option<usize>)) -> IResult<&str, Option<Folder>> {
    map(
        preceded(tag("$ "), parse_cd_or_ls),
        |some_name| {
            match some_name {
                Some(name) => Some(Folder {
                    name,
                    parent
                }),
                None => None
            }
        }
    )(i)
}

fn parse_set(i: &str) -> IResult<&str, (&str, Vec<File>)> {
    
}



#[cfg(test)]
#[test]
fn part1_works() {
    let input = include_str!("day7_test.txt");
    assert_eq!(process_part1(input), 95437);
}

// #[cfg(test)]
// #[test]
// fn part2_works() {
//     let input = include_str!("day5_test.txt");
//     assert_eq!(process_part2(input), "MCD");
// }