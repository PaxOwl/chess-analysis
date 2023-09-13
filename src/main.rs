mod file_handling;
mod pretty_printers;
mod game;
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
    let mut current_game = game::Game {
        id: 0,
        white_elo: 0,
        black_elo: 0,
        time_control: 0,
        number_of_moves: 0,
    };
    let mut game_count: i32 = 0;
    if let Ok(lines) = file_handling::read_lines(
        // "./res/lichess_db_standard_rated_2016-02.pgn"
        "./res/reduced_data.pgn"
    ) {
        for line in lines {
            if let Ok(line_content) = line {
                if line_content.contains("TimeControl") {
                    if line_content.contains('-') {
                        time = 0;
                    }
                    else {
                        current_game.set_time_control(file_handling::get_time_control(&line_content));
                    }
                }
                else if line_content.contains("WhiteElo") {
                    current_game.set_white_elo(file_handling::get_elo(&line_content));
                }
                else if line_content.contains("BlackElo") {
                    current_game.set_black_elo(file_handling::get_elo(&line_content));
                    // println!("{}", &line_content);
                }
                current_game.set_id(game_count);
                if line_content.starts_with('1') {
                    current_game.set_number_of_moves(file_handling::get_number_of_moves(&line_content));
                    if current_game.get_number_of_moves() == 1 {
                        continue;
                    }
                    game_timers.entry(time).or_insert_with(Vec::new);
                    game_timers.get_mut(&time).expect("REASON").push(current_game.number_of_moves);
                    game_count += 1;
                    current_game.set_id(game_count);
                    games.push(current_game);
                    current_game.print();
                    println!();
                }
            }
        }
    }

    // let time_moves:Vec<(i32, Vec<i32>)> = hashmap_to_sorted_vector(&game_timers);
    // // plot::time_histogram(&time_moves);
    // let mut table_data: Vec<Vec<i32>> = Vec::new();
    // let mut headers: Vec<String> = Vec::new();
    // for (time, mut moves) in time_moves {
    //     let mut avg: i32 = 0;
    //     let mut var: i32 = 0;
    //     let mut min: i32 = 10000;
    //     let mut max: i32 = 0;
    //     for i in &moves {
    //         avg += *i;
    //         if max < *i {
    //             max = *i;
    //         }
    //         if min > *i {
    //             min = *i;
    //         }
    //     }
    //     avg /= moves.len() as i32;
    //     for i in &moves {
    //         var += (i - avg).pow(2);
    //     }
    //     var /= moves.len() as i32;
    //     let std: i32 = (var as f32).sqrt() as i32;
    //     moves.sort();
    //     let median = moves[moves.len() / 2];
    //
    //     let mut temp_vec: Vec<i32> = Vec::new();
    //     temp_vec.push(time);
    //     temp_vec.push(moves.len() as i32);
    //     temp_vec.push(avg);
    //     temp_vec.push(median);
    //     temp_vec.push(var);
    //     temp_vec.push(std);
    //     temp_vec.push(min);
    //     temp_vec.push(max);
    //     table_data.push(temp_vec);
    // }
    // headers.push(String::from("Time (s)"));
    // headers.push(String::from("Games"));
    // headers.push(String::from("Avg"));
    // headers.push(String::from("Med"));
    // headers.push(String::from("Var"));
    // headers.push(String::from("Std"));
    // headers.push(String::from("Min"));
    // headers.push(String::from("Max"));
    // pretty_printers::table_printer(headers, table_data);

}
