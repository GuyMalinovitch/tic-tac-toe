const ROW_COL_SIZE: usize = 3;

pub struct Board {
    board: [[char; ROW_COL_SIZE]; ROW_COL_SIZE],
    move_counter: usize,
    current_player: char,
    game_ended: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[' '; ROW_COL_SIZE]; ROW_COL_SIZE],
            move_counter: 0,
            current_player: 'X',
            game_ended: false
        }
    }

    pub fn reset(&mut self) {
        for row in 0..3 {
            for col in 0..3 {
                self.board[row][col] = ' ';
            }
        }
        self.move_counter = 0;
        self.current_player = 'X';
        self.game_ended = false;
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

    pub fn has_game_ended(&self) -> bool {self.game_ended}

    pub fn make_move(&mut self, row: usize, col: usize) -> Result<bool, &'static str> {
        if row > ROW_COL_SIZE || row < 1 {
            return Err("Invalid row, try again");
        }
        if col > ROW_COL_SIZE || col < 1 {
            return Err("Invalid column, try again");
        }
        if self.board[row - 1][col - 1] != ' '
        {
            return Err("Invalid move, try again");
        }
        self.board[row - 1][col - 1] = *self.get_current_player();
        self.move_counter += 1;
        println!("{} made a move - {} {}", self.get_current_player(), row, col);
        Ok(true)
    }
    pub fn check_win(&mut self) -> Result<String, ()> {
        if self.check_rows() || self.check_cols() || self.check_diag() {
            self.game_ended = true;
            return Ok(format!("Player {} is the Winner !!!!", self.get_current_player()));
        }
        if self.is_full() {
            self.game_ended = true;
            return Ok(format!("Draw! no one wins today"));
        }
        Err(())
    }

    pub fn switch_player(&mut self)-> String {
        if *self.get_current_player() == 'X' {
            self.current_player = 'O';
        } else {
            self.current_player = 'X';
        }
        format!("Player {} it's your turn !", self.get_current_player())
    }

    pub fn get_current_player(&self) -> &char { &self.current_player }
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
        assert!(board.make_move(1 , 1).is_ok());
        board.switch_player();
        assert!(board.make_move(2, 2).is_ok());
        board.switch_player();
        assert!(board.make_move(3, 3).is_ok());
    }
    #[test]
    fn test_invalid_move() {
        let mut board = Board::new();
        assert!(board.make_move(1 , 1).is_ok());
        board.switch_player();
        assert!(!board.make_move(1, 1).is_ok());
        board.switch_player();
        assert!(!board.make_move(4, 4).is_ok());
        assert!(!board.make_move(0, 1).is_ok());
    }
    #[test]
    fn switch_player_test() {
        let mut board = Board::new();
        board.switch_player();
        assert_eq!(*board.get_current_player(), 'O');
        board.switch_player();
        assert_eq!(*board.get_current_player(), 'X');
    }
    #[test]
    fn check_full_test() {
        let mut board = Board::new();
        for i in 1..4 {
            for j in 1..4 {
                let _ = board.make_move(i, j);
                board.switch_player();
            }
        }
        assert!(board.is_full())
    }
    #[test]
    fn check_win_test1() {
        let mut board = Board::new();
        assert!(board.check_win().is_err());
        let _ = board.make_move(1, 1);
        let _ = board.make_move(2, 2);
        let _ = board.make_move(3, 3);
        assert!(board.check_win().is_ok());
    }
    #[test]
    fn check_win_test2() {
        let mut board = Board::new();
        assert!(board.check_win().is_err());
        let _ = board.make_move(1, 1);
        let _ = board.make_move(1, 2);
        let _ = board.make_move(1, 3);
        assert!(board.check_win().is_ok());
    }
    #[test]
    fn check_win_test3() {
        let mut board = Board::new();
        assert!(board.check_win().is_err());
        let _ = board.make_move(1, 1);
        let _ = board.make_move(2, 1);
        let _ = board.make_move(3, 1);
        assert!(board.check_win().is_ok());
    }
    #[test]
    fn check_win_test4() {
        let mut board = Board::new();
        assert!(board.check_win().is_err());
        let _ = board.make_move(1, 3);
        let _ = board.make_move(2, 2);
        let _ = board.make_move(3, 1);
        assert!(board.check_win().is_ok());
    }
}

