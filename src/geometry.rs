use serde::Serialize;

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
