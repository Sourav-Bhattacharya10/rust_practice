// Manually implementing Display trait for enum
use std::fmt;

pub enum CardinalDirections {
    North,
    South(String),
    East,
    West
}

impl fmt::Display for CardinalDirections {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardinalDirections::North => write!(fmt, "0"),
            CardinalDirections::South(name) => write!(fmt, "{}", name),
            CardinalDirections::East => write!(fmt, "2"),
            CardinalDirections::West => write!(fmt, "3"),
        }
    }
}