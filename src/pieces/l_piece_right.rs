use crate::pieces::piece::ClonePiece;
use crate::pieces::piece::Piece;
use serde::Serialize;
use serde_json;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

use crate::colors;
use crate::geometry::{Position, Rect};

/// LPieceRight is the L tetris piece. It has Right in it's name because the base of the piece points
/// to the right. On the other hand this tetris piece -> ⅃ <- has its base pointing to the left so
/// we'll be calling it LPieceLeft.
#[derive(Serialize, Copy, Clone)]
pub struct LPieceRight {
    orientation: Orientation,
    origin: Position,
}

/// There are four possible orientations for LPieceRight and the names for these orientations come
/// from the direction that the big hand of the piece is pointing towards
#[derive(Serialize, Copy, Clone)]
enum Orientation {
    Up,    // as in L
    Down,  // as in ⅂
    Left,  // as in ___|
    Right, // as in |___
}

impl LPieceRight {
    pub fn new(x: i32, y: i32) -> Self {
        let origin = Position { x, y };

        LPieceRight {
            orientation: Orientation::Up,
            origin,
        }
    }
}

impl ClonePiece for LPieceRight {
    fn clone_piece(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

impl Display for LPieceRight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&serde_json::to_string(self).expect("could not convert to json"))
    }
}

impl Piece for LPieceRight {
    fn bounding_box(&self) -> Rect<i32> {
        match self.orientation {
            Orientation::Up => Rect {
                x1: self.origin.x,
                y1: self.origin.y - 1,
                x2: self.origin.x + 1,
                y2: self.origin.y + 1,
            },
            Orientation::Down => Rect {
                x1: self.origin.x-1,
                y1: self.origin.y - 1,
                x2: self.origin.x,
                y2: self.origin.y + 1,
            },
            Orientation::Left => Rect {
                x1: self.origin.x - 1,
                y1: self.origin.y - 1,
                x2: self.origin.x + 1,
                y2: self.origin.y,
            },
            Orientation::Right => Rect {
                x1: self.origin.x - 1,
                y1: self.origin.y,
                x2: self.origin.x + 1,
                y2: self.origin.y + 1,
            },
        }
    }

    fn mask(&self) -> Vec<Position> {
        match self.orientation {
            Orientation::Up => vec![
                self.origin + (1, 1),
                self.origin + (0, 1),
                self.origin,
                self.origin + (0, -1),
            ],
            Orientation::Down => vec![
                self.origin + (-1, -1),
                self.origin + (0, -1),
                self.origin,
                self.origin + (0, 1),
            ],
            Orientation::Left => vec![
                self.origin + (-1, 0),
                self.origin,
                self.origin + (1, 0),
                self.origin + (1, -1),
            ],
            Orientation::Right => vec![
                self.origin + (-1, 1),
                self.origin + (-1, 0),
                self.origin,
                self.origin + (1, 0),
            ],
        }
    }

    fn rotate_left(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Left,
            Orientation::Left => Orientation::Down,
            Orientation::Down => Orientation::Right,
            Orientation::Right => Orientation::Up,
        }
    }

    fn rotate_right(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
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

    fn get_origin(&self) -> Position {
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

        context.set_stroke_style(&colors::LPIECE_STROKE.into());
        context.set_fill_style(&colors::LPIECE_FILL.into());

        context.begin_path();

        match self.orientation {
            Orientation::Up => {
                context.move_to(origin_x, origin_y - pixels_per_cell);
                context.line_to(origin_x + pixels_per_cell, origin_y - pixels_per_cell);
                context.line_to(origin_x + pixels_per_cell, origin_y + pixels_per_cell);
                context.line_to(origin_x + 2.0 * pixels_per_cell, origin_y + pixels_per_cell);
                context.line_to(origin_x + 2.0 * pixels_per_cell, origin_y + 2.0 * pixels_per_cell);
                context.line_to(origin_x, origin_y + 2.0 * pixels_per_cell);
                context.line_to(origin_x, origin_y - pixels_per_cell);
            }
            Orientation::Down => {
                context.move_to(origin_x, origin_y);
                context.line_to(origin_x-pixels_per_cell, origin_y);
                context.line_to(origin_x-pixels_per_cell, origin_y-pixels_per_cell);
                context.line_to(origin_x+pixels_per_cell, origin_y-pixels_per_cell);
                context.line_to(origin_x+pixels_per_cell, origin_y + 2.0 * pixels_per_cell);
                context.line_to(origin_x, origin_y + 2.0 * pixels_per_cell);
                context.line_to(origin_x, origin_y);
            }
            Orientation::Left => {
                context.move_to(origin_x + pixels_per_cell, origin_y);
                context.line_to(origin_x + pixels_per_cell, origin_y - pixels_per_cell);
                context.line_to(origin_x + 2.0 * pixels_per_cell, origin_y - pixels_per_cell);
                context.line_to(origin_x + 2.0 * pixels_per_cell, origin_y + pixels_per_cell);
                context.line_to(origin_x - pixels_per_cell, origin_y + pixels_per_cell);
                context.line_to(origin_x - pixels_per_cell, origin_y);
                context.line_to(origin_x + pixels_per_cell, origin_y);
            }
            Orientation::Right => {
                context.move_to(origin_x - pixels_per_cell, origin_y);
                context.line_to(origin_x + 2.0 * pixels_per_cell, origin_y);
                context.line_to(origin_x + 2.0 * pixels_per_cell, origin_y + pixels_per_cell);
                context.line_to(origin_x, origin_y + pixels_per_cell);
                context.line_to(origin_x, origin_y + 2.0 * pixels_per_cell);
                context.line_to(origin_x - pixels_per_cell, origin_y + 2.0 * pixels_per_cell);
                context.line_to(origin_x - pixels_per_cell, origin_y);
            }
        }

        context.stroke();
        context.fill();
    }
}
