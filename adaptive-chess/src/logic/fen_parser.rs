use crate::model::piece::{BoardPiece, BoardPiece::*, Colour, Piece::*};
use crate::model::board::{BoardConfig, BoardConfig::*, BoardMatrix};
pub struct Fen;

impl Fen {

    pub fn make_board_config_from_str(s: &str) -> BoardConfig {
        Fen::make_board_config(s);
    }

    pub fn make_fen_from_board_config(board_config: &BoardConfig) -> String {
        let mut fen = String::new();
        for y in 0..8 {
            let mut empty = 0;
            for x in 0..8 {
                if let Some(p) = board_config.get_piece_at_coord(x, y) {
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }
                    fen.push(Fen::get_fen_char_from_board_piece(p));
                } else {
                    empty += 1;
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
            }
            //end of rank   
            fen.push('/');
        }
        // end of files and add which colours turn it is to config
        match board_config.get_active_colour(){
            Colour::White => fen.push_str(" w "),
            Colour::Black => fen.push_str(" b "),
        }
        //TODO: can white castle short
        //TODO: can white castle long
        //TODO: can black castle short
        //TODO: can black castle long
        fen.push(' ');
        //TODO: is en passant available
        //TODO: half move clock
        //TODO: full move clock
        s
    }

    fn make_board_config(fen_str: &str) -> BoardConfig {
        let mut board_matrix = BoardMatrix::default();
        let mut active_colour = Colour::White;
        for(i, data) in fen_str.split_whitespace().enumerate() {
            match i {
                0 => {
                    for (i, rank) in data.split('/').enumerate() {
                        let mut x = 0;
                        for c in rank.chars() {
                            if c.is_digit(10) {
                                x += c.to_digit(10).unwrap() as usize;
                            } else {
                                log::debug!("Piece {c} at {:?}", (i, x));
                                board_matrix[i][x] = Some(Fen::get_board_piece_from_char(c));
                                x += 1;
                            }
                        }
                    }
                }
                1 => {
                    if data.len() > 1 {
                        log::error!("Active colour field invalid");
                    } else {
                        if let Some(c) = data.chars().next() {
                            match c {
                                'w' => active_colour = Colour::White,
                                'b' => active_colour = Colour::Black,
                                _ => {
                                    log::error!("Active colour field invalid: {}", c);
                                    panic!();
                                }
                            }
                        }
                    }
                }
                2 => {
                    //TODO castling
                }
                3 => {
                    //TODO en passant
                }
                4 => {
                    //TODO half move clock
                }
                5 => {
                    //TODO full move clock
                }
                _ => {
                    log::error!("Fen string invalid");
                    panic!();
                }
            };
        }

        BoardConfig {
            board_matrix,
            fen_str: fen_str.to_string(),
            active_colour,
        }
    }


    //TODO is there a rust BiMap? These two lookups will be called a lot, so need to research efficient data structures for them

    fn get_fen_char_from_board_piece(p: BoardPiece) -> char {
        match p {
            White(p) => match p {
                King => 'K',
                Queen => 'Q',
                Rook => 'R',
                Bishop => 'B',
                Knight => 'N',
                Pawn => 'P',
            },
            Black(p) => match p {
                King => 'k',
                Queen => 'q',
                Rook => 'r',
                Bishop => 'b',
                Knight => 'n',
                Pawn => 'p',
            },
        }
    }

    fn get_board_piece_from_char(c: char) -> BoardPiece {
        match c {
            'K' => BoardPiece::White(King),
            'k' => BoardPiece::Black(King),
            'Q' => BoardPiece::White(Queen),
            'q' => BoardPiece::Black(Queen),
            'R' => BoardPiece::White(Rook),
            'r' => BoardPiece::Black(Rook),
            'B' => BoardPiece::White(Bishop),
            'b' => BoardPiece::Black(Bishop),
            'N' => BoardPiece::White(Knight),
            'n' => BoardPiece::Black(Knight),
            'P' => BoardPiece::White(Pawn),
            'p' => BoardPiece::Black(Pawn),
            _ => {
                    log::error!("Invalid piece: {}", c);
                    panic!()
                }
        }
    }
}