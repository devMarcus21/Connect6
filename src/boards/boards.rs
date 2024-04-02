use std::cmp;

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

pub struct Connect6Board {
    board: [[u8; 19]; 19], // min size must be 19x19
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
        // Do not expect boards bigger than 127 by 127

        // Check points that are horizontal to place spot
        {
            let mut count_of_player_spaces_in_a_row: u8 = 0;
            let min_x: u8= 0;
            let max_x: u8 = self.x_size(); // non inclusive

            for x_point in min_x..max_x {
                if self.board[y as usize][x_point as usize] == color {
                    count_of_player_spaces_in_a_row += 1;
                    if count_of_player_spaces_in_a_row == 6 {
                        return true;
                    }
                } else {
                    count_of_player_spaces_in_a_row = 0;
                }
            }
        }

        // Check points that are vertical to place spot
        {
            let mut count_of_player_spaces_in_a_row: u8 = 0;
            let min_y: u8 = 0;
            let max_y: u8 = self.y_size(); // non inclusive

            for y_point in min_y..max_y {
                if self.board[y_point as usize][x as usize] == color {
                    count_of_player_spaces_in_a_row += 1;
                    if count_of_player_spaces_in_a_row == 6 {
                        return true;
                    }
                } else {
                    count_of_player_spaces_in_a_row = 0;
                }
            }
        }

        // Check points that are diagonal top left to bottom right from place spot
        {
            let mut count_of_player_spaces_in_a_row: u8 = 0;
            let diff_to_top_left_edge: u8 = cmp::min(x, y);

            let mut top_left_x: u8 = x - diff_to_top_left_edge;
            let mut top_left_y: u8 = y - diff_to_top_left_edge;

            while top_left_x < self.x_size() && top_left_y < self.y_size() {
                if self.board[top_left_y as usize][top_left_x as usize] == color {
                    count_of_player_spaces_in_a_row += 1;
                    if count_of_player_spaces_in_a_row == 6 {
                        return true;
                    }
                } else {
                    count_of_player_spaces_in_a_row = 0;
                }

                top_left_x += 1;
                top_left_y += 1;
            }
        }

        // Check points that are diagonal bottom left to top right from place spot
        {
            let mut count_of_player_spaces_in_a_row: u8 = 0;
            let diff_to_bottom_left_edge: u8 = cmp::min(x, self.y_size() - y - 1);

            let mut bottom_left_x: u8 = x - diff_to_bottom_left_edge;
            let mut bottom_left_y: i8 = (y + diff_to_bottom_left_edge) as i8; // Allows for checking 0th y index

            while bottom_left_x < self.x_size() && bottom_left_y >= 0 {
                if self.board[bottom_left_y as usize][bottom_left_x as usize] == color {
                    count_of_player_spaces_in_a_row += 1;
                    if count_of_player_spaces_in_a_row == 6 {
                        return true;
                    }
                } else {
                    count_of_player_spaces_in_a_row = 0;
                }

                bottom_left_x += 1;
                bottom_left_y -= 1;
            }
        }

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