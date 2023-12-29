use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

/// Read the file in argument and store it line by line
///
/// # Arguments
///
/// `filename` - Name of the file to read
///
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_chunk<I>(lines: &mut I) -> Option<HashMap<String, String>>
where
    I: Iterator<Item = String>,
{
    let mut chunk = HashMap::new();
    let mut line: String = String::from("dummy");
    while line != "" {
        if let Some(line) = lines.next() {
            if line == "" {
                lines.next();
                lines.next();
                break;
            }
            else {
                let key = &line[1..].split_once(char::is_whitespace).unwrap().0;
                let value = &line[..line.len()-1].split_once(char::is_whitespace).unwrap().1;
                chunk.insert(String::from(*key), String::from(*value));
            }
        }
        else {
            return if chunk.is_empty() { None } else { Some(chunk)};
        }
    }
    Some(chunk)
}


/// When given an &String element with the right format, isolate the `TimeControl` sequence
/// and returns it as an f32
///
///
/// # Arguments
///
/// `ip` - A &String holding the sequence to extract
///
/// # Examples
///
/// ```
/// Expected format : [TimeControl "60+0"]
/// In this case, 60. is returned as an f32
/// ```
pub fn get_time_control(line_content: &String) -> i32 {
    
    // First instance of " character
    let start_byte = line_content
        .find('\"')
        .unwrap_or(0);

    // First instance of + character
    let end_byte = line_content
        .find('+')
        .unwrap_or(0);

    // Retrieve the TimeControl value as a &str, convert it and store it as an f32 variable
    let str_time = &line_content[start_byte + 1..end_byte];
    let time: i32 = str_time.parse().unwrap();

    time
}

pub fn get_elo(line_content: &String) -> i32 {

    // First instance of " character
    let start_byte = line_content
        .find('\"')
        .unwrap_or(0);

    // First instance of + character
    let end_byte = line_content[start_byte + 1..]
        .find('\"')
        .unwrap_or(0);

    // Retrieve the TimeControl value as a &str, convert it and store it as an f32 variable
    let str_elo = &line_content[start_byte + 1..start_byte + end_byte + 1];
    let elo: i32 = str_elo.parse().unwrap();

    elo
}

/// Given an &String containing the information of a game, extract the total number of moves
///
/// # Arguments
///
/// `game_moves` - &String containing all the moves in the played game
///
pub fn get_number_of_moves(game_moves: &String) -> i32 {
    let mut moves: i32 = 0;

    // Split the string at each space and store it in a collection
    let collection = game_moves.split(' ').collect::<Vec<&str>>();

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