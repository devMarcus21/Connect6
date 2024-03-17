use crate::boards::boards::GameBoard;
use crate::game::result_states::MoveResults;

pub trait Game<'r> {
    fn is_game_over(&'r self) -> bool;
    fn make_move_on_board(&mut self, x: u8, y: u8, color: u8) -> MoveResults;
    fn game_winner(self) -> u8;
}

pub struct Connect6Game<'r> {
    board: &'r mut dyn GameBoard,
    //board: &'r dyn GameBoard,
    //board: Box<dyn GameBoard + 'r>,
}

impl<'r> Game<'r> for Connect6Game<'r> {
    fn is_game_over(&'r self) -> bool {
        //let board: &dyn GameBoard = self.board;
        //board.is_game_finished()
        //let board: &dyn GameBoard = self.board;
        //board.is_game_finished()
        self.board.is_game_finished()
    }

    fn make_move_on_board(&mut self, x: u8, y: u8, color: u8) -> MoveResults {
        MoveResults::GameIsOver
    }

    fn game_winner(self) -> u8 {
        0
    }
}