mod file_handling;
mod pretty_printers;
mod plot;

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

fn pretty_printer(hash_vec: &Vec<(i32, Vec<i32>)>) {
    println!("    ╭──────────┬───────╮");
    println!("    │ Time (s) │ Games │");
    println!("    ├──────────┼───────┤");
    for (sorted_time, sorted_move) in hash_vec {
        println!("    │ {:>8} │ {:>5} │", sorted_time, sorted_move.len());
    }
    println!("    ╰──────────────────╯");
}

fn main() {
    let mut game_timers:HashMap<i32, Vec<i32>> = HashMap::new();
    let mut time: i32 = 0;
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
                        time = file_handling::get_time_control(&line_content);
                    }
                }
                if line_content.starts_with('1') {
                    let moves: i32 = file_handling::extract_number_of_moves(&line_content);
                    game_timers.entry(time).or_insert_with(Vec::new);
                    game_timers.get_mut(&time).expect("REASON").push(moves)
                }
            }
        }
    }

    let time_moves:Vec<(i32, Vec<i32>)> = hashmap_to_sorted_vector(&game_timers);
//    pretty_printer(&time_moves);
    plot::time_histogram(&time_moves);
    let mut table_data: Vec<Vec<i32>> = Vec::new();
    let mut headers: Vec<String> = Vec::new();
    headers.push(String::from("Times (s)"));
    headers.push(String::from("Games"));
    for i in 1..10 {
        let timers: Vec<i32> = Vec::from([i, i * 3]);
        table_data.push(timers);
        println!("{}", i);
    }
    pretty_printers::table_printer(headers, table_data);

}
