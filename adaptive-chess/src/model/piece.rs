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

        //matching name of the image in assets

        let name = match self {
            White(p) => match p {
                King => "k_q",
                Queen => "q_w",
                Rook => "r_w",
                Bishop => "b_w",
                Knight => "n_w",
                Pawn => "p_w",
            },
            Black(p) => match p {
                King => "k_b",
                Queen => "q_b",
                Rook => "r_b",
                Bishop => "b_b",
                Knight => "n_b",
                Pawn => "p_b",
            },
        };

        write!(f, "{}", name)
    }
}
