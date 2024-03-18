mod boards;
mod game;

#[allow(unused_imports)]
use crate::boards::boards::{Connect6Board, GameBoard};
use crate::game::result_states::MoveResults;
use crate::game::game_wrapper::{Connect6Game, Game};

fn main() {
    let mut board = boards::boards::Connect6Board::new();

    let game = Connect6Game::new(&mut board);

    println!("{}", game.is_game_over());
    println!("END");
}
