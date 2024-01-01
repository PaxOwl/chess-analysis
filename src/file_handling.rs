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
