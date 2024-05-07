use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod file_handling;
mod pretty_printers;
mod game_old;
mod statistics;
mod cluster;

mod board;
mod game;


pub fn run_analysis(file: &str) {
    // let current_game: Option<HashMap<String, String>> = file_handling::load_chess_game(file);
    // for element in current_game.expect("Error during process of the game") {
    //     println!("{}: {}", element.0, element.1);
    // }

    let mut board = board::Board::new();
    board.print();
    // game::play(board, current_game.expect("Error during process of the game"));
    println!();
    board.print();
}

pub fn run_statistics(file: &str) {
    let mut games_hashmap = load_data(file);

    let mut time_keys: Vec<i32> = Vec::new();
    for (key, _) in &games_hashmap {
        time_keys.push(*key);
    }
    time_keys.sort();
    let mut headers: Vec<String> = Vec::new();
    let mut data: Vec<Vec<f32>> = Vec::new();
    headers.push("Time".to_string());
    headers.push("Games".to_string());
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
        data_row.push(*key as f32);
        data_row.push(games_hashmap[&key].get_games().len() as f32);
        data_row.push(games_hashmap[&key].get_statistics().get_avg());
        data_row.push(games_hashmap[&key].get_statistics().get_median() as f32);
        data_row.push(games_hashmap[&key].get_statistics().get_var());
        data_row.push(games_hashmap[&key].get_statistics().get_std());
        data_row.push(games_hashmap[&key].get_statistics().get_min() as f32);
        data_row.push(games_hashmap[&key].get_statistics().get_max() as f32);
        data.push(data_row);
    }
    pretty_printers::table_printer(headers, data);
}

fn load_data(file: &str) -> HashMap<i32, cluster::Cluster> {
    let mut games_hashmap:HashMap<i32, cluster::Cluster> = HashMap::new();
    let mut current_game = game_old::Game::new();
    let mut game_count: i32 = 0;
    if let Ok(file) = File::open(file) {
        let reader = BufReader::new(file);

        let mut lines = reader.lines().map(|line| line.unwrap());
        while let Some(chunk) = file_handling::read_chunk(&mut lines) {
            // If the game is abandoned, skip it
            if chunk["Termination"] == "\"Abandoned\"" {
                continue;
            }

            // Storing the read data in an instance of the `game` structure
            current_game.set_time_control(retrieve_time(&chunk["TimeControl"]));
            current_game.set_white_elo(retrieve_elo(&chunk["WhiteElo"]));
            current_game.set_black_elo(retrieve_elo(&chunk["BlackElo"]));
            current_game.set_number_of_moves(retrieve_number_of_moves(&chunk["Moves"]));
            current_game.set_id(game_count);

            // Storing the current game in an instance of the `cluster` structure
            games_hashmap.entry(current_game.get_time_control()).or_insert_with(cluster::Cluster::new);
            games_hashmap.get_mut(&current_game.get_time_control()).unwrap().add_game(current_game);

            game_count += 1;
        }
    }

    games_hashmap
}

/// Given a &String from a .pgn file containing the time allocated to each player, extract its value as an i32
///
/// # Arguments
///
/// `data_string` - &String containing the time allocated to the players
///
fn retrieve_time(data_string: &String) -> i32 {
    let mut time: i32 = 0;
    match data_string.as_str() {
        "\"-\"" => {},
        _ => {
            time = data_string[1..]
                .split('+')
                .nth(0)
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
    }

    time
}

/// Given a &String from a .pgn file containing the elo of a player, extract its value as an i32
///
/// # Arguments
///
/// `data_string` - &String containing the elo of the player
///
fn retrieve_elo(data_string: &String) -> i32 {
    let mut elo: i32 = -1;
    match data_string.as_str() {
        "\"?\"" => {},
        _ => {
            elo = (&data_string[1..&data_string.len() - 1])
                .parse::<i32>()
                .unwrap();
        }
    }

    elo
}

/// Given a &String containing the information of a game, extract the total number of moves
///
/// # Arguments
///
/// `data_string` - &String containing all the moves in the played game
///
fn retrieve_number_of_moves(data_string: &String) -> i32 {
    let mut moves: i32 = 0;

    // Split the string at each space and store it in a collection
    let collection = data_string.split(' ').collect::<Vec<&str>>();

    // Iterates backwards and continues until it finds a string that ends with a dot
    // Since number of moves are the only value ending with dots,
    // it is a convenient way to filter them
    for element in collection.into_iter().rev() {
        if element.ends_with('.') {
            // Remove all dots at the end of the string and store it as an i32
            let element = element.trim_end_matches('.');
            moves += element.parse::<i32>().unwrap();
            break;
        }
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        let time_string: String = String::from("\"180+0\"");

        assert_eq!(180, retrieve_time(&time_string));
    }

    #[test]
    fn test_elo() {
        let elo_string: String = String::from("\"512\"");

        assert_eq!(512, retrieve_elo(&elo_string));
    }

    #[test]
    fn test_moves() {
        let moves_string: String = String::from("1. e4 e5 2. f4 Bc5 3. Nf3 Nc6 4. fxe5 Qh4+ 5. Nxh4 Nxe5 6. d4 Bb4+ 7. c3 Nd3+ 1-0");

        assert_eq!(7, retrieve_number_of_moves(&moves_string));
    }
}
