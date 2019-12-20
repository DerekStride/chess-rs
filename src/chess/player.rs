use std::fmt;

#[derive(Copy, Clone)]
pub enum Player {
    White,
    Black,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::White => write!(f, "White"),
            Player::Black => write!(f, "Black"),
        }
    }
}
