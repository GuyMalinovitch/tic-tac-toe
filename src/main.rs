#![allow(non_snake_case)]
#![allow(unused_imports)]
mod board;
mod userIO;
mod gui;
use crate::board::*;
use crate::userIO::{print_info, print_warning};

fn run_no_gui() {
    let mut board = Board::new();

    loop {
        board.display();
        let coordinates = userIO::get_input_coordinates(*board.get_current_player());
        if coordinates.is_empty() {continue}
        if let Err(e) = board.make_move(coordinates[0], coordinates[1]) {
            print_warning(e)
        }
        if let Ok(msg) = board.check_win() {
            print_info(msg.as_str());
            break;
        }

        board.switch_player();
    }
    print_info("Done");
}

fn run_gui() {
    gui::start_gui();
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len()>1 && &args[1] == "gui" {
        run_gui();
    }
    else {
        run_no_gui();
    }
}
