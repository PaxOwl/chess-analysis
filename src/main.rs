mod file_handling;
mod pretty_printers;
mod game;
mod statistics;
mod cluster;
// mod plot;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use colored::Colorize;


fn main() {
    let mut games_hashmap:HashMap<i32, cluster::Cluster> = HashMap::new();
    let mut current_game = game::Game::new();
    let mut game_count: i32 = 0;
    if let Ok(file) = File::open(
        // "./res/lichess_db_standard_rated_2016-02.pgn"
        "./res/reduced_data.pgn"
    ) {
        let reader = BufReader::new(file);

        let mut lines = reader.lines().map(|line| line.unwrap());
        let mut index = 0;
        while let Some(chunk) = file_handling::read_chunk(&mut lines) {
            // If the game is abandoned, skip it
            if chunk["Termination"] == "\"Abandoned\"" {
                continue;
            }

            // Time handling
            let mut time = 0;
            match chunk["TimeControl"].as_str() {
                "\"-\"" => {},
                _ => {
                    time = (&chunk["TimeControl"][1..]
                            .split('+')
                            .nth(0)
                            .unwrap())
                            .parse::<i32>()
                            .unwrap();
                }
            }
            current_game.set_time_control(time);

            // Elo handling
            match chunk["WhiteElo"].as_str() {
                "\"?\"" => {current_game.set_white_elo(-1)},
                _ => {
                    current_game
                        .set_white_elo((&chunk["WhiteElo"][1..&chunk["WhiteElo"].len() - 1])
                            .parse::<i32>()
                            .unwrap());
                }
            }
            match chunk["BlackElo"].as_str() {
                "\"?\"" => {current_game.set_black_elo(-1)},
                _ => {
                    current_game
                        .set_black_elo((&chunk["BlackElo"][1..&chunk["BlackElo"].len() - 1])
                            .parse::<i32>()
                            .unwrap());
                }
            }

            // Moves handling
            current_game
                .set_number_of_moves(file_handling::get_number_of_moves(&chunk["Moves"]));

            // Adding the current game to a cluster using HashMap
            games_hashmap.entry(time).or_insert_with(cluster::Cluster::new);
            games_hashmap.get_mut(&time).unwrap().add_game(current_game);
        }

        println!("\n{}", "Program exited normally".green().bold());
    }


    let mut time_keys: Vec<i32> = Vec::new();
    for (key, _) in &games_hashmap {
        time_keys.push(*key);
    }
    time_keys.sort();
    for key in time_keys {
        let stats: statistics::Statistics = statistics::Statistics::init(games_hashmap[&key].get_games());
        games_hashmap.get_mut(&key).map(|val| val.add_statistics(stats));
        println!("{} - {}", key, &games_hashmap[&key].get_statistics().get_avg());
    }
}
