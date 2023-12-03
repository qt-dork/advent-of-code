use std::cmp::{self, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cubes {
    pub amt: usize,
    pub color: Color,
}

impl PartialOrd for Cubes {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.color == other.color {
            Some(self.amt.cmp(&other.amt))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bag(pub Cubes, pub Cubes, pub Cubes);

impl Default for Bag {
    fn default() -> Self {
        Bag(
            Cubes {
                amt: 0,
                color: Color::Red,
            },
            Cubes {
                amt: 0,
                color: Color::Green,
            },
            Cubes {
                amt: 0,
                color: Color::Blue,
            },
        )
    }
}

// This feels like shit code lol
impl PartialOrd for Bag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.0.amt <= other.0.amt
            && self.1.amt <= other.1.amt
            && self.2.amt <= other.2.amt
        {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for Bag {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.0.amt <= other.0.amt
            && self.1.amt <= other.1.amt
            && self.2.amt <= other.2.amt
        {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Bag {
    pub fn insert(&mut self, cube: &Cubes) {
        match cube.color {
            Color::Red => self.0.amt = cube.amt,
            Color::Green => self.1.amt = cube.amt,
            Color::Blue => self.2.amt = cube.amt,
        }
    }

    /// Returns the maximum value of each cube
    pub fn maximum(&self, other: Self) -> Self {
        Bag(
            Cubes {
                amt: cmp::max(self.0.amt, other.0.amt),
                color: Color::Red,
            },
            Cubes {
                amt: cmp::max(self.1.amt, other.1.amt),
                color: Color::Green,
            },
            Cubes {
                amt: cmp::max(self.2.amt, other.2.amt),
                color: Color::Blue,
            },
        )
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub pos: usize,
    pub pulls: Vec<Bag>,
}
