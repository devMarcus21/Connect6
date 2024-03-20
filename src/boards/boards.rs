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

struct BoardPoint {
    x: u8,
    y: u8,
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

    fn vector_of_points_contain_win(&self, points_to_check: Vec<BoardPoint>, player_color: u8) -> bool {
        let mut count: u8 = 0;
        for point in points_to_check.iter() {
            if self.board[point.y as usize][point.x as usize] == player_color {
                count += 1;
                if count == 6 {
                    return true;
                }
            } else {
                count = 0;
            }
        }

        false
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

    fn check_last_player_move_for_win(&self, x: u8, y: u8, color: u8) -> bool { // TODO investigate this logic more
        // Do not expect boards bigger than 127 by 127
        let x_signed = x as i8;
        let y_signed = y as i8;

        // Check points that are horizontal to place spot
        {
            let mut horizontal_points: Vec<BoardPoint> = Vec::new();
            let min_x: u8= 0;
            let max_x: u8 = self.x_size(); // non inclusive

            for x_point in min_x..max_x {
                horizontal_points.push(BoardPoint{
                    x: x_point,
                    y,
                });
            }

            if self.vector_of_points_contain_win(horizontal_points, color) {
                return true;
            }
        }
        // Check points that are vertical to place spot
        {
            let mut vertical_points: Vec<BoardPoint> = Vec::new();
            let min_y: u8 = 0;
            let max_y: u8 = self.y_size(); // non inclusive

            for y_point in min_y..max_y {
                vertical_points.push(BoardPoint{
                    x,
                    y: y_point,
                });
            }

            if self.vector_of_points_contain_win(vertical_points, color) {
                return true;
            }
        }
        // Check points top left to bottom right diagonal
        { // Fails to work on diagonal left up to bottom right
            let mut diagonal_top_left_to_bottom_right_points: Vec<BoardPoint> = Vec::new();
            let top_left_x: i8 = x_signed - 6;
            let top_left_y: i8 = y_signed - 6;

            for step in 0_i8..5_i8 {
                if (top_left_x + step >= 0) && (top_left_y + step >= 0) {
                    diagonal_top_left_to_bottom_right_points.push(BoardPoint{
                        x: (top_left_x + step) as u8,
                        y: (top_left_y + step) as u8,
                    });
                }
            }
            diagonal_top_left_to_bottom_right_points.push(BoardPoint{ x, y });
            for step in 1_i8..7_i8 {
                if (x + (step as u8) >= self.x_size()) || (y + (step as u8) >= self.y_size()) {
                    break;
                }
                diagonal_top_left_to_bottom_right_points.push(BoardPoint{
                    x: x + (step as u8),
                    y: y + (step as u8) ,
                });
            }

            if self.vector_of_points_contain_win(diagonal_top_left_to_bottom_right_points, color) {
                return true;
            }
        }
        // Check points bottom left to top right diagonal
        {
            let mut diagonal_bottom_left_to_top_right_points: Vec<BoardPoint> = Vec::new();
            let bottom_left_x: i8 = x_signed - 6;
            let bottom_left_y: i8 = y_signed + 6;

            for step in 0_i8..5_i8 {
                if (bottom_left_x + step >= 0) && (bottom_left_y - step < self.y_size() as i8) {
                    diagonal_bottom_left_to_top_right_points.push(BoardPoint{
                        x: (bottom_left_x + step) as u8,
                        y: (bottom_left_y - step) as u8,
                    });
                }
            }
            diagonal_bottom_left_to_top_right_points.push(BoardPoint{ x, y });
            for step in 1_i8..7_i8 {
                if (x + (step as u8) >= self.x_size()) || (y < step as u8) {//(y - (step as u8) < 0) {
                    break;
                }
                diagonal_bottom_left_to_top_right_points.push(BoardPoint{
                    x: x + (step as u8),
                    y: y - (step as u8) ,
                });
            }

            if self.vector_of_points_contain_win(diagonal_bottom_left_to_top_right_points, color) {
                return true;
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