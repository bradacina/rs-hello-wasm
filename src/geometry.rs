use serde::Serialize;
use std::ops::Add;

#[derive(Serialize)]
pub struct Rect<T> {
    pub x1: T,
    pub y1: T,
    pub x2: T,
    pub y2: T,
}

#[derive(Serialize, Copy, Clone)]
pub struct Position<R> {
    pub x: R,
    pub y: R,
}

impl<R> Add<(R, R)> for Position<R> where R: std::ops::Add<Output=R> {
    type Output = Self;
    fn add(self, rhs: (R, R)) -> Self::Output {
        Position {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl<R> From<Position<R>> for (R, R) {
    fn from(position: Position<R>) -> Self {
        (position.x, position.y)
    }
}
