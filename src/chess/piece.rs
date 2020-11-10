use std::fmt;

use crate::chess::piece_rank::PieceRank;
use crate::chess::player::Player;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    rank: PieceRank,
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

    fn new(rank: PieceRank, player: Player) -> Self {
        Self {
            rank,
            player,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.player {
            Player::White => write!(f, "w{}", self.rank),
            Player::Black => write!(f, "b{}", self.rank),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_returns_a_piece_with_type_pawn() {
        let expected = Piece {
            rank: PieceRank::Pawn,
            player: Player::White,
        };

        assert_eq!(expected, Piece::pawn(Player::White));
    }
}
