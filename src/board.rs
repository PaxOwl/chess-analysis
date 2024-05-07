use colored::Colorize;
use crate::board;

const BOARD_SIZE: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Piece {
    piece_type: PieceType,
    color: Color,
    position: (usize, usize), // Position on the board (row, column)
}

impl Piece {
    // Constructor method to create a new piece instance
    fn new(piece_type: PieceType, color: Color, position: (usize, usize)) -> Piece {
        Piece {
            piece_type,
            color,
            position,
        }
    }

    fn symbol(&self) -> char {
        match self.color {
            Color::White => match self.piece_type {
                PieceType::Pawn => '♙',
                PieceType::Knight => '♘',
                PieceType::Bishop => '♗',
                PieceType::Rook => '♖',
                PieceType::Queen => '♕',
                PieceType::King => '♔',
                _ => '·',
            },
            Color::Black => match self.piece_type {
                PieceType::Pawn => '♟',
                PieceType::Knight => '♞',
                PieceType::Bishop => '♝',
                PieceType::Rook => '♜',
                PieceType::Queen => '♛',
                PieceType::King => '♚',
                _ => '·',
            },
        }
    }

    fn update_position(&mut self, new_position: (usize, usize)) {
        self.position = new_position;
    }
}

#[derive(Debug)]
pub(crate) struct Board {
    pub(crate) board: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub(crate) fn new() -> Board {
        let mut board = [[None; BOARD_SIZE]; BOARD_SIZE];

        // Pieces initialization
        // Pawns
        for col in 0..8 {
            board[1][col] = Some(Piece::new(PieceType::Pawn, Color::White, (1, col)));
            board[6][col] = Some(Piece::new(PieceType::Pawn, Color::Black, (6, col)));
        }

        //Other pieces
        board[0][0] = Some(Piece::new(PieceType::Rook, Color::White, (0, 0)));
        board[0][1] = Some(Piece::new(PieceType::Knight, Color::White, (0, 1)));
        board[0][2] = Some(Piece::new(PieceType::Bishop, Color::White, (0, 2)));
        board[0][3] = Some(Piece::new(PieceType::Queen, Color::White, (0, 3)));
        board[0][4] = Some(Piece::new(PieceType::King, Color::White, (0, 4)));
        board[0][5] = Some(Piece::new(PieceType::Bishop, Color::White, (0, 5)));
        board[0][6] = Some(Piece::new(PieceType::Knight, Color::White, (0, 6)));
        board[0][7] = Some(Piece::new(PieceType::Rook, Color::White, (0, 7)));

        board[7][0] = Some(Piece::new(PieceType::Rook, Color::Black, (7, 0)));
        board[7][1] = Some(Piece::new(PieceType::Knight, Color::Black, (7, 1)));
        board[7][2] = Some(Piece::new(PieceType::Bishop, Color::Black, (7, 2)));
        board[7][3] = Some(Piece::new(PieceType::Queen, Color::Black, (7, 3)));
        board[7][4] = Some(Piece::new(PieceType::King, Color::Black, (7, 4)));
        board[7][5] = Some(Piece::new(PieceType::Bishop, Color::Black, (7, 5)));
        board[7][6] = Some(Piece::new(PieceType::Knight, Color::Black, (7, 6)));
        board[7][7] = Some(Piece::new(PieceType::Rook, Color::Black, (7, 7)));

        Board { board }
    }

    fn is_move_valid(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        // Check if the start and end positions are within the board bounds
        if from.0 >= 8 || from.1 >= 8 || to.0 >= 8 || to.1 >= 8 {
            return false;
        }

        // Check if the end position is not occupied by another piece
        if self.board[to.0][to.1] != None {
            return false;
        }

        // You can add more complex validation logic here, such as checking for piece-specific movement rules
        match self.board[from.0][from.1].unwrap().piece_type {
            PieceType::Pawn => { if !self.is_pawn_move_valid(from, to) {return false} },
            PieceType::Knight => { if !self.is_knight_move_valid(from, to) {return false} },
            PieceType::Bishop => { if !self.is_bishop_move_valid(from, to) {return false} },
            PieceType::Rook => { if !self.is_rook_move_valid(from, to) {return false} },
            PieceType::Queen => { if !self.is_queen_move_valid(from, to) {return false} },
            PieceType::King => { if !self.is_king_move_valid(from, to) {return false} },
        }
        // If none of the conditions above are met, the move is valid
        true
    }

    fn is_pawn_move_valid(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        // Moving horizontally
        if from.1 != to.1 { return false }
        // Moving more than 2 squares vertically
        if (from.0 as i32 - to.0 as i32).abs() > 2 { return false}
        // Moving 2 squares vertically when not on initial position
        match self.board[from.0][from.1].unwrap().color {
            Color::White => {
                if from.0 == to.0 && from.1 == to.1 + 2 {
                    if !(from.1 == 1) { return false }
                }
            },
            Color::Black => {
                if from.0 == to.0 && from.1 == to.1 - 2 {
                    if !(from.1 == 6) { return false }
                }
            }
        }
        true
    }

    fn is_knight_move_valid(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        if ((from.0 as i32 - to.0 as i32).abs() == 1
            && (from.1 as i32 - to.1 as i32).abs() == 2)
            || ((from.0 as i32 - to.0 as i32).abs() == 2
            && (from.1 as i32 - to.1 as i32).abs() == 1) {
            return true
        }
        false
    }

    fn is_bishop_move_valid(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        if (from.0 as i32 - to.0 as i32).abs() == (from.1 as i32 - to.1 as i32).abs() {
            return true
        }
        false
    }

    fn is_rook_move_valid(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        if (from.0 as i32 - to.0 as i32).abs() > 0 && (from.1 as i32 - to.1 as i32).abs() > 0 {
            return false
        }
        true
    }

    fn is_queen_move_valid(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        if (from.0 as i32 - to.0 as i32).abs() == (from.1 as i32 - to.1 as i32).abs() {
            return true
        }
        if !((from.0 as i32 - to.0 as i32).abs() > 0 && (from.1 as i32 - to.1 as i32).abs() > 0) {
            return true
        }
        false
    }

    fn is_king_move_valid(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        true
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        println!("Moving {:?} {:?} in {:?} to {:?}",
                 self.board[from.0][from.1].unwrap().color,
                 self.board[from.0][from.1].unwrap().piece_type,
                 self.board[from.0][from.1].unwrap().position,
                 to);
        if !self.is_move_valid(from, to) {
            println!("{}", "Invalid move.".red());
            return;
        }

        if let Some(piece) = self.board[from.0][from.1] {
            self.board[to.0][to.1] = Some(piece);
            self.board[from.0][from.1] = None;
            println!("{}", "Valid move.".green());
        } else {
            println!("No piece found at the specified position.");
        }
    }

    pub(crate) fn print(&self) {
        println!("{}", " ------------------".yellow());
        let mut row_index: usize = 8;
        for row in self.board.iter() {
            print!("{}{}", row_index.to_string().blue(), "|".yellow());
            for square in row.iter() {
                match square {
                    Some(piece) => print!("{: <2}", piece.symbol()),
                    None => print!("{: <2}", ' '),
                }
            }
            row_index -= 1;
            println!("{}", "|".yellow());

        }
        println!("{}", " ------------------".yellow());
        println!("{}", "  a b c d e f g h".blue());

    }
}
