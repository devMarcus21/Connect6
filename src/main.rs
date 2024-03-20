mod boards;
mod game;

use crate::boards::boards::{Connect6Board, GameBoard};
use crate::game::result_states::MoveResults;
use crate::game::game_wrapper::{Connect6Game, Game};

fn clean_input_string(input: &mut String) {
    *input = input.replace("\r", "");
    *input = input.replace("\n", "");
}

fn valid_move_input_args(args: &Vec<&str>) -> bool {
    args[0].parse::<u8>().is_ok() && args[1].parse::<u8>().is_ok() && args[2].parse::<u8>().is_ok()
}

fn main() {
    let mut board = boards::boards::Connect6Board::new();
    let mut game = Connect6Game::new(&mut board);

    let mut run_game_loop = true;

    while run_game_loop {
        game.print();

        // Handle input string
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        clean_input_string(&mut input); // remove extra characters
        let input_args: Vec<&str> = input.split(' ').collect();

        // Invalid arguments
        let input_length = input_args.len();
        if input_length == 0 || input_length == 2 || input_length > 3 {
            println!("Error: Invalid input");
            continue;
        }

        // Game loop arguments
        if input_length == 1 {
            let command = input_args[0];
            if command == "stop" || command == "kill" {
                run_game_loop = false; // stop game
                continue;
            }
            println!("Error: Command");
            continue;
        }

        if !valid_move_input_args(&input_args) {
            println!("Error: Invalid move input");
            continue;
        }
        
        // Convert game move arguments to proper integer values
        let move_x = input_args[0].parse::<u8>().unwrap();
        let move_y = input_args[1].parse::<u8>().unwrap();
        let move_color = input_args[2].parse::<u8>().unwrap();

        let result = game.make_move_on_board(move_x, move_y, move_color);
        println!("Result: {}", result);
        
        if game.is_game_over() {
            run_game_loop = false;
            println!("Game is finished");
            println!("Game winner: {}", game.game_winner());
        }
    }
}
