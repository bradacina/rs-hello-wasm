use serde::Serialize;

pub trait Animation: std::fmt::Display {
    fn update(&mut self, tick: f64);
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        origin_x: f64,
        origin_y: f64,
        pixels_per_cell: f64,
    );
    fn is_finished(&self) -> bool;
}

impl Serialize for Box<dyn Animation> {
    fn serialize<S>(&self, s: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.collect_str(self)
    }
}