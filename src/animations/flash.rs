use crate::animations::animation::Animation;
use crate::geometry::Position;
use crate::colors;
use serde::Serialize;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

const TOGGLE_AFTER: f64 = 150.0;

#[derive(Serialize)]
pub struct Flash {
    origin: Position<f64>,
    last_toggle: f64,
    start_time: f64,
    is_finished: bool,
    is_running: bool,
    duration: f64,
    flash_enabled: bool,
}

impl Flash {
    pub fn new(x: f64, y: f64, start_time: f64, duration: f64) -> Self {
        Flash {
            origin: Position { x: x, y: y },
            last_toggle: 0.0,
            start_time: start_time,
            duration: duration,
            is_finished: false,
            is_running: false,
            flash_enabled: false,
        }
    }
}

impl Display for Flash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&serde_json::to_string(self).expect("could not convert to json"))
    }
}

impl Animation for Flash {
    fn update(&mut self, tick: f64) {
        if self.is_finished {
            return;
        }

        if tick > self.start_time + self.duration {
            self.is_finished = true;
            return;
        }

        if tick > self.start_time && !self.is_running {
            self.is_running = true;
            self.flash_enabled = true;
            self.last_toggle = tick;
        }

        if !self.is_running {
            return;
        }

        if self.last_toggle + TOGGLE_AFTER < tick {
            self.last_toggle = tick;
            self.flash_enabled = !self.flash_enabled;
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, _: f64, _: f64, pixels_per_cell: f64) {
        if self.is_finished || !self.is_running {
            return
        }

        if !self.flash_enabled {
            return
        }

        context
            .set_line_dash(&JsValue::from_serde(&([] as [i32; 0])).unwrap())
            .unwrap();

        context.set_stroke_style(&colors::CELL_COMPLETE_COLOR.into());
        context.set_fill_style(&colors::CELL_COMPLETE_FILL.into());

        context.fill_rect(self.origin.x, self.origin.y, pixels_per_cell, pixels_per_cell);
    }

    fn is_finished(&self) -> bool {
        self.is_finished
    }
}
