#![allow(non_snake_case)]
mod board;
mod userIO;
use crate::board::*;


fn main() {
    let mut board = Board::new();

    board.display();

    let mut current_player = 'X';
    loop {
        let coordinates = userIO::get_input_coordinates(current_player);
        if coordinates.is_empty() {continue}
        board.make_move(coordinates[0], coordinates[1], current_player);
        board.display();
        if board.check_win(current_player) {
            break;
        }
        board.switch_player(&mut current_player);
    }
    println!("Done");
}
