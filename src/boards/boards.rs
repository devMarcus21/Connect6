pub trait GameBoard {
    /// Sets corresponding x-y coordinate with the corresponding player color on the board
    fn make_move(&mut self, x: u8, y: u8, color: u8) -> bool;
    /// Returns true if it is the corresponding player color's turn, if not returns false
    fn is_player_turn(&self, color: u8) -> bool;
    /// Used to check if a move made won the game or not and if so will set the board state accordingly
    fn check_last_player_move_for_win(&self, x: u8, y: u8, color: u8) -> bool;
    /// Returns true if the game is over
    fn is_game_finished(&self) -> bool;
    /// Returns the player's color who won the game if it is finished. Returns 0 if game isn't over
    fn game_winner(&self) -> u8;
    /// Returns the color of the space at the x-y corrdinate if filled. If not filled returns 0 denoting
    /// an blank or empty space
    fn get_space_color(&self, x: u8, y: u8) -> u8;

    /// Returns the x size of the board
    fn x_size(&self) -> u8;
    /// Returns the y size of the board
    fn y_size(&self) -> u8;

    /// Used to print the board to output. Used when running games in the command line mode
    fn print_board(&self);
}

#[allow(dead_code)]
pub struct Connect6Board {
    board: [[u8; 19]; 19],
    game_finished: bool,
    game_won_by: u8,
    player_turn: u8,
    number_of_players: u8,
}

impl Connect6Board {
    pub fn new() -> impl GameBoard {
        Connect6Board {
            board:  [[0u8; 19]; 19],
            game_finished: false,
            game_won_by: 0,
            player_turn: 1,
            number_of_players: 2,
        }
    }

    /// Sets the turn to the next player's turn. Allows the next player to make a move,
    /// current player should not be able to make a move until its their turn again
    fn set_next_player_turn(&mut self) {
        self.player_turn = (self.player_turn+1) % (self.number_of_players + 1);
        if self.player_turn == 0 {
            self.player_turn = self.player_turn + 1;
        }
    }
}

impl GameBoard for Connect6Board {
    fn make_move(&mut self, x: u8, y: u8, color: u8) -> bool {
        self.board[y as usize][x as usize] = color;
        self.set_next_player_turn();

        true
    }

    fn is_player_turn(&self, color: u8) -> bool {
        self.player_turn == color
    }

    fn check_last_player_move_for_win(&self, x: u8, y: u8, color: u8) -> bool {
        false
    }

    fn is_game_finished(&self) -> bool {
        self.game_finished
    }

    fn game_winner(&self) -> u8 {
        self.game_won_by
    }

    fn get_space_color(&self, x: u8, y: u8) -> u8 {
        self.board[y as usize][x as usize]
    }

    fn x_size(&self) -> u8 {
        19
    }

    fn y_size(&self) -> u8 {
        19
    }

    fn print_board(&self) {
        for x in 0_u8 .. 19_u8 {
            print!("[");
            for y in 0_u8 .. 19_u8 {
                print!("{} ", self.board[x as usize][y as usize]);
            }
            println!("]");
        }
    }
}