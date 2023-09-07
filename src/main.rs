use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// When given an &String element with the right format, isolate the TimeControl element
/// and returns it as an f32
///
///
/// # Arguments
///
/// `ip` - A &String holding the value to extract
///
/// # Examples
///
/// ```
/// Expected format : [TimeControl "60+0"]
/// In this case, 60. is returned
/// ```
fn get_time_control(ip: &String) -> f32 {
        let time: f32;
    // First instance of " character
    let start_byte = ip
        .find("\"")
        .unwrap_or(0);

    // First instance of + character
    let end_byte = ip
        .find("+")
        .unwrap_or(0);

    // Retrieve the TimeControl value as a &str, convert it and store it as an f32 variable
    let str_time = &ip[start_byte + 1..end_byte];
    time = str_time.parse().unwrap();

    return time
}

fn main() {
    let mut game_timers = HashMap::new();
    let mut total_moves: f32 = 0.;
    let mut lines_counter : f32 = 0.;
    let mut time: f32 = 0.;
    if let Ok(lines) = read_lines(
        "./res/lichess_db_standard_rated_2016-02.pgn"
        // "./res/temp.pgn"
    ) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.contains("TimeControl") {
                    if ip.contains("-") {
                        time = 0.;
                    }
                    else {
                        time = get_time_control(&ip);
                    }
                }
                if ip.starts_with('1') {
                    lines_counter += 1.0;
                    println!("Game {}", lines_counter);
                    let parts = ip.split(" ");
                    let collection = parts.collect::<Vec<&str>>();
                    for element in collection.into_iter().rev() {
                        if element.chars().last().unwrap() == '.' {
                            let element = element.trim_end_matches(".");
                            total_moves += element.parse::<i32>().unwrap() as f32;
                            break;
                        }
                    }
                    if !game_timers.contains_key(&time.to_string()) {
                        game_timers.insert(time.to_string(), Vec::new());
                    }
                    game_timers.get_mut(&time.to_string()).expect("REASON").push(total_moves)
                }
            }
        }
    }
    for (htime, _) in &game_timers {
        println!("{}", htime);
    }
}
