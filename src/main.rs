mod boards;
mod game;

#[allow(unused_imports)]
use crate::boards::boards::{Connect6Board, GameBoard};
use crate::game::result_states::MoveResults;
use crate::game::game_wrapper::{Connect6Game, Game};

fn make_move(game: &mut dyn Game, x: u8, y: u8, color: u8) {
    println!("{}", game.is_game_over());
    println!("{}", game.game_winner());

    let result = game.make_move_on_board(x, y, color);
    println!("{}", result);

    println!("{}", game.is_game_over());
    println!("{}", game.game_winner());

    println!();
}

fn main() {
    let mut board = boards::boards::Connect6Board::new();
    let mut game = Connect6Game::new(&mut board);

    make_move(&mut game, 8_u8, 7_u8, 1_u8);

    game.print();
    println!();

    make_move(&mut game, 8_u8, 7_u8, 1_u8);
    make_move(&mut game, 8_u8, 7_u8, 2_u8);

    println!("END");
}
