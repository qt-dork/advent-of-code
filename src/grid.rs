#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Converts the letters "U, D, L, R" to direction.
/// Any other inputs fail
impl TryFrom<char> for Direction {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(color_eyre::eyre::eyre!("Not a valid move: {c:?}"))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    fn in_bounds(&self, pos: Pos) -> bool {
        pos.x < self.width && pos.y < self.height
    }
}