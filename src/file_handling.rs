use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


/// Given a file to read, will go through it line by line and store it in a key:value format
/// in a HashMap
///
/// List of elements read:
///     WhiteElo, BlackElo, ECO, Opening, TimeControl, Termination
///
/// # Arguments
///
/// `filename` - &str File to read
///
/// # Returns
///
/// `Option<HashMap<String, String>>`
pub fn load_chess_game(filename: &str) -> Option<HashMap<String, String>> {
    let mut hash_game = HashMap::new();
    // Open the file
    if let Ok(file) = File::open(filename) {
        // Create a buffered reader
        let reader = BufReader::new(file);
        // Iterate over lines
        for line in reader.lines() {
            // Handle each line
            if let Ok(line_contents) = line {
                if !&line_contents.is_empty() {
                    if line_contents.contains("WhiteElo")
                        || line_contents.contains("BlackElo")
                        || line_contents.contains("ECO")
                        || line_contents.contains("Opening")
                        || line_contents.contains("TimeControl")
                        || line_contents.contains("Termination") {
                        // Create key:value association and store it in the HashMap
                        let key = &line_contents[1..]
                            .split_once(char::is_whitespace)
                            .unwrap().0;
                        let value = &line_contents[..line_contents.len() - 1]
                            .split_once(char::is_whitespace)
                            .unwrap().1;
                        hash_game.insert(String::from(*key), String::from(*value));
                    } else {
                        hash_game.insert(String::from("Moves"), line_contents);
                    }
                }
            }
            // Panics at the end of the file
        }
    }
    else {
        println!("Failed to open the file.");
    }
    Some(hash_game)
}

/// Given an Iterator of the current line to be read, returns a HashMap containing data of the current game by ordering
/// it as chunks
///
/// # Arguments
///
/// `lines` - &mut I of the current line
///
pub fn read_chunk<I>(lines: &mut I) -> Option<HashMap<String, String>>
where
    I: Iterator<Item = String>,
{
    let mut chunk = HashMap::new();
    let line: String = String::from("dummy");
    while line != "" {
        if let Some(line) = lines.next() {
            if line == "" {
                if let Some(moves) = lines.next() {
                    chunk.insert(String::from("Moves"), moves);
                }
                else {}
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
