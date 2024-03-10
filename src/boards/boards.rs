pub mod boards {
    pub trait GameBoard {
        fn new() -> Self;

        fn make_move(&self, x: u8, y: u8, color: u8) -> bool;
        fn is_player_turn(&self, color: u8) -> bool;
        fn check_last_player_move_for_win(&self, x: u8, y: u8, color: u8) -> bool;
        fn is_game_over(&self) -> bool;
        fn game_winner(&self) -> u8;

        fn x_size(&self) -> u8;
        fn y_size(&self) -> u8;
    }

    #[allow(dead_code)]
    pub struct Connect6Board {
        board: [[u8; 19]; 19],
        game_over: bool,
        game_won_by: u8,
        player_turn: u8,
    }

    impl GameBoard for Connect6Board {
        fn new () -> Connect6Board {
            Connect6Board { board:  [[0; 19]; 19], game_over: false, game_won_by: 0, player_turn: 1 }
        }

        #[allow(unused_variables)]
        fn make_move(&self, x: u8, y: u8, color: u8) -> bool {
            return false;
        }

        fn is_player_turn(&self, color: u8) -> bool {
            return self.player_turn == color;
        }

        #[allow(unused_variables)]
        fn check_last_player_move_for_win(&self, x: u8, y: u8, color: u8) -> bool {
            return false;
        }

        fn is_game_over(&self) -> bool {
            return self.game_over;
        }

        fn game_winner(&self) -> u8 {
            return self.game_won_by;
        }

        fn x_size(&self) -> u8 {
            return 19;
        }

        fn y_size(&self) -> u8 {
            return 19;
        }
    }
}