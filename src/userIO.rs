pub fn print_warning(string: &str)
{
    println!("[tic-tac-toe] WARNING: {}", string);
}
pub fn print_info(string: &str)
{
    println!("[tic-tac-toe] INFO: {}", string);
}
pub fn get_input_coordinates(current_player: char) -> Vec<usize> {
    loop {
        print_info(format!("[tic-tac-toe] Current Player: {}", current_player).as_str());
        print_info("[tic-tac-toe] Enter your move (row, col, e.g. 1 1:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input !");
        let coordinates: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input!"))
            .collect();
        if !coordinates.is_empty() {
            if coordinates.len() > 2
            {
                print_warning(">2 coordinates provided, try again");
                continue;
            }
            return coordinates;
        }
        print_warning("Empty move, try again..");
    }
}