#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn get_offset(&self) -> (i8, i8) {
        use Direction::*;
        match self {
            North => (-1, 0),
            South => (1, 0),
            East => (0, 1),
            West => (0, -1),
        }
    }

    pub fn get_opposite(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}
