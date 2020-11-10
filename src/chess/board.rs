use std::fmt;

use crate::chess::piece::Piece;
use crate::chess::player::Player;

pub struct Board {
    grid: [[Option<Piece>; 8]; 8],
    turn: Player,
}

impl Board {
    pub fn new() -> Self {
        let mut initial_grid: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        initial_grid[0][0] = Some(Piece::rook(Player::Black));
        initial_grid[0][1] = Some(Piece::knight(Player::Black));
        initial_grid[0][2] = Some(Piece::bishop(Player::Black));
        initial_grid[0][3] = Some(Piece::queen(Player::Black));
        initial_grid[0][4] = Some(Piece::king(Player::Black));
        initial_grid[0][5] = Some(Piece::bishop(Player::Black));
        initial_grid[0][6] = Some(Piece::knight(Player::Black));
        initial_grid[0][7] = Some(Piece::rook(Player::Black));

        for number in 0..8 {
            initial_grid[1][number] = Some(Piece::pawn(Player::Black));
            initial_grid[6][number] = Some(Piece::pawn(Player::White));
        }

        initial_grid[7][0] = Some(Piece::rook(Player::White));
        initial_grid[7][1] = Some(Piece::knight(Player::White));
        initial_grid[7][2] = Some(Piece::bishop(Player::White));
        initial_grid[7][3] = Some(Piece::queen(Player::White));
        initial_grid[7][4] = Some(Piece::king(Player::White));
        initial_grid[7][5] = Some(Piece::bishop(Player::White));
        initial_grid[7][6] = Some(Piece::knight(Player::White));
        initial_grid[7][7] = Some(Piece::rook(Player::White));


        Self {
            grid: initial_grid,
            turn: Player::White,
        }
    }

    // pub fn available_moves(&self) -> [Move] {

    // }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::with_capacity(264);

        for (i, row) in self.grid.iter().enumerate() {
            buffer.push_str(&format!("{} |", 8 - i));
            for tile in row.iter() {
                match tile {
                    Some(piece) => {
                        buffer.push_str(&piece.to_string());
                        buffer.push_str("|");
                    }
                    None => {
                        buffer.push_str("  |");
                    }
                }
            }
            buffer.push_str("\n");
        }

        buffer.push_str("   a  b  c  d  e  f  g  h\n");
        buffer.push_str(&format!("{} to move.", self.turn));

        write!(f, "{}", buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes_correctly() {
        let mut buffer = String::with_capacity(264);
        buffer.push_str("8 |bR|bN|bB|bQ|bK|bB|bN|bR|\n");
        buffer.push_str("7 |bP|bP|bP|bP|bP|bP|bP|bP|\n");
        buffer.push_str("6 |  |  |  |  |  |  |  |  |\n");
        buffer.push_str("5 |  |  |  |  |  |  |  |  |\n");
        buffer.push_str("4 |  |  |  |  |  |  |  |  |\n");
        buffer.push_str("3 |  |  |  |  |  |  |  |  |\n");
        buffer.push_str("2 |wP|wP|wP|wP|wP|wP|wP|wP|\n");
        buffer.push_str("1 |wR|wN|wB|wQ|wK|wB|wN|wR|\n");
        buffer.push_str("   a  b  c  d  e  f  g  h\n");
        buffer.push_str("White to move.");

        assert_eq!(buffer, format!("{}", Board::new()));
        assert_eq!(buffer.len(), 264);
    }
}
