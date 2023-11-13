mod file_handling;
mod pretty_printers;
mod game;
mod statistics;
mod cluster;
// mod plot;

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
                // Lecture du temps des joueurs pour la partie en cours de traitement
                if line_content.contains("TimeControl") {
                    time = 0;
                    if !line_content.contains('-') {
                        time = file_handling::get_time_control(&line_content);
                        current_game.set_time_control(time);
                    }
                }
                // Lecture de l'elo des blancs pour la partie en cours de traitement
                else if line_content.contains("WhiteElo") {
                    current_game.set_white_elo(file_handling::get_elo(&line_content));
                }
                // Lecture de l'elo des noirs pour la partie en cours de traitement
                else if line_content.contains("BlackElo") {
                    current_game.set_black_elo(file_handling::get_elo(&line_content));
                }
                // Assignation d'un identificateur pour la partie en cours de traitement
                current_game.set_id(game_count);
                if line_content.starts_with('1') {
                    current_game.set_number_of_moves(file_handling::get_number_of_moves(&line_content));
                    if current_game.get_number_of_moves() == 1 {
                        continue;
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
            }
        }
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
