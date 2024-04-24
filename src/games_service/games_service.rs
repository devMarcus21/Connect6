use std::collections::HashMap;
use crate::boards::boards::Connect6Board;
use crate::game::game_wrapper::{Game, Connect6Game};

pub trait GamesServicer {
    fn start_new_game(&mut self) -> u64;
}

pub struct GamesService {
    games_data: HashMap<u64, Box<dyn Game>>
}

impl GamesService {
    pub fn new() -> Self {
        let game_map_con: HashMap<u64, Box<dyn Game>> = HashMap::new();

        Self {
            games_data: game_map_con // TODO check if this will mutate
        }
    }
}

impl GamesServicer for GamesService {
    fn start_new_game(&mut self) -> u64 {
        let board = Box::new(Connect6Board::new());
        let game = Box::new(Connect6Game::new(board));
        self.games_data.insert(1, game);

        1 // TODO implement
    }
}