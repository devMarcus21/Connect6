pub mod boards;

#[allow(unused_imports)]
use crate::boards::boards::{Connect6Board, GameBoard, new_connect6_game_board};

pub fn new_board() -> Box<dyn GameBoard> {
    Box::new(new_connect6_game_board())
}