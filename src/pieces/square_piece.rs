use crate::colors;
use crate::geometry::{Position, Rect};
use crate::pieces::piece::ClonePiece;
use crate::pieces::piece::Piece;
use serde::Serialize;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Copy, Clone)]
pub struct SquarePiece {
    origin: Position<i32>,
}

impl SquarePiece {
    pub fn new(x: i32, y: i32) -> Self {
        SquarePiece {
            origin: Position { x, y },
        }
    }
}

impl ClonePiece for SquarePiece {
    fn clone_piece(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

impl Display for SquarePiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&serde_json::to_string(self).expect("could not convert to json"))
    }
}

impl Piece for SquarePiece {
    fn bounding_box(&self) -> Rect<i32> {
        Rect {
            x1: self.origin.x,
            y1: self.origin.y,
            x2: self.origin.x + 1,
            y2: self.origin.y + 1,
        }
    }

    fn mask(&self) -> std::vec::Vec<Position<i32>> {
        vec![
            self.origin,
            self.origin + (0, 1),
            self.origin + (1, 0),
            self.origin + (1, 1),
        ]
    }

    fn rotate_left(&mut self) {}

    fn rotate_right(&mut self) {}

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

        context.set_stroke_style(&colors::SQUARE_STROKE.into());
        context.set_fill_style(&colors::SQUARE_FILL.into());

        let bb: Rect<f64> = Rect {
            x1: origin_x,
            y1: origin_y,
            x2: origin_x + 2.0 * pixels_per_cell,
            y2: origin_y + 2.0 * pixels_per_cell,
        };

        context.begin_path();

        context.move_to(bb.x1, bb.y1);
        context.line_to(bb.x1, bb.y2);
        context.line_to(bb.x2, bb.y2);
        context.line_to(bb.x2, bb.y1);
        context.line_to(bb.x1, bb.y1);

        context.stroke();
        context.fill();
    }
}
