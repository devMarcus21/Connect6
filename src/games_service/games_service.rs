use chashmap::CHashMap;
use crate::boards::boards::Connect6Board;
use crate::game::game_wrapper::{Game, Connect6Game};

pub trait GamesServicer<'r> {
    fn start_new_game(&mut self) -> u64;
}

pub struct GamesService<'r> {
    //games_data: &'r mut CHashMap<u64, &'r mut dyn Game<'r>>
    games_data: CHashMap<u64, &'r mut dyn Game<'r>>
}

impl<'r> GamesService<'r> {
    pub fn new(self) -> impl GamesServicer<'r> {
        let game_map_con: CHashMap<u64, &mut dyn Game<'r>> = CHashMap::new();

        GamesService{
            games_data: game_map_con // TODO check if this will mutate
        }
    }
}

impl<'r> GamesServicer<'r> for GamesService<'r> {
    fn start_new_game(&mut self) -> u64 {
        /*let mut board = Box::new(Connect6Board::new());
        //let mut game = Connect6Game::new(&mut board);
        let mut game = Connect6Game::new(board.as_mut());

        self.games_data.insert_new(1, &mut game);
        1*/
        0 // TODO implement
    }
}