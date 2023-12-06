mod file_handling;
mod pretty_printers;
mod game;
mod statistics;
mod cluster;
// mod plot;

use crate::pretty_printers::table_printer;
use std::collections::HashMap;


fn main() {
    let mut games_hashmap:HashMap<i32, cluster::Cluster> = HashMap::new();
    let mut time: i32 = 0;
    let mut current_game = game::Game::new();
    let mut game_count: i32 = 0;
    if let Ok(lines) = file_handling::read_lines(
        // "./res/lichess_db_standard_rated_2016-02.pgn"
        "./res/reduced_data.pgn"
    ) {
        for line in lines {
            if let Ok(line_content) = line {
                // println!("{}", line_content);
                // Lecture du temps des joueurs pour la partie en cours de traitement
                let first_word = line_content.as_str().split_whitespace().nth(0);
                let second_word = line_content.as_str().split_whitespace().nth(1);
                match first_word {
                    Some("[TimeControl") => {
                        time = 0;
                        match second_word {
                            Some("\"-\"]") => {},
                            _ => {
                                time = file_handling::get_time_control(&line_content);
                                current_game.set_time_control(time);
                            }
                        }
                    },
                    Some("[WhiteElo") => current_game
                        .set_white_elo(file_handling::get_elo(&line_content)),
                    Some("[BlackElo") => current_game
                        .set_black_elo(file_handling::get_elo(&line_content)),
                    _ => {}
                }
                // Assignation d'un identificateur pour la partie en cours de traitement
                current_game.set_id(game_count);
                match line_content.starts_with('1') {
                    true => {
                        current_game.set_number_of_moves(file_handling::get_number_of_moves(&line_content));
                        match current_game.get_number_of_moves() {
                            1 => continue,
                            _ => {}
                        }
                        game_count += 1;
                        current_game.set_id(game_count);
                        // current_game.print();
                        // println!();

                        // Fin de la création de la partie ici, ajouter une manière de remplir
                        // la HashMap de Cluster
                        games_hashmap.entry(time).or_insert_with(cluster::Cluster::new);
                        games_hashmap.get_mut(&time).unwrap().add_game(current_game);
                    }
                    false => {}
                }
            }
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
}
