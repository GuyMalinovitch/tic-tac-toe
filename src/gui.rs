use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Button, Grid, Label, Window, WindowType};
use crate::board::*;

fn game_button_action(board: &mut Board, button: &mut Button, message_label: &mut Label, row: usize, col: usize)
{
    if board.has_game_ended() {
        return;
    }
    if let Err(msg) = board.make_move(row, col) {
        message_label.set_text(msg);
    }
    else {
        button.set_label(board.get_current_player().to_string().as_str());
        if let Ok(msg) = board.check_win() {
            message_label.set_text(msg.as_str());
        } else {
            message_label.set_text(board.switch_player().as_str());
        }
    }
}

/*fn restart_button_action(board: &mut Board, buttons: &mut Vec<Vec<Rc<RefCell<Button>>>>, message_label: &mut Label)
{

}*/

pub fn start_gui() {
    if let Err(err) = gtk::init() {
        eprintln!("Failed to initialize GTK: {}", err);
        return;
    }

    let board = Rc::new(RefCell::new(Board::new()));
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Tic Tac Toe");
    window.set_default_size(300, 300);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let grid = Grid::new();
    grid.set_row_homogeneous(true);
    grid.set_column_homogeneous(true);



    // Add GUI elements to the grid (buttons, labels, etc.)
    let mut buttons: Vec<Vec<Rc<RefCell<Button>>>> = Vec::new();
    for i in 0..3 {
        let mut row_buttons = Vec::new();

        for j in 0..3 {
            let button = Button::new();
            grid.attach(&button, j as i32, i as i32, 1, 1);
            row_buttons.push(Rc::new(RefCell::new(button)));
        }

        buttons.push(row_buttons);
    }
    let buttons_refs: Vec<Vec<_>> = buttons.iter().map(|row| row.iter().map(|button| button.clone()).collect()).collect();

    let message_label = Label::new(None);
    grid.attach(&message_label, 0, 3, 3, 1);
    let label = Rc::new(RefCell::new(message_label));

    // restart button
    let restart_button = Button::new();
    restart_button.set_label("Restart");
    grid.attach(&restart_button, 0, 4, 3, 1);
    let board_clone = Rc::clone(&board);
    let label_clone = Rc::clone(&label);
    restart_button.connect_clicked(move |_| {
        board_clone.borrow_mut().reset();
        for i in 0..3 {
            for j in 0..3 {
                let button_ref = &buttons_refs[i][j].borrow_mut();
                button_ref.set_label("");
            }
        }
        label_clone.borrow_mut().set_text("");
    });

    // game action
    for i in 0..3 {
        for j in 0..3 {
            let button = Rc::clone(&buttons[i][j]);
            let board_clone = Rc::clone(&board);
            let label_clone = Rc::clone(&label);
            button.borrow().set_size_request(100, 100);
            button.clone().borrow()
                .connect_clicked( move |_| {
                    game_button_action(&mut board_clone.borrow_mut(),
                                       &mut button.borrow_mut(),
                                       &mut label_clone.borrow_mut(),
                                       i + 1,
                                       j + 1);
                }
                );
        }
    }



    window.add(&grid);
    window.show_all();

    gtk::main();
}