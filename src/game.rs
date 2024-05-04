use std::collections::HashMap;
use crate::board;
use colored::Colorize;


pub fn play(board_state: board::Board, input_game: HashMap<String, String>) {
    let moves = moves_to_vector(&input_game["Moves"]);
    // TODO: Iterate over the moves, decode the current move and apply it to the board
    for played_move in moves {
        let white_move: &String = &played_move.0;
        let black_move: &String = &played_move.1;
        decode_move(white_move, &board_state);
        decode_move(black_move, &board_state);
    }
}

fn moves_to_vector(input_moves: &String) -> Vec<(String, String)> {
    let mut moves: Vec<(String, String)> = Vec::new();

    let v: Vec<&str> = input_moves.as_str().split_whitespace().collect();
    for mut i in (0..v.len()).step_by(3) {
        if i + 2 >= v.len() {
            break;
        }
        let data_tuple = (v[i + 1].to_string(), v[i + 2].to_string());
        moves.push(data_tuple);
    }
    moves
}

fn decode_move(played_move: &String, board_state: &board::Board) {
        if !played_move.chars().nth(0).expect("Not a character").is_ascii_uppercase() {
            // Pawn
            move_pawn(played_move, board_state);
        }
    else {
        match played_move.chars().nth(0).expect("Not a character") {
            'P'=>{ move_pawn(played_move, board_state) },
            'N'=>{ move_knight(played_move, board_state) },
            'B'=>{ move_bishop(played_move, board_state) },
            'R'=>{ move_rook(played_move, board_state) },
            'Q'=>{ move_queen(played_move, board_state) },
            'K'=>{ move_king(played_move, board_state) },
            'O'=>{ castling(played_move, board_state) }
            _=> { println!("{:<5} -> {} ", played_move.yellow(), "Piece not recognized".red()) }
        }
    }

}

fn move_pawn(played_move: &String, board_state: &board::Board) {
    println!("{:<5} -> {}", played_move.yellow(), "Moving Pawn".blue());

}

fn move_knight(played_move: &String, board_state: &board::Board) {
    println!("{:<5} -> {}", played_move.yellow(), "Moving Knight".blue());

}

fn move_bishop(played_move: &String, board_state: &board::Board) {
    println!("{:<5} -> {}", played_move.yellow(), "Moving Bishop".blue());

}

fn move_rook(played_move: &String, board_state: &board::Board) {
    println!("{:<5} -> {}", played_move.yellow(), "Moving Rook".blue());

}

fn move_queen(played_move: &String, board_state: &board::Board) {
    println!("{:<5} -> {}", played_move.yellow(), "Moving Queen".blue());

}

fn move_king(played_move: &String, board_state: &board::Board) {
    println!("{:<5} -> {}", played_move.yellow(), "Moving King".blue());

}

fn castling(played_move: &String, board_state: &board::Board) {
    println!("{:<5} -> {}", played_move.yellow(), "Castling: moving King and Rook".blue());

}
