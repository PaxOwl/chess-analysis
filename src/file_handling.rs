use std::collections::HashMap;


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
