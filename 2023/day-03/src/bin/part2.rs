use std::usize;

use itertools::Itertools;

fn main() {
    let input = include_str!("sample.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    pub x: usize,
    pub y: usize,
}

// impl Coord {
//     fn from((x, y): (usize, usize)) -> Self {
//         Self { x, y }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PartType {
    Symbol,
    Number(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part {
    pub pos: Coord,
    pub width: usize,
    pub kind: PartType,
}

impl Part {
    /// Get the start and end coordinates of a part.
    ///
    /// Useful if a part occupies multiple spaces. Assumes parts
    /// can only occupy multiple spaces horizontally.
    fn coords(&self) -> (Coord, Coord) {
        (
            self.pos,
            Coord {
                x: self.pos.x + self.width - 1,
                y: self.pos.y,
            },
        )
    }

    /// Returns the coordinates of a box that spans the
    /// range of a part's "sight". This is used so you
    /// can see if a symbol/part is in the line of sight
    /// of another symbol/part.
    ///
    /// The box is in the schema of (top left, bottom right)
    fn line_of_sight(&self) -> (Coord, Coord) {
        (
            // Top Left
            // Won't check if it's in bounds because i'm cringe
            Coord {
                x: self.pos.x - 1,
                y: self.pos.y - 1,
            },
            // Bottom Right
            Coord {
                x: self.pos.x + 1,
                y: self.pos.y + 1,
            },
        )
    }
}

#[derive(Debug, Clone)]
struct Grid {
    _width: usize,
    _height: usize,
    // it kinda sucks that the vector doesn't match the shape of the data
    // but since parts can occupy multiple squares it feels easier to instead
    // have to read every part individually to see if parts intersect. tragic
    pub items: Vec<Part>,
}

impl Grid {
    pub fn new((width, height): (usize, usize)) -> Self {
        Self {
            _width: width,
            _height: height,
            items: Vec::new(),
        }
    }
}

fn parse(i: &str) -> Grid {
    let input = i.to_owned();
    let width = input.lines().next().unwrap().len();
    let height: usize = input.lines().map(|_x| 1).sum();
    let mut grid = Grid::new((width, height));
    let parts: Vec<Part> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.match_indices(|c: char| c.is_numeric() || c == '*')
                .map(|x| (x.0, x.1.to_owned()))
                .rev()
                .coalesce(|cur, next| {
                    if cur.1.parse::<usize>().is_ok()
                        && next.1.parse::<usize>().is_ok()
                        && cur.0 == next.0 + 1
                    {
                        let merge = format!("{}{}", next.1, cur.1);
                        Ok((next.0, merge))
                    } else {
                        Err((cur, next))
                    }
                })
                .map(move |(x, elem)| {
                    if elem.parse::<usize>().is_ok() {
                        Part {
                            pos: Coord { x, y },
                            width: elem.len(),
                            // please don't get mad
                            kind: PartType::Number(elem.parse().unwrap()),
                        }
                    } else {
                        Part {
                            pos: Coord { x, y },
                            width: 1,
                            kind: PartType::Symbol,
                        }
                    }
                })
        })
        .collect();

    grid.items = parts;
    grid
}

fn part2(i: &str) -> String {
    let grid = parse(i);

    // return all items in scope of symbol
    let items: usize = grid
        .items
        .iter()
        .flat_map(|part| match part.kind {
            PartType::Symbol => {
                let mut parts = Vec::new();

                for sub_part in grid.items.iter() {
                    if sub_part.kind != PartType::Symbol && {
                        let (start_coord, end_coord) = sub_part.coords();
                        let (top_left, bottom_right) = part.line_of_sight();
                        // abandon all hope all ye who enter here
                        start_coord.x >= top_left.x
                            && start_coord.x <= bottom_right.x
                            && start_coord.y >= top_left.y
                            && start_coord.y <= bottom_right.y
                            || end_coord.x >= top_left.x
                                && end_coord.x <= bottom_right.x
                                && end_coord.y >= top_left.y
                                && end_coord.y <= bottom_right.y
                    } {
                        parts.push(sub_part);
                    }
                }
                Some(parts.into_iter())
            }
            PartType::Number(_) => None,
        })
        .filter(|x| x.len() == 2)
        .flat_map(|x| {
            let y = x.map(|elem| {
                if let PartType::Number(i) = elem.kind {
                    i
                } else {
                    unreachable!()
                }
            });
            y.reduce(|accum, e| accum * e)
        })
        .sum();
    items.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = part2(input);
        assert_eq!(result, "467835");
    }
}
