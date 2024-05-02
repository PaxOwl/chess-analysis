use std::collections::HashMap;
use crate::board;


pub fn play(board_state: board::Board, input_game: HashMap<String, String>) {
    let moves = moves_to_vector(&input_game["Moves"]);
    // TODO: Iterate over the moves, decode the current move and apply it to the board
}

fn moves_to_vector(input_moves: &String) -> Vec<(&str, &str)> {
    let mut moves: Vec<(&str, &str)> = Vec::new();

    let v: Vec<&str> = input_moves.as_str().split(" ").collect();
    for mut i in (0..v.len()).step_by(3) {
        if i + 2 >= v.len() {
            break;
        }
        let data_tuple = (v[i + 1], v[i + 2]);
        moves.push(data_tuple);
    }
    moves
}