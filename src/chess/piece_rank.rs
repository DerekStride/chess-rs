use std::fmt;

#[derive(Copy, Clone)]
pub enum PieceRank {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl fmt::Display for PieceRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieceRank::Pawn => write!(f, "P"),
            PieceRank::Knight => write!(f, "N"),
            PieceRank::Bishop => write!(f, "B"),
            PieceRank::Rook => write!(f, "R"),
            PieceRank::Queen => write!(f, "Q"),
            PieceRank::King => write!(f, "K"),
        }
    }
}
