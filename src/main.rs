mod boards;
mod game;

#[allow(unused_imports)]
use crate::boards::boards::{Connect6Board, GameBoard};
use crate::boards::new_board;
use crate::game::result_states::MoveResults;

fn main() {
    let _board = new_board();
}
