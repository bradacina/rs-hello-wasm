use crate::pieces::piece::ClonePiece;
use crate::pieces::piece::Piece;
use serde::Serialize;
use serde_json;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

use crate::colors;
use crate::geometry::{Position, Rect};

#[derive(Serialize, Copy, Clone)]
pub struct LinePiece {
    orientation: Orientation,
    origin: Position<i32>,
}

#[derive(Serialize, Copy, Clone)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl LinePiece {
    pub fn new(x: i32, y: i32) -> Self {
        let origin = Position { x, y };

        LinePiece {
            orientation: Orientation::Vertical,
            origin,
        }
    }
}

impl ClonePiece for LinePiece {
    fn clone_piece(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

impl Display for LinePiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&serde_json::to_string(self).expect("could not convert to json"))
    }
}

impl Piece for LinePiece {
    fn bounding_box(&self) -> Rect<i32> {
        match self.orientation {
            Orientation::Horizontal => Rect {
                x1: self.origin.x - 1,
                y1: self.origin.y,
                x2: self.origin.x + 2,
                y2: self.origin.y,
            },
            _ => Rect {
                x1: self.origin.x,
                y1: self.origin.y - 1,
                x2: self.origin.x,
                y2: self.origin.y + 2,
            },
        }
    }

    fn mask(&self) -> Vec<Position<i32>> {
        match self.orientation {
            Orientation::Horizontal => vec![
                self.origin + (-1, 0),
                self.origin,
                self.origin + (1, 0),
                self.origin + (2, 0),
            ],
            Orientation::Vertical => vec![
                self.origin + (0, -1),
                self.origin,
                self.origin + (0, 1),
                self.origin + (0, 2),
            ],
        }
    }

    fn rotate_left(&mut self) {
        self.rotate_right();
    }

    fn rotate_right(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Horizontal => Orientation::Vertical,
            _ => Orientation::Horizontal,
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
        context.begin_path();

        let bb: Rect<f64> = match self.orientation {
            Orientation::Horizontal => Rect {
                x1: origin_x - pixels_per_cell,
                y1: origin_y,
                x2: origin_x + 3.0 * pixels_per_cell,
                y2: origin_y + pixels_per_cell,
            },
            _ => Rect {
                x1: origin_x,
                y1: origin_y - pixels_per_cell,
                x2: origin_x + pixels_per_cell,
                y2: origin_y + 3.0 * pixels_per_cell,
            },
        };

        context.set_stroke_style(&colors::BAR_STROKE.into());
        context.set_fill_style(&colors::BAR_FILL.into());

        context.move_to(bb.x1, bb.y1);
        context.line_to(bb.x1, bb.y2);
        context.line_to(bb.x2, bb.y2);
        context.line_to(bb.x2, bb.y1);
        context.line_to(bb.x1, bb.y1);

        context.stroke();
        context.fill();
    }
}
