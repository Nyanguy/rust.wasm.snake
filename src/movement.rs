#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn is_opposite(&self, other: Direction) -> bool {
        match other {
            Direction::Up    => self == Direction::Down,
            Direction::Down  => self == Direction::Up,
            Direction::Left  => self == Direction::Right,
            Direction::Right => self == Direction::Left,
        }
    }
}