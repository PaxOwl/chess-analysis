#[derive(Clone, Copy)]
pub struct Game {
    id: i32,
    white_elo: i32,
    black_elo: i32,
    time_control: i32,
    number_of_moves: i32,
}

impl Game {
    // Constructor
    pub const fn new() -> Self {
        Game {
            id: 0,
            white_elo: 0,
            black_elo: 0,
            time_control: 0,
            number_of_moves: 0
        }
    }

    // Getters
    pub fn get_id(&self) -> i32{
        self.id
    }

    pub fn get_white_elo(&self) -> i32 {
        self.white_elo
    }

    pub fn get_black_elo(&self) -> i32 {
        self.black_elo
    }

    pub fn get_time_control(&self) -> i32 {
        self.time_control
    }

    pub fn get_number_of_moves(&self) -> i32 {
        self.number_of_moves
    }

    // Setters
    pub fn set_id(&mut self, id: i32) {
        self.id = id
    }

    pub fn set_white_elo(&mut self, white_elo: i32) {
        self.white_elo = white_elo
    }

    pub fn set_black_elo(&mut self, black_elo: i32) {
        self.black_elo = black_elo
    }

    pub fn set_time_control(&mut self, time_control: i32) {
        self.time_control = time_control
    }

    pub fn set_number_of_moves(&mut self, number_of_moves: i32) {
        self.number_of_moves = number_of_moves
    }

    // Pretty printer
    // pub fn print(&self) {
    //     println!("Game {:>5}", self.id);
    //     println!("    White elo: {:>4}", self.white_elo);
    //     println!("    Black elo: {:>4}", self.black_elo);
    //     println!("    Time control: {:>4}", self.time_control);
    //     println!("    Number of moves: {:>3}", self.number_of_moves);
    // }
}
