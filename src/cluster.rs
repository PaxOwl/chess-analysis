use crate::game;


pub struct Cluster {
    games: Vec<game::Game>,
    mean: i32,
    median: i32,
    variance: f32,
    std: f32,
    min: i32,
    max: i32,
}

impl Cluster {
    pub fn new() -> Self {
        Cluster {
            games: vec![],
            mean: -1,
            median: -1,
            variance: -1.,
            std: -1.0,
            min: -1,
            max: -1,
        }
    }

    // pub fn add_game(&mut self, game: game::Game) {
    //     self.games.push(game)
    // }

    // pub fn get_games(&self) -> &Vec<game::Game> {
    //     &self.games
    // }

    pub fn get_mean(&mut self) -> &i32 {
        match &self.mean {
            -1 => {
                let mut mean: i32 = 0;
                for game in &self.games {
                    mean += game.get_number_of_moves();
                }
                mean /= self.games.len() as i32;
                self.mean = mean;
            },
            &_ => {}
        }
        &self.mean
    }
}
