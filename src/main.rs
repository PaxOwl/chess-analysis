use chess_analysis;

use colored::Colorize;


fn main() {
    chess_analysis::run("./res/reduced_data.pgn");
    // chess_analysis::run("./res/lichess_db_standard_rated_2016-02.pgn");

    println!("\n{}", "Program exited normally".green().bold());
}
