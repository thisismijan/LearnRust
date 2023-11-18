use crate::model::board::{BoardConfig, BoardMatrix};
use crate::model::piece::{BoardPiece, BoardPiece::*, Colour, Piece::*};


pub struct Fen;

impl Fen {
    pub fn make_board_config_from_fen_str(s: &str) -> BoardConfig {
        Fen::make_board_config(s)
    }

    pub fn make_fen_str_from_board_config(board_config: &BoardConfig) -> String {
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
            if y < 7 { 
                fen.push('/');
            }
        }
        // end of files and add which colour's turn it is to config
        match board_config.get_active_colour() {
            Colour::White => fen.push_str(" w "),
            Colour::Black => fen.push_str(" b "),
        }
        if board_config.can_white_castle_short() {
            fen.push_str("K");
        }
        if board_config.can_white_castle_long() {
            fen.push_str("Q");
        }
        if board_config.can_black_castle_short() {
            fen.push_str("k");
        }
        if board_config.can_black_castle_long() {
            fen.push_str("q");
        }
        fen.push(' ');
        if let Some(coord) = board_config.get_en_passant_on_coord() {
            fen.push_str(&Fen::get_square_from_coord(coord));
        } else {
            fen.push('-');
        }
        fen.push(' ');
        fen.push_str(&board_config.get_halfmove_clock().to_string());
        fen.push(' ');
        fen.push_str(&board_config.get_fullmove_number().to_string());
        fen
    }

    fn get_square_from_coord(coord: (usize, usize)) -> String {
        let file = 'a'.to_ascii_lowercase() as u8 + coord.0 as u8;
        let rank = (8 - coord.1) as u8;
        std::str::from_utf8(&[file, rank]).unwrap().to_string()
    }

    fn get_coords_from_square(square: &str) -> (usize, usize) {
        let a = 'a'.to_ascii_lowercase() as usize;

        let mut square_chars = square.chars();
        let c = square_chars.next().unwrap();

        let file = if c.is_alphabetic() {
            c.to_ascii_lowercase() as usize - a
        } else {
            log::error!("invalid file: {} is invalid square", square);
            panic!();
        };

        let n: String = square_chars.collect();
        let rank = 8 as usize - n.parse::<usize>().unwrap();

        (file, rank)
    }

    fn make_board_config(fen_str: &str) -> BoardConfig {
        let mut board_matrix = BoardMatrix::default();
        let mut active_colour = Colour::White;
        let mut white_castle_short = false;
        let mut white_castle_long = false;
        let mut black_castle_short = false;
        let mut black_castle_long = false;
        let mut en_passant_on_coord: Option<(usize, usize)> = None;
        let mut halfmove_clock = 0;
        let mut fullmove_number = 0;
        for (i, data) in fen_str.split_whitespace().enumerate() {
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
                    if !(data.len() == 1 && data.chars().next() == Some('-')) {
                        let mut chars = data.chars();
                        while let Some(c) = chars.next() {
                            match c {
                                'K' => white_castle_short = true,
                                'Q' => white_castle_long = true,
                                'k' => black_castle_short = true,
                                'q' => black_castle_long = true,
                                _ => {
                                    log::error!("Castle field invalid: {}", c);
                                    panic!();
                                }
                            }
                        }
                    }
                }
                3 => {
                    if !(data.len() == 1 && data.chars().next() == Some('-')) {
                        en_passant_on_coord = Some(Self::get_coords_from_square(data));
                    }
                }
                4 => {
                    if let Ok(n) = data.parse::<u32>() {
                        halfmove_clock = n;
                    } else {
                        log::error!("Halfmove clock field invalid: {}", data);
                        panic!();
                    }
                }
                5 => {
                    if let Ok(n) = data.parse::<u32>() {
                        fullmove_number = n;
                    } else {
                        log::error!("Fullmove number field invalid: {}", data);
                        panic!();
                    }
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
            white_castle_short,
            white_castle_long,
            black_castle_short,
            black_castle_long,
            en_passant_on_coord,
            halfmove_clock,
            fullmove_number,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_board_config_from_fen_string_default() {
        let board_config = Fen::make_board_config_from_fen_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(board_config.get_active_colour(), Colour::White);
    }

    #[test]
    fn test_make_fen_str_from_board_config_default() {
        let board_config = BoardConfig::default();
        let fen_str = Fen::make_fen_str_from_board_config(&board_config);
        assert_eq!(fen_str, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }
}