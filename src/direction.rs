use std::fmt;

/// Represents each of the sides of the image.
///
/// Also used to represent the index of the neighbors of a slot.
/// Up = 0, Right = 1, Down = 2, Left = 3
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    /// Returns a vector of all the directions in order.
    ///
    /// ```
    /// use billow::Direction;
    ///
    /// for direction in Direction::all() {
    ///    println!("{:?}", direction);
    /// }
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }

    /// Returns the opposite direction.
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "up"),
            Direction::Right => write!(f, "right"),
            Direction::Down => write!(f, "down"),
            Direction::Left => write!(f, "left"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Direction::Up.reverse(), Direction::Down);
        assert_eq!(Direction::Right.reverse(), Direction::Left);
        assert_eq!(Direction::Down.reverse(), Direction::Up);
        assert_eq!(Direction::Left.reverse(), Direction::Right);
    }

    #[test]
    fn test_display() {
        assert_eq!(Direction::Up.to_string(), "up");
        assert_eq!(Direction::Right.to_string(), "right");
        assert_eq!(Direction::Down.to_string(), "down");
        assert_eq!(Direction::Left.to_string(), "left");
    }

    #[test]
    fn test_as_usize() {
        assert_eq!(Direction::Up as usize, 0);
        assert_eq!(Direction::Right as usize, 1);
        assert_eq!(Direction::Down as usize, 2);
        assert_eq!(Direction::Left as usize, 3);
    }
}
