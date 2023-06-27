#![allow(non_snake_case)]
mod board;
mod userIO;
use crate::board::*;
use crate::userIO::{print_info, print_warning};


fn main() {
    let mut board = Board::new();

    let mut current_player = 'X';
    loop {
        board.display();
        let coordinates = userIO::get_input_coordinates(current_player);
        if coordinates.is_empty() {continue}
        if let Err(e) = board.make_move(coordinates[0], coordinates[1], current_player) {
            print_warning(e)
        }
        if let Ok(e) = board.check_win(current_player) {
            print_info(e);
            break;
        }

        board.switch_player(&mut current_player);
    }
    print_info("Done");
}
