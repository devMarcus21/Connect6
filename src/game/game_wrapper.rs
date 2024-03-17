use crate::boards::boards::GameBoard;
use crate::game::result_states::MoveResults;

pub trait Game<'r> {
    fn new(board: &'r mut dyn GameBoard) -> Self;
    fn is_game_over(&'r self) -> bool;
    fn make_move_on_board(&mut self, x: u8, y: u8, color: u8) -> MoveResults;
    fn game_winner(self) -> u8;
}

pub struct Connect6Game<'r> {
    board: &'r mut dyn GameBoard,
}

impl<'r> Game<'r> for Connect6Game<'r> {
    fn new(game_board: &'r mut dyn GameBoard) -> Self {
        Self {
            board: game_board,
        }
    }

    fn is_game_over(&'r self) -> bool {
        self.board.is_game_finished()
    }

    fn make_move_on_board(&mut self, x: u8, y: u8, color: u8) -> MoveResults {
        self.board.make_move(x, y, color);
        MoveResults::GameIsOver
    }

    fn game_winner(self) -> u8 {
        self.board.game_winner()
    }
}