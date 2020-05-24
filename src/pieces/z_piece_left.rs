use crate::pieces::piece::ClonePiece;
use crate::pieces::piece::Piece;
use serde::Serialize;
use serde_json;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

use crate::colors;
use crate::geometry::{Position, Rect};

/// ZPieceLeft is the Z (also stylized as '--__') tetris piece. It has Left in it's name because the
/// top of the piece points to the left. On the other hand this tetris piece -> __-- <- has its top
/// pointing to the right so we'll be calling it ZPieceRight.
#[derive(Serialize, Copy, Clone)]
pub struct ZPieceLeft {
    orientation: Orientation,
    origin: Position<i32>,
}

#[derive(Serialize, Copy, Clone)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl ZPieceLeft {
    pub fn new(x: i32, y: i32) -> Self {
        let origin = Position { x, y };

        ZPieceLeft {
            orientation: Orientation::Horizontal,
            origin,
        }
    }
}

impl ClonePiece for ZPieceLeft {
    fn clone_piece(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

impl Display for ZPieceLeft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&serde_json::to_string(self).expect("could not convert to json"))
    }
}

impl Piece for ZPieceLeft {
    fn bounding_box(&self) -> Rect<i32> {
        match self.orientation {
            Orientation::Horizontal => Rect {
                x1: self.origin.x - 1,
                y1: self.origin.y,
                x2: self.origin.x + 1,
                y2: self.origin.y + 1,
            },
            Orientation::Vertical => Rect {
                x1: self.origin.x - 1,
                y1: self.origin.y - 1,
                x2: self.origin.x,
                y2: self.origin.y + 1,
            },
        }
    }

    fn mask(&self) -> Vec<Position<i32>> {
        match self.orientation {
            Orientation::Horizontal => vec![
                self.origin + (-1, 0),
                self.origin,
                self.origin + (0, 1),
                self.origin + (1, 1),
            ],
            Orientation::Vertical => vec![
                self.origin + (0, -1),
                self.origin,
                self.origin + (-1, 0),
                self.origin + (-1, 1),
            ],
        }
    }

    fn rotate_left(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Horizontal => Orientation::Vertical,
            _ => Orientation::Horizontal,
        }
    }

    fn rotate_right(&mut self) {
        self.rotate_left();
    }

    fn move_left(&mut self) {
        self.origin.x -= 1;
    }

    fn move_right(&mut self) {
        self.origin.x += 1;
    }

    fn set_origin(&mut self, x: i32, y: i32) {
        self.origin.x = x;
        self.origin.y = y;
    }

    fn get_origin(&self) -> Position<i32> {
        self.origin
    }

    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        origin_x: f64,
        origin_y: f64,
        pixels_per_cell: f64,
    ) {
        context
            .set_line_dash(&JsValue::from_serde(&([] as [i32; 0])).unwrap())
            .unwrap();

        context.set_stroke_style(&colors::ZPIECE_STROKE.into());
        context.set_fill_style(&colors::ZPIECE_FILL.into());

        context.begin_path();

        match self.orientation {
            Orientation::Horizontal => {
                context.move_to(origin_x - pixels_per_cell, origin_y);
                context.line_to(origin_x + pixels_per_cell, origin_y);
                context.line_to(origin_x + pixels_per_cell, origin_y + pixels_per_cell);
                context.line_to(origin_x + 2.0 * pixels_per_cell, origin_y + pixels_per_cell);
                context.line_to(
                    origin_x + 2.0 * pixels_per_cell,
                    origin_y + 2.0 * pixels_per_cell,
                );
                context.line_to(origin_x, origin_y + 2.0 * pixels_per_cell);
                context.line_to(origin_x, origin_y + pixels_per_cell);
                context.line_to(origin_x - pixels_per_cell, origin_y + pixels_per_cell);
                context.line_to(origin_x - pixels_per_cell, origin_y);
            },
            Orientation::Vertical => {
                context.move_to(origin_x, origin_y - pixels_per_cell);
                context.line_to(origin_x + pixels_per_cell, origin_y - pixels_per_cell);
                context.line_to(origin_x + pixels_per_cell, origin_y + pixels_per_cell);
                context.line_to(origin_x, origin_y + pixels_per_cell);
                context.line_to(origin_x, origin_y + 2.0 * pixels_per_cell);
                context.line_to(origin_x - pixels_per_cell, origin_y + 2.0 * pixels_per_cell);
                context.line_to(origin_x - pixels_per_cell, origin_y);
                context.line_to(origin_x, origin_y);
                context.line_to(origin_x, origin_y - pixels_per_cell);
            }
        }

        context.stroke();
        context.fill();
    }
}
