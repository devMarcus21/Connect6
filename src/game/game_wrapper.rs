use crate::boards::boards::GameBoard;
use crate::game::result_states::MoveResults;

pub trait Game {
    fn is_game_over(&self) -> bool;
    fn make_move_on_board(&mut self, x: u8, y: u8, color: u8) -> MoveResults;
    fn game_winner(&self) -> u8;

    // For playing with CLI
    fn print(&self);
}

pub struct Connect6Game {
    board: Box<dyn GameBoard>,
}

impl Connect6Game {
    pub fn new(game_board: Box<dyn GameBoard>) -> Self {
        Self {
            board: game_board,
        }
    }

    fn is_move_inbounds(&self, x: u8, y: u8) -> bool {
        // Don't check for negative values since unsigned ints
        x < self.board.x_size() || y < self.board.y_size()
    }
}

impl Game for Connect6Game {
    // TODO still makes move even when game is over in some cases
    fn is_game_over(&self) -> bool {
        self.board.is_game_finished()
    }

    fn make_move_on_board(&mut self, x: u8, y: u8, color: u8) -> MoveResults {
        if self.is_game_over() {
            return MoveResults::GameIsFinished;
        }

        if color == 0 {
            return MoveResults::MoveWithInvalidColor;
        }
        if !self.board.is_player_turn(color) {
            return MoveResults::NotPlayerTurn;
        }

        if !self.is_move_inbounds(x, y) {
            return MoveResults::MoveOutOfBounds;
        }

        // Check that there's not a piece already on the spot
        let space_at_coordinate_color = self.board.get_space_color(x, y);
        if space_at_coordinate_color == color {
            return MoveResults::SpaceIsAlreadyPlacedByPlayer;
        }
        if space_at_coordinate_color != 0 {
            return MoveResults::SpaceIsAlreadyPlacedByOponent;
        }

        self.board.make_move(x, y, color);
        // Check to see if this move won the game or not
        let did_last_move_win = self.board.check_last_player_move_for_win(x, y, color);
        if did_last_move_win {
            return MoveResults::MoveMadeAndGameWon;
        }
        MoveResults::MoveMade
    }

    fn game_winner(&self) -> u8 {
        self.board.game_winner()
    }

    fn print(&self) {
        self.board.print_board()
    }
}