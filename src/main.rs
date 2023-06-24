#![allow(non_snake_case)]
mod board;
mod userIO;
use crate::board::*;


fn main() {
    let mut board = Board::new();

    let mut current_player = 'X';
    loop {
        board.display();
        let coordinates = userIO::get_input_coordinates(current_player);
        if coordinates.is_empty() {continue}
        if !board.make_move(coordinates[0], coordinates[1], current_player)
        {
            continue;
        }
        if board.check_win(current_player) {
            break;
        }
        board.switch_player(&mut current_player);
    }
    println!("Done");
}
