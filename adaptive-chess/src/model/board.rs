use crate::logic::board_config::BoardConfig;
use resvg::{tiny_skia, usvg};
use crate::cache::Cache;
use crate::model::piece::BoardPiece;

pub struct Board {
    side_length: u32,
    board_config: BoardConfig,
    white_colour: [u8; 4],
    black_colour: [u8; 4],
    glyph_cache: Cache<usvg::Tree>,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            side_length: 720,
            board_config: BoardConfig::default(),
            white_colour: [0xe3, 0xc1, 0x6f, 0xff],
            black_colour: [0xb8, 0x8b, 0x4a, 0xff],
            glyph_cache: Cache::default(),
        }
    }
}

impl Board {
    pub fn get_side_length(&self) -> u32 {
        self.side_length
    }

    pub fn reset(&mut self) {
        *self = Board::default()
    }

    pub fn set_board_config_from_fen_str(&mut self, fen_str: &str) {
        self.board_config = BoardConfig::get_board_config_from_fen_str(fen_str)
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        let mut pixmap = tiny_skia::Pixmap::new(self.side_length, self.side_length).unwrap();

        let mut white_paint = tiny_skia::Paint::default();
        white_paint.set_color_rgba8(
            self.white_colour[0],
            self.white_colour[1],
            self.white_colour[2],
            self.white_colour[3],
        );
        let mut black_paint = tiny_skia::Paint::default();
        black_paint.set_color_rgba8(
            self.black_colour[0],
            self.black_colour[1],
            self.black_colour[2],
            self.black_colour[3],
        );

        let check_side = self.get_check_side();
        let glyph_width = (check_side * 0.75) as u32;

        // Draw the checkboard and all the arrangement of pieces
        for y in 0..8 {
            for x in 0..8 {
                let rect = tiny_skia::Rect::from_xywh(
                    x as f32 * check_side,
                    y as f32 * check_side,
                    check_side,
                    check_side,
                )
                .unwrap();
                let paint = if x % 2 == 0 {
                    if y % 2 == 0 {
                        &white_paint
                    } else {
                        &black_paint
                    }
                } else {
                    if y % 2 == 0 {
                        &black_paint
                    } else {
                        &white_paint
                    }
                };

                pixmap.fill_rect(rect, paint, tiny_skia::Transform::identity(), None);

            }
        }

        frame.copy_from_slice(pixmap.data());
    }

    fn get_check_side(&self) -> f32 {
        (self.side_length / 8) as f32
    }

    fn get_glyph_tree(& mut self, p: &BoardPiece) -> usvg::Tree {
        let glyph_path = Board::get_glyph_path(p);
        match self.glyph_cache.get(&glyph_path) {
            Some(t) => t,
            None => {
                log::info!("Importing glyph {}", glyph_path);
                let str = std::fs::read_to_string(&glyph_path).unwrap_or_else(|e| {
                    log::error!("Error Importing {}: {}", &glyph_path, e);
                    panic!();
                });
                let t = usvg::TreeParsing::from_str(&str, &usvg::Options::default()).unwrap();
                self.glyph_cache.put(&glyph_path, &t);
                t
            }
        }
    }

    fn get_glyph_path(p: &BoardPiece) -> String {
        let s = format!("assets/pieces/{}.svg", p);
        s.to_owned()
    }
    
}