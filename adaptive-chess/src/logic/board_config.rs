use crate::model::piece::{BoardPiece, Colour};
use crate::logic::fen_parser::Fen;

pub type BoardMatrix = [[Option<BoardPiece>; 8]; 8];

pub struct BoardConfig {
    pub board_matrix: BoardMatrix,
    pub fen_str: String,
    pub active_colour: Colour,
    pub white_castle_short: bool,
    pub white_castle_long: bool,
    pub black_castle_short: bool,
    pub black_castle_long: bool,
    pub en_passant_on_coord: Option<(usize, usize)>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl Default for BoardConfig {
    fn default() -> Self {
        Fen::make_board_config_from_fen_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl BoardConfig {
    pub fn get_board_config_from_fen_str(s: &str) -> Self {
        Fen::make_board_config_from_fen_str(s)
    }

    pub fn get_piece_at_coord(&self, x: usize, y: usize) -> Option<BoardPiece> {
        self.board_matrix[y][x]
    }

    pub fn get_active_colour(&self) -> Colour {
        self.active_colour
    }

    pub fn can_white_castle_long(&self) -> bool {
        self.white_castle_long
    }

    pub fn can_white_castle_short(&self) -> bool {
        self.white_castle_short
    }

    pub fn can_black_castle_long(&self) -> bool {
        self.black_castle_long
    }

    pub fn can_black_castle_short(&self) -> bool {
        self.black_castle_short
    }

    pub fn get_en_passant_on_coord(&self) -> Option<(usize, usize)> {
        self.en_passant_on_coord
    }

    pub fn get_halfmove_clock(&self) -> u32 {
        self.halfmove_clock
    }

    pub fn get_fullmove_number(&self) -> u32 {
        self.fullmove_number
    }
}