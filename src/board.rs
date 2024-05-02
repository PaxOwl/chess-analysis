use colored::Colorize;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Piece {
    Empty,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    fn symbol(&self) -> char {
        match *self {
            Piece::Empty => ' ',
            Piece::Pawn(Color::White) => '♙',
            Piece::Pawn(Color::Black) => '♟',
            Piece::Knight(Color::White) => '♘',
            Piece::Knight(Color::Black) => '♞',
            Piece::Bishop(Color::White) => '♗',
            Piece::Bishop(Color::Black) => '♝',
            Piece::Rook(Color::White) => '♖',
            Piece::Rook(Color::Black) => '♜',
            Piece::Queen(Color::White) => '♕',
            Piece::Queen(Color::Black) => '♛',
            Piece::King(Color::White) => '♔',
            Piece::King(Color::Black) => '♚',
        }
    }
}


#[derive(Debug)]
pub(crate) struct Board {
    squares: [[Piece; 8]; 8],
}

impl Board {
    pub(crate) fn new() -> Board {
        // Initialize the board with starting positions
        let mut squares = [[Piece::Empty; 8]; 8];

        // Fill in the starting positions for white pieces
        squares[0] = [
            Piece::Rook(Color::White),
            Piece::Knight(Color::White),
            Piece::Bishop(Color::White),
            Piece::Queen(Color::White),
            Piece::King(Color::White),
            Piece::Bishop(Color::White),
            Piece::Knight(Color::White),
            Piece::Rook(Color::White),
        ];
        squares[1] = [Piece::Pawn(Color::White); 8];

        // Fill in the starting positions for black pieces
        squares[6] = [Piece::Pawn(Color::Black); 8];
        squares[7] = [
            Piece::Rook(Color::Black),
            Piece::Knight(Color::Black),
            Piece::Bishop(Color::Black),
            Piece::Queen(Color::Black),
            Piece::King(Color::Black),
            Piece::Bishop(Color::Black),
            Piece::Knight(Color::Black),
            Piece::Rook(Color::Black),
        ];

        Board { squares }
    }

    pub(crate) fn print(&self) {
        let mut iter: i32 = 8;
        for row in self.squares.iter() {
            print!("{:<3}", iter.to_string().yellow().bold());
            iter -= 1;
            for piece in row.iter() {
                let symbol = piece.symbol();
                print!("{:<2}", symbol);
            }
            println!();
        }
        print!("{}", "\n   A B C D E F G H\n".yellow().bold());
    }
}
