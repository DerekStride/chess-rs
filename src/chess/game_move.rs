use std::fmt;

use crate::chess::piece_rank::PieceRank;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GameMove {
    rank: PieceRank<T>,
    src_x: u8,
    src_y: u8,
    dest_x: u8,
    dest_y: u8,
    capture: bool,
    check: bool,
    checkmate: bool,
}

impl GameMove {
    pub fn new(rank: PieceRank, src_x: u8, src_y: u8, dest_x: u8, dest_y: u8, capture: bool, check: bool, checkmate: bool) -> Self {
        Self {
            rank,
            src_x,
            src_y,
            dest_x,
            dest_y,
            capture,
            check,
            checkmate,
        }
    }

    fn src_file(&self) -> char {
        Self::get_file(self.src_x)
    }

    fn dest_file(&self) -> char {
        Self::get_file(self.dest_x)
    }

    fn get_file(x: u8) -> char {
        match x {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            8...255 => '?',
        }
    }
}

impl fmt::Display for GameMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::with_capacity(5);

        if self.capture {
            match self.rank {
                PieceRank::Pawn => buffer.push(self.src_file()),
                _ => buffer.push_str(&self.rank.move_str()),
            }
            buffer.push_str("x");
        }

        buffer.push(self.dest_file());
        buffer.push_str(&format!("{}", self.dest_y + 1));

        if self.check {
            buffer.push_str("+");
        } else if self.checkmate {
            buffer.push_str("#");
        }

        write!(f, "{}", buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_formats_a_pawn_move() {
        let game_move = GameMove::new(PieceRank::Pawn, 4, 1, 4, 3, false, false, false);
        assert_eq!("e4", format!("{}", game_move));
    }

    #[test]
    fn it_formats_a_pawn_capture() {
        let game_move = GameMove::new(PieceRank::Pawn, 4, 1, 3, 2, true, false, false);
        assert_eq!("exd3", format!("{}", game_move));
    }

    #[test]
    fn checkmate() {
        let game_move = GameMove::new(PieceRank::Queen, 4, 4, 5, 5, true, false, true);
        assert_eq!("Qxf6#", format!("{}", game_move));
    }
}
