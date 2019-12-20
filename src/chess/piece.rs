use std::fmt;

use crate::chess::piece_rank::PieceRank;
use crate::chess::player::Player;

#[derive(Copy, Clone)]
pub struct Piece {
    piece: PieceRank,
    player: Player,
}

impl Piece {
    pub fn pawn(player: Player) -> Self {
        Self::new(PieceRank::Pawn, player)
    }

    pub fn knight(player: Player) -> Self {
        Self::new(PieceRank::Knight, player)
    }

    pub fn bishop(player: Player) -> Self {
        Self::new(PieceRank::Bishop, player)
    }

    pub fn rook(player: Player) -> Self {
        Self::new(PieceRank::Rook, player)
    }

    pub fn queen(player: Player) -> Self {
        Self::new(PieceRank::Queen, player)
    }

    pub fn king(player: Player) -> Self {
        Self::new(PieceRank::King, player)
    }

    pub fn new(piece: PieceRank, player: Player) -> Self {
        Self {
            piece,
            player,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.player {
            Player::White => write!(f, "w{}", self.piece),
            Player::Black => write!(f, "b{}", self.piece),
        }
    }
}
