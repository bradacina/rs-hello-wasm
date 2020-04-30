use serde::Serialize;

use crate::colors;
use crate::pieces::bar::Bar;

#[derive(Debug, Serialize)]
pub struct Board {
    rows: u32,
    cols: u32,
    pixels_per_cell: u32,
    pixel_width: f64,
    pixel_height: f64,
    origin_x: f64, // x coord on context where the board resides
    origin_y: f64, // y coord on context where the board resides

    active_piece: Bar,
}

impl Board {
    pub fn new(rows: u32, cols: u32, pixels_per_cell: u32, origin_x: f64, origin_y: f64) -> Self {
        Board {
            rows,
            cols,
            pixels_per_cell,
            pixel_width: (cols * pixels_per_cell) as f64,
            pixel_height: (rows * pixels_per_cell) as f64,
            origin_x,
            origin_y,

            active_piece: Bar::new(2, 2),
        }
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        // draw border
        context.set_stroke_style(&colors::BORDER.into());
        context.set_line_width(1.0);
        context.begin_path();
        context.move_to(self.relative_x(0.0), self.relative_y(0.0));
        context.line_to(self.relative_x(self.pixel_width), self.relative_y(0.0));
        context.line_to(
            self.relative_x(self.pixel_width),
            self.relative_y(self.pixel_height),
        );
        context.line_to(self.relative_x(0.0), self.relative_y(self.pixel_height));
        context.line_to(self.relative_x(0.0), self.relative_y(0.0));

        context.stroke();

        // draw cross hatch
        context.begin_path();

        for i in 1..self.cols {
            context.move_to(
                self.relative_x((i * self.pixels_per_cell).into()),
                self.origin_y,
            );

            context.line_to(
                self.relative_x((i * self.pixels_per_cell).into()),
                self.relative_y(self.pixel_height),
            );
        }

        for j in 1..self.rows {
            context.move_to(
                self.origin_x,
                self.relative_y((j * self.pixels_per_cell).into()),
            );

            context.line_to(
                self.relative_x(self.pixel_width),
                self.relative_y((j * self.pixels_per_cell).into()),
            );
        }

        context.stroke();

        // draw pieces

        self.active_piece.draw(
            context,
            self.origin_x + (self.active_piece.origin.x * self.pixels_per_cell) as f64,
            self.origin_y + (self.active_piece.origin.y * self.pixels_per_cell) as f64,
            self.pixels_per_cell as f64,
        );
    }

    fn relative_x(&self, x: f64) -> f64 {
        self.origin_x + x
    }

    fn relative_y(&self, y: f64) -> f64 {
        self.origin_y + y
    }
}
