use crate::model::piece::{BoardPiece, Colour};
use fen_parser::Fen;

pub type BoardMatrix = [[Option<BoardPiece>; 8]; 8];

pub struct BoardConfig {
    board_matrix: BoardMatrix,
    fen_str: String,
    active_colour: Color,
}

impl Default for BoardConfig {
    fn default() -> Self {
        Fen::make_board_config_from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }
}

impl BoardConfig {
    pub fn get_from_fen_str(s: &str) -> Self {
        Fen::make_board_config_from_str(s)
    }

    pub fn get_piece_at_coord(&self, x: usize, y: usize) -> Option<BoardPiece> {
        self.board_matrix[y][x]
    }

    pub fn get_active_colour(&self) -> Colour {
        self.active_colour
    }
}