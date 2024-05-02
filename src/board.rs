use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Piece {
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
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
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::Queen,
            Piece::King,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];
        squares[1] = [Piece::Pawn; 8];

        // Fill in the starting positions for black pieces
        squares[6] = [Piece::Pawn; 8];
        squares[7] = [
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::Queen,
            Piece::King,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];

        Board { squares }
    }

    pub(crate) fn print(&self) {
        let piece_symbols = [
            // ('E', Piece::Empty),
            ('P', Piece::Pawn),
            ('N', Piece::Knight),
            ('B', Piece::Bishop),
            ('R', Piece::Rook),
            ('Q', Piece::Queen),
            ('K', Piece::King),
        ];

        for row in self.squares.iter() {
            for piece in row.iter() {
                let piece_str = format!("{:?}", piece);
                let symbol = piece_symbols.iter()
                    .find_map(|(s, p)| if *p == *piece { Some(*s) } else { None }).unwrap_or(' ');
                print!("{:<2}", symbol);
            }
            println!();
        }
    }
}
