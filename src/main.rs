mod file_handling;
mod pretty_printers;
mod game;
mod statistics;
// mod plot;

use std::collections::HashMap;

fn hashmap_to_sorted_vector(hash: &HashMap<i32, Vec<i32>>) -> Vec<(i32, Vec<i32>)> {
    // Convert the keys of the HashMap to a Vector
    let mut hash_vec:Vec<i32> = Vec::new();
    for (&key, _) in hash {
        hash_vec.push(key);
    }

    // Sort the keys Vector
    hash_vec.sort_unstable();

    // Create a sorted Vector and assing the Vectors of values back to their key respective keys
    let mut sorted_vec = Vec::new();
    for index in &hash_vec {
        sorted_vec.push((*index, hash[index].clone()));
    }
    sorted_vec
}


fn main() {
    let mut game_timers:HashMap<i32, Vec<i32>> = HashMap::new();
    let mut time: i32 = 0;
    let mut games: Vec<game::Game> = Vec::new();
    let mut current_game = game::Game::new();
    let mut game_count: i32 = 0;
    let mut statistics: Vec<statistics::Statistics> = Vec::new();
    if let Ok(lines) = file_handling::read_lines(
        // "./res/lichess_db_standard_rated_2016-02.pgn"
        "./res/reduced_data.pgn"
    ) {
        for line in lines {
            if let Ok(line_content) = line {
                // Lecture du temps des joueurs pour la partie en cours de traitement
                if line_content.contains("TimeControl") {
                    if line_content.contains('-') {
                        time = 0;
                    }
                    else {
                        current_game.set_time_control(file_handling::get_time_control(&line_content));
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
                    game_timers.entry(time).or_insert_with(Vec::new);
                    game_timers
                        .get_mut(&time)
                        .expect("REASON")
                        .push(current_game.get_number_of_moves());
                    game_count += 1;
                    current_game.set_id(game_count);
                    games.push(current_game);
                    current_game.print();
                    println!();
                }
            }
        }
    }
}
