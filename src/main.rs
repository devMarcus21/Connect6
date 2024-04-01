mod boards;
mod game;

// Keeping these commented so I remember
// Also so that the compiler stops giving warnings without using unused
// use crate::boards::boards::{Connect6Board, GameBoard};
// use crate::game::result_states::MoveResults;
use crate::game::game_wrapper::{Connect6Game, Game};

fn clean_input_string(input: &mut String) {
    *input = input.replace("\r", "");
    *input = input.replace("\n", "");
}

fn valid_move_input_args(args: &Vec<&str>) -> bool {
    args[0].parse::<u8>().is_ok() && args[1].parse::<u8>().is_ok() && args[2].parse::<u8>().is_ok()
}

fn run_command_line_version() {
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

struct Move {
    x: u8,
    y: u8,
    color: u8,
}

fn test_runner(moves: Vec<Move>) {
    let mut board = boards::boards::Connect6Board::new();
    let mut game = Connect6Game::new(&mut board);
    for board_move in moves.iter() {
        println!("{} {} {} {}", game.make_move_on_board(board_move.x, board_move.y, board_move.color), board_move.x, board_move.y, board_move.color);
    }
    game.print();
}

fn make_move(x: u8, y: u8, color: u8) -> Move {
    Move {
        x,
        y,
        color
    }
}

fn main() {
    //run_command_line_version()
    /*{
        let moves = vec![
            make_move(0, 0, 1),
            make_move(0, 1, 2),
            make_move(1, 0, 1),
            make_move(0, 2, 2),
            make_move(2, 0, 1),
            make_move(0, 3, 2),
            make_move(3, 0, 1),
            make_move(0, 4, 2),
            make_move(4, 0, 1),
            make_move(0, 5, 2),
            make_move(5, 0, 1)];

        test_runner(moves);
    }
    {
        let moves = vec![
            make_move(1, 0, 1),
            make_move(0, 1, 2),
            make_move(2, 0, 1),
            make_move(0, 2, 2),
            make_move(3, 0, 1),
            make_move(0, 3, 2),
            make_move(4, 0, 1),
            make_move(0, 4, 2),
            make_move(5, 0, 1),
            make_move(0, 5, 2),
            make_move(6, 0, 1)
        ];
        test_runner(moves);
    }
    {
        let moves = vec![
            make_move(1, 7, 1),
            make_move(0, 1, 2),
            make_move(2, 7, 1),
            make_move(0, 2, 2),
            make_move(3, 7, 1),
            make_move(0, 3, 2),
            make_move(4, 7, 1),
            make_move(0, 4, 2),
            make_move(5, 7, 1),
            make_move(0, 5, 2),
            make_move(6, 7, 1)
        ];
        test_runner(moves);
    }
    {
        let moves = vec![
            make_move(0, 0, 1),
            make_move(1, 1, 2),
            make_move(0, 1, 1),
            make_move(1, 2, 2),
            make_move(0, 2, 1),
            make_move(1, 3, 2),
            make_move(0, 3, 1),
            make_move(1, 4, 2),
            make_move(0, 4, 1),
            make_move(1, 5, 2),
            make_move(0, 5, 1)];

        test_runner(moves);
    }
    {
        let moves = vec![
            make_move(5, 0, 1),
            make_move(1, 1, 2),
            make_move(5, 1, 1),
            make_move(1, 2, 2),
            make_move(5, 2, 1),
            make_move(1, 3, 2),
            make_move(5, 3, 1),
            make_move(1, 4, 2),
            make_move(5, 4, 1),
            make_move(1, 5, 2),
            make_move(5, 5, 1)];

        test_runner(moves);
    }
    {
        let moves = vec![
            make_move(7, 7, 1),
            make_move(1, 1, 2),
            make_move(7, 8, 1),
            make_move(1, 2, 2),
            make_move(7, 9, 1),
            make_move(1, 3, 2),
            make_move(7, 10, 1),
            make_move(1, 4, 2),
            make_move(7, 11, 1),
            make_move(1, 5, 2),
            make_move(7, 12, 1)];

        test_runner(moves);
    }
    {
        let moves = vec![
            make_move(0, 18, 1),
            make_move(1, 1, 2),
            make_move(0, 17, 1),
            make_move(1, 2, 2),
            make_move(0, 16, 1),
            make_move(1, 3, 2),
            make_move(0, 15, 1),
            make_move(1, 4, 2),
            make_move(0, 14, 1),
            make_move(1, 5, 2),
            make_move(7, 7, 1)];

        test_runner(moves);
    }*/
    {
        let moves = vec![
            make_move(0, 0, 1),
            make_move(0, 1, 2),
            make_move(1, 1, 1),
            make_move(0, 2, 2),
            make_move(2, 2, 1),
            make_move(0, 3, 2),
            make_move(3, 3, 1),
            make_move(0, 4, 2),
            make_move(4, 4, 1),
            make_move(0, 5, 2),
            make_move(5, 5, 1)];

        test_runner(moves);
    }
    {
        let moves = vec![
            make_move(18, 18, 1),
            make_move(0, 1, 2),
            make_move(17, 17, 1),
            make_move(0, 2, 2),
            make_move(16, 16, 1),
            make_move(0, 3, 2),
            make_move(15, 15, 1),
            make_move(0, 4, 2),
            make_move(14, 14, 1),
            make_move(0, 5, 2),
            make_move(13, 13, 1)];

        test_runner(moves);
    }
    {
        let moves = vec![
            make_move(10, 10, 1),
            make_move(0, 1, 2),
            make_move(11, 11, 1),
            make_move(0, 2, 2),
            make_move(12, 12, 1),
            make_move(0, 3, 2),
            make_move(13, 13, 1),
            make_move(0, 4, 2),
            make_move(14, 14, 1),
            make_move(0, 5, 2),
            make_move(15, 15, 1)];

        test_runner(moves);
    }
}
