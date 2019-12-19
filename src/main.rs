mod board;

use board::Board;

fn main() {
    let game =  Board::new();
    println!("{}", game);
}
