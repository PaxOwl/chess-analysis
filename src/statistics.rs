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

    pub fn new(games: &Vec<game::Game>) -> Self {
        let mut avg: f32 = 0.;
        let mut var: f32 = 0.;
        let mut min: i32 = 10000;
        let mut max: i32 = 0;
        let mut moves_list: Vec<i32> = Vec::new();

        for game in games {
            let moves: i32 = game.get_number_of_moves();
            avg += moves as f32;
            if max < moves {
                max = moves;
            }
            if min > moves {
                min = moves;
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
}
