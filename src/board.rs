use std::fmt;

pub struct Board {
    grid: [[Option<Piece>; 8]; 8],
    turn: Player,
}

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

#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "P"),
            PieceType::Knight => write!(f, "N"),
            PieceType::Bishop => write!(f, "B"),
            PieceType::Rook => write!(f, "R"),
            PieceType::Queen => write!(f, "Q"),
            PieceType::King => write!(f, "K"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    piece: PieceType,
    player: Player,
}

impl Piece {
    pub fn pawn(player: Player) -> Self {
        Self::new(PieceType::Pawn, player)
    }

    pub fn knight(player: Player) -> Self {
        Self::new(PieceType::Knight, player)
    }

    pub fn bishop(player: Player) -> Self {
        Self::new(PieceType::Bishop, player)
    }

    pub fn rook(player: Player) -> Self {
        Self::new(PieceType::Rook, player)
    }

    pub fn queen(player: Player) -> Self {
        Self::new(PieceType::Queen, player)
    }

    pub fn king(player: Player) -> Self {
        Self::new(PieceType::King, player)
    }

    pub fn new(piece: PieceType, player: Player) -> Self {
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
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::with_capacity(20);

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
