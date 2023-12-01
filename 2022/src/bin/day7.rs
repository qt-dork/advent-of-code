use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

#[derive(Debug)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
    children: Vec<FsEntry>,
}

impl FsEntry {
    fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<u64>()
    }

    fn all_dirs(&self) -> Box<dyn Iterator<Item = &FsEntry> + '_> {
        Box::new(
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|c| !c.children.is_empty())
                    .flat_map(|c| c.all_dirs()),
            )
        )
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap(); 

    let input = include_str!("day7.txt");
    let result = process_part1(input)?;
    let result2 = process_part2(input)?;
    println!("{}", result);
    println!("{}", result2);

    Ok(())
}

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into
    )(i)
}
#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf)
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

fn process_part1(i: &str) -> color_eyre::Result<u64, color_eyre::Report> {
    let lines = i
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);
    
    let mut stack = vec![FsEntry {
        path: "/".into(),
        size: 0,
        children: vec![],
    }];

    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // ignore
                },
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore
                    },
                    ".." => {
                        let child = stack.pop();
                        stack.last_mut().unwrap().children.push(child.unwrap());
                    },
                    _ => {
                        let node = FsEntry {
                            path: path.clone(),
                            size: 0,
                            children: vec![],
                        };
                        stack.push(node);
                    },
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // ignore
                },
                Entry::File(size, path) => {
                    let node = FsEntry {
                        size,
                        path,
                        children: vec![],
                    };
                    stack.last_mut().unwrap().children.push(node);
                }
            },
        }
    }

    let mut root = stack.pop().unwrap();
    while let Some(mut next) = stack.pop() {
        next.children.push(root);
        root = next;
    }
    dbg!(&root);

    let sum = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>();
    dbg!(sum);

    Ok(sum)
}

fn process_part2(i: &str) -> color_eyre::Result<u64, color_eyre::Report> {
    let lines = i
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);
    
    let mut stack = vec![FsEntry {
        path: "/".into(),
        size: 0,
        children: vec![],
    }];

    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // ignore
                },
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore
                    },
                    ".." => {
                        let child = stack.pop();
                        stack.last_mut().unwrap().children.push(child.unwrap());
                    },
                    _ => {
                        let node = FsEntry {
                            path: path.clone(),
                            size: 0,
                            children: vec![],
                        };
                        stack.push(node);
                    },
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // ignore
                },
                Entry::File(size, path) => {
                    let node = FsEntry {
                        size,
                        path,
                        children: vec![],
                    };
                    stack.last_mut().unwrap().children.push(node);
                }
            },
        }
    }

    let mut root = stack.pop().unwrap();
    while let Some(mut next) = stack.pop() {
        next.children.push(root);
        root = next;
    }
    dbg!(&root);

    let sum = root.total_size();
    let unused_space = 70000000 - sum;
    println!("Total Space: {}\nOccupied Space: {}\nUnused Space: {}\n", 70000000, sum, unused_space);
    dbg!(unused_space);
    let necessary_space = 30000000 - unused_space;
    println!("Space needed to delete for update: {}", necessary_space);
    dbg!(necessary_space);

    let dir = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s >= necessary_space)
        .min()
        .unwrap();
    dbg!(dir);

    Ok(dir)
}

#[cfg(test)]
#[test]
fn part1_works() {
    let input = include_str!("day7_test.txt");
    assert_eq!(process_part1(input).unwrap(), 95437);
}

#[cfg(test)]
#[test]
fn part2_works() {
    let input = include_str!("day7_test.txt");
    assert_eq!(process_part2(input).unwrap(), 24933642);
}