use self::boards::boards::{GameBoard, Connect6Board};

pub mod boards;

pub fn new() -> Connect6Board {
    let board: Connect6Board = GameBoard::new();
    return board;
}