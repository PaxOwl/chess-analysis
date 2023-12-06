use crate::game;
use crate::statistics;


#[derive(Clone)]
pub struct Cluster {
    games: Vec<game::Game>,
    stats: statistics::Statistics,
}

impl Cluster {
    pub fn new() -> Self {
        Cluster {
            games: vec![],
            stats: statistics::Statistics::new(),
        }
    }

    pub fn add_game(&mut self, game: game::Game) {
        self.games.push(game)
    }

    pub fn add_statistics(&mut self, stats: statistics::Statistics) {
        self.stats = stats
    }

    pub fn get_games(&self) -> &Vec<game::Game> {
        &self.games
    }

    pub fn get_statistics(&self) -> &statistics::Statistics {
        &self.stats
    }
}
