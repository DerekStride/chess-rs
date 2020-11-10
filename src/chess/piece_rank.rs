// use std::fmt;

// use crate::chess::board::Board;
// use crate::chess::game_move::GameMove;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Board {
    grid: [[u8; 8]; 8],
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GameMove {
    dest_x: u8,
    dest_y: u8,
    // rank: &PieceRank,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Pawn {
    position_x: u8,
    position_y: u8,
}

impl PieceRank for Pawn {
    fn avaliable_moves(&self, board: Board) -> Vec<GameMove> {
        let gmove = GameMove {
            dest_x: 5,
            dest_y: 3,
            // rank: self,
        };

        vec![gmove]
    }
}


pub trait PieceRank {
    fn avaliable_moves(&self, board: Board) -> Vec<GameMove>;
}

// #[derive(Debug, Copy, Clone, PartialEq)]
// pub enum PieceRank {
//     Pawn,
//     Knight,
//     Bishop,
//     Rook,
//     Queen,
//     King,
// }

// impl PieceRank {
//     pub fn move_str(&self) -> String {
//         match self {
//             PieceRank::Pawn => String::from(""),
//             PieceRank::Knight => String::from("N"),
//             PieceRank::Bishop => String::from("B"),
//             PieceRank::Rook => String::from("R"),
//             PieceRank::Queen => String::from("Q"),
//             PieceRank::King => String::from("K"),
//         }
//     }
// }

// impl fmt::Display for PieceRank {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             PieceRank::Pawn => write!(f, "P"),
//             PieceRank::Knight => write!(f, "N"),
//             PieceRank::Bishop => write!(f, "B"),
//             PieceRank::Rook => write!(f, "R"),
//             PieceRank::Queen => write!(f, "Q"),
//             PieceRank::King => write!(f, "K"),
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let board = Board {
            grid: [[0; 8]; 8],
        };
        let pawn = Pawn {
            position_x: 4,
            position_y: 1,
        };
        let moves = pawn.avaliable_moves(board);

        let gmoves: Vec<GameMove> = vec![];

        assert_eq!(gmoves, moves);
    }
}
