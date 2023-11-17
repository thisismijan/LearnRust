use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Colour {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoardPiece {
    White(Piece),
    Black(Piece),
}

impl fmt::Display for BoardPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BoardPiece::*;
        use Piece::*;

        //using FEN

        let name = match self {
            White(p) => match p {
                King => "K",
                Queen => "Q",
                Rook => "R",
                Bishop => "B",
                Knight => "N",
                Pawn => "P",
            },
            Black(p) => match p {
                King => "k",
                Queen => "q",
                Rook => "r",
                Bishop => "b",
                Knight => "n",
                Pawn => "p",
            },
        };

        write!(f, "{}", name)
    }
}