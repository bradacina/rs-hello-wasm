use crate::animations::animation::Animation;
use crate::colors;
use crate::geometry::Position;
use core::fmt::Display;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct CellCompleteAnimation {
    start_time: f64,
    time_since_start: f64,
    duration: f64,
    origin: Position<f64>,
    finished: bool,
    started: bool,
}

impl CellCompleteAnimation {
    pub fn new(origin: Position<f64>, start_tick: f64, duration: f64) -> Self {
        CellCompleteAnimation {
            start_time: start_tick,
            duration: duration,
            time_since_start: 0.0,
            origin: origin,
            finished: false,
            started: false,
        }
    }
}

impl Display for CellCompleteAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&serde_json::to_string(self).expect("could not convert to json"))
    }
}

impl Animation for CellCompleteAnimation {
    fn update(&mut self, tick: f64) {
        if self.finished {
            return;
        }

        if !self.started && tick > self.start_time {
            self.started = true;
        }

        if tick > (self.start_time + self.duration) {
            self.finished = true;
            return;
        }

        if self.started && !self.finished {
            self.time_since_start = tick - self.start_time;
        }
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        _: f64,
        _: f64,
        pixels_per_cell: f64,
    ) {
        if self.finished || !self.started {
            return;
        }

        context
            .set_line_dash(&JsValue::from_serde(&([] as [i32; 0])).unwrap())
            .unwrap();

        context.set_stroke_style(&colors::CELL_COMPLETE_COLOR.into());
        context.set_fill_style(&colors::CELL_COMPLETE_FILL.into());

        context.begin_path();

        context
            .ellipse(
                self.origin.x,
                self.origin.y,
                pixels_per_cell,
                pixels_per_cell,
                0.0,
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();

        context.stroke();
        context.fill();
    }
}
