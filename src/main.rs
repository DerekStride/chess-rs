mod chess;

use crate::chess::board::Board;

fn main() {
    let game =  Board::new();
    println!("{}", game);
}
