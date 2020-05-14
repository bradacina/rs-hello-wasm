use serde::Serialize;
use std::ops::Add;

#[derive(Debug, Serialize)]
pub struct Rect<T> {
    pub x1: T,
    pub y1: T,
    pub x2: T,
    pub y2: T,
}

#[derive(Debug, Serialize, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Add<(i32, i32)> for Position {
    type Output = Self;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Position {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl From<Position> for (i32, i32) {
    fn from(position: Position) -> Self {
        (position.x, position.y)
    }
}
