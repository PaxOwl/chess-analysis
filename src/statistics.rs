use core::cmp::Ordering;
use crate::game;

#[derive(Clone)]
pub struct Statistics {
    avg: f32,
    median: i32,
    var: f32,
    std: f32,
    min: i32,
    max: i32,
}

impl Statistics {

    pub fn new() -> Self {
        Statistics {
            avg: 0.0,
            median: 0,
            var: 0.0,
            std: 0.0,
            min: 0,
            max: 0,
        }
    }

    pub fn init(games: &Vec<game::Game>) -> Self {
        let mut avg: f32 = 0.;
        let mut var: f32 = 0.;
        let mut min: i32 = 10000;
        let mut max: i32 = 0;
        let mut moves_list: Vec<i32> = Vec::new();

        for game in games {
            let moves: i32 = game.get_number_of_moves();
            avg += moves as f32;
            match min.cmp(&moves) {
                Ordering::Less => {},
                Ordering::Greater => { min = moves },
                Ordering::Equal => {},
            }
            match max.cmp(&moves) {
                Ordering::Less => { max = moves },
                Ordering::Greater => {},
                Ordering::Equal => {},
            }
            moves_list.push(moves);
        }
        avg /= games.len() as f32;
        for game in games {
            var += (game.get_number_of_moves() as f32 - avg).powf(2.);
        }
        var /= games.len() as f32;
        let std: f32 = var.sqrt();
        moves_list.sort();
        let median = moves_list[moves_list.len() / 2];

        Statistics {
            avg,
            median,
            var,
            std,
            min,
            max,
        }
    }

    pub fn get_avg(&self) -> f32{
        self.avg
    }

    pub fn get_median(&self) -> i32{
        self.median
    }

    pub fn get_var(&self) -> f32{
        self.var
    }

    pub fn get_std(&self) -> f32{
        self.std
    }

    pub fn get_min(&self) -> i32{
        self.min
    }

    pub fn get_max(&self) -> i32{
        self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics() {
        let mut games: Vec<game::Game> = Vec::new();
        
        for i in 2..10 {
            let mut current_game: game::Game = game::Game::new();
            current_game.set_number_of_moves(2 * i);
            games.push(current_game);
        }

        let stat: Statistics = Statistics::new();
        stat.init(&games);
    }
}
