struct Board {
    board: [[char; 3]; 3],
    move_counter: usize,
}

impl Board {
    fn new() -> Self {
        Board {
            board: [[' ';3]; 3],
            move_counter: 0,
        }
    }
    fn len(&self) -> usize {
        self.board[0].len() * self.board.len()
    }
    fn display(&self) {
        for row in &self.board {
            println!("-------------------");
            for &cell in row {
                print!("|  {}  ", cell);
            }
            println!("|");
        }
        println!("-------------------");
    }
    fn make_move(&mut self, row: usize, col: usize, symbol: char) -> bool{
        if row > 3 || row < 1  {
            println!("Invalid row, try again");
            return false;
        }
        if col > 3 || col < 1
        {
            println!("Invalid column, try again");
            return false;
        }
        self.board[row-1][col-1] = symbol;
        self.move_counter+=1;
        true
    }
    fn is_full(&self) -> bool {
        self.move_counter == self.len()
    }
    fn check_win(&self) -> bool {
        if self.is_full()
        {
            println!("Draw! no one wins today");
            return true;
        }
        false
    }
}

fn get_input_coordinates(current_player: char) -> Vec<usize> {
    loop {
        println!("Current Player: {}", current_player);
        println!("Enter your move (row, col, e.g. 0 1:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input !");
        let coordinates: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input!"))
            .collect();
        if !coordinates.is_empty()
        {
            return coordinates;
        }
        println!("Empty move, try again..");
    }
}

fn switch_player(current_player: &mut char)  {
    if *current_player == 'X'
    {
        *current_player = 'O';
    }
    else
    {
        *current_player = 'X';
    }
}

fn main() {
    let mut board = Board::new();

    board.display();

    let mut current_player = 'X';
    loop {
        let coordinates = get_input_coordinates(current_player);
        board.make_move(coordinates[0], coordinates[1], current_player);
        board.display();
        if board.check_win() {
            break;
        }
        switch_player(&mut current_player);

    }
    println!("Done");
}