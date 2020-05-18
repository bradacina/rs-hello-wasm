use crate::geometry::{Position, Rect};
use serde::Serialize;

pub trait Piece: ClonePiece + std::fmt::Display {
    fn bounding_box(&self) -> Rect<i32>;
    fn mask(&self) -> Vec<Position>;
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn set_origin(&mut self, x: i32, y: i32);
    fn get_origin(&self) -> Position;
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        origin_x: f64,
        origin_y: f64,
        pixels_per_cell: f64,
    );
}

pub trait ClonePiece {
    fn clone_piece(&self) -> Box<dyn Piece>;
}

impl Clone for Box<dyn Piece> {
    fn clone(&self) -> Self {
        return self.clone_piece();
    }
}

impl Serialize for Box<dyn Piece> {
    fn serialize<S>(&self, s: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.collect_str(self)
    }
}
