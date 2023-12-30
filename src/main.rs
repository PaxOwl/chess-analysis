mod file_handling;
mod pretty_printers;
mod game;
mod statistics;
mod cluster;
// mod plot;

use crate::pretty_printers::table_printer;
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

            current_game.set_id(game_count);

            // Adding the current game to a cluster using HashMap
            games_hashmap.entry(time).or_insert_with(cluster::Cluster::new);
            games_hashmap.get_mut(&time).unwrap().add_game(current_game);

            game_count += 1;
        }
    }


    let mut time_keys: Vec<i32> = Vec::new();
    for (key, _) in &games_hashmap {
        time_keys.push(*key);
    }
    time_keys.sort();
    let mut headers: Vec<String> = Vec::new();
    let mut data: Vec<Vec<f32>> = Vec::new();
    headers.push("Time".to_string());
    headers.push("Games".to_string());
    headers.push("Percentage".to_string());
    headers.push("Average".to_string());
    headers.push("Median".to_string());
    headers.push("Variance".to_string());
    headers.push("Standard Deviation".to_string());
    headers.push("Minimum".to_string());
    headers.push("Maximum".to_string());
    for key in &time_keys {
        let stats: statistics::Statistics = statistics::Statistics::init(games_hashmap[&key].get_games());
        games_hashmap.get_mut(&key).map(|val| val.add_statistics(stats));
        let mut data_row: Vec<f32> = Vec::new();
        let percentage: f32 = games_hashmap[&key].get_games().len() as f32 / game_count as f32 * 100.;
        data_row.push(*key as f32);
        data_row.push(games_hashmap[&key].get_games().len() as f32);
        data_row.push(percentage);
        data_row.push(games_hashmap[&key].get_statistics().get_avg());
        data_row.push(games_hashmap[&key].get_statistics().get_median() as f32);
        data_row.push(games_hashmap[&key].get_statistics().get_var());
        data_row.push(games_hashmap[&key].get_statistics().get_std());
        data_row.push(games_hashmap[&key].get_statistics().get_min() as f32);
        data_row.push(games_hashmap[&key].get_statistics().get_max() as f32);
        data.push(data_row);
        // println!("Time : {:>5} - Games : {:>7} - avg : {:>5.2} - med : {:>2} - var : {:>6.2} - std : {:>5.2} - min : {:>2} - max : {:>3}",
        //     key,
        //     &games_hashmap[&key].get_games().len(),
        //     &games_hashmap[&key].get_statistics().get_avg(),
        //     &games_hashmap[&key].get_statistics().get_median(),
        //     &games_hashmap[&key].get_statistics().get_var(),
        //     &games_hashmap[&key].get_statistics().get_std(),
        //     &games_hashmap[&key].get_statistics().get_min(),
        //     &games_hashmap[&key].get_statistics().get_max());
    }
    table_printer(headers, data);

    println!("\n{}", "Program exited normally".green().bold());
}
