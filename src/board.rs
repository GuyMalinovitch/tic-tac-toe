use crate::userIO;

const ROW_COL_SIZE: usize = 3;

pub struct Board {
    board: [[char; ROW_COL_SIZE]; ROW_COL_SIZE],
    move_counter: usize,
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[' '; ROW_COL_SIZE]; ROW_COL_SIZE],
            move_counter: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.board[0].len() * self.board.len()
    }
    pub fn display(&self) {
        for row in &self.board {
            println!("-------------------");
            for &cell in row {
                print!("|  {}  ", cell);
            }
            println!("|");
        }
        println!("-------------------");
    }
    pub fn make_move(&mut self, row: usize, col: usize, symbol: char) -> bool {
        if row > ROW_COL_SIZE || row < 1 {
            userIO::print_warning("Invalid row, try again");
            return false;
        }
        if col > ROW_COL_SIZE || col < 1 {
            userIO::print_warning("Invalid column, try again");
            return false;
        }
        if self.board[row - 1][col - 1] != ' '
        {
            userIO::print_warning("Invalid move, try again");
            return false;
        }
        self.board[row - 1][col - 1] = symbol;
        self.move_counter += 1;
        true
    }
    pub fn check_win(&self, current_player: char) -> bool {
        if self.check_rows() || self.check_cols() || self.check_diag() {
            userIO::print_info(format!("Winner winner checking dinner ! {} is the Winner !!!!", current_player).as_str());
            return true;
        }
        if self.is_full() {
            userIO::print_info("Draw! no one wins today");
            return true;
        }
        false
    }
    pub fn switch_player(&self, current_player: &mut char) {
        if *current_player == 'X' {
            *current_player = 'O';
        } else {
            *current_player = 'X';
        }
    }
    fn is_full(&self) -> bool {
        self.move_counter == self.len()
    }
    fn check_rows(&self) -> bool {
        for row in self.board {
            if row[0] != ' ' && row[0] == row[1] && row[1] == row[2] {
                return true;
            }
        }
        false
    }
    fn check_cols(&self) -> bool {
        let board = self.board;
        for col in 0..ROW_COL_SIZE {
            if board[0][col] != ' '
                && board[0][col] == board[1][col]
                && board[1][col] == board[2][col]
            {
                return true;
            }
        }
        false
    }
    fn check_diag(&self) -> bool {
        let board = self.board;
        if (board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2])
            || (board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1] == board[2][0])
        {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let mut board = Board::new();
        assert!(board.make_move(1 , 1, 'X'));
        assert!(board.make_move(2, 2, 'O'));
        assert!(board.make_move(3, 3, 'X'));
    }
    #[test]
    fn test_invalid_move() {
        let mut board = Board::new();
        assert!(board.make_move(1 , 1, 'X'));
        assert!(!board.make_move(1, 1, 'O'));
        assert!(!board.make_move(4, 4, 'X'));
        assert!(!board.make_move(0, 1, 'X'));
    }
    #[test]
    fn switch_player_test() {
        let mut current_player = 'X';
        let board = Board::new();
        board.switch_player(&mut current_player);
        assert_eq!(current_player, 'O');
        board.switch_player(&mut current_player);
        assert_eq!(current_player, 'X');
    }
    #[test]
    fn check_full_test() {
        let mut board = Board::new();
        let current_player = 'X';
        for i in 1..4 {
            for j in 1..4 {
                board.make_move(i, j, current_player);
            }
        }
        assert!(board.is_full())
    }
    #[test]
    fn check_win_test1() {
        let mut board = Board::new();
        let current_player = 'X';
        assert!(!board.check_win(current_player));
        board.make_move(1, 1, current_player);
        board.make_move(2, 2, current_player);
        board.make_move(3, 3, current_player);
        assert!(board.check_win(current_player));
    }
    #[test]
    fn check_win_test2() {
        let mut board = Board::new();
        let current_player = 'O';
        assert!(!board.check_win(current_player));
        board.make_move(1, 1, current_player);
        board.make_move(1, 2, current_player);
        board.make_move(1, 3, current_player);
        assert!(board.check_win(current_player));
    }
    #[test]
    fn check_win_test3() {
        let mut board = Board::new();
        let current_player = 'X';
        assert!(!board.check_win(current_player));
        board.make_move(1, 1, current_player);
        board.make_move(2, 1, current_player);
        board.make_move(3, 1, current_player);
        assert!(board.check_win(current_player));
    }
    #[test]
    fn check_win_test4() {
        let mut board = Board::new();
        let current_player = 'O';
        assert!(!board.check_win(current_player));
        board.make_move(1, 3, current_player);
        board.make_move(2, 2, current_player);
        board.make_move(3, 1, current_player);
        assert!(board.check_win(current_player));
    }
}

