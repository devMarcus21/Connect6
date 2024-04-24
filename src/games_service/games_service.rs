use rand::Rng;
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
            games_data: game_map_con
        }
    }
}

impl GamesServicer for GamesService {
    fn start_new_game(&mut self) -> u64 {
        let board = Box::new(Connect6Board::new());
        let game = Box::new(Connect6Game::new(board));

        // Generate random number for the game id
        let mut rng = rand::thread_rng();
        let game_id: u64 = rng.gen();
        self.games_data.insert(game_id, game);

        game_id
    }
}