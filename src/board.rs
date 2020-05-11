use crate::colors;
use crate::geometry::Rect;
use crate::pieces::bar::Bar;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Board {
    rows: i32,
    cols: i32,
    pixels_per_cell: i32,
    pixel_width: f64,
    pixel_height: f64,
    origin_x: f64, // x coord on context where the board resides
    origin_y: f64, // y coord on context where the board resides

    active_piece: Bar,

    keys: Vec<String>,
}

impl Board {
    pub fn new(rows: i32, cols: i32, pixels_per_cell: i32, origin_x: f64, origin_y: f64) -> Self {
        Board {
            rows,
            cols,
            pixels_per_cell,
            pixel_width: (cols * pixels_per_cell) as f64,
            pixel_height: (rows * pixels_per_cell) as f64,
            origin_x,
            origin_y,

            active_piece: Bar::new(2, 2),
            keys: Vec::with_capacity(4),
        }
    }

    pub fn set_origin(&mut self, x: f64, y: f64) {
        self.origin_x = x;
        self.origin_y = y;
    }

    pub fn keydown(&mut self, event: &web_sys::KeyboardEvent) {
        self.keys.push(event.code());
    }

    pub fn process_input(&mut self) {
        let mut cp: Vec<String> = Vec::with_capacity(4);
        cp.append(&mut self.keys);

        for key in cp {
            match key.as_ref() {
                "ArrowUp" => self.rotate_left(),
                "ArrowDown" => self.rotate_right(),
                "ArrowLeft" => self.move_left(),
                "ArrowRight" => self.move_right(),
                _ => (),
            }
        }
    }

    fn is_inside_board(&self, bb: &Rect<i32>) -> bool {
        if bb.x1 < 0 || bb.x2 >= self.cols || bb.y1 < 0 || bb.y2 > self.rows {
            return false;
        }

        return true;
    }

    fn rotate_left(&mut self) {
        let mut attempt = self.active_piece;
        attempt.rotate_left();

        let mut bb = attempt.bounding_box();

        if !self.is_inside_board(&bb) {
            while bb.x2 >= self.cols {
                attempt.move_left();
                bb = attempt.bounding_box();
            }

            while bb.x1 < 0 {
                attempt.move_right();
                bb = attempt.bounding_box();
            }

            if !self.is_inside_board(&bb) {
                return;
            }
        }

        // todo: check collision with other pieces

        for col in bb.x1..bb.x2 {
            for row in bb.y1..bb.y2 {}
        }

        self.active_piece = attempt;
    }

    fn rotate_right(&mut self) {
        let mut attempt = self.active_piece;
        attempt.rotate_right();
        let bb = attempt.bounding_box();
        if !self.is_inside_board(&bb) {
            return;
        }

        // todo: check collision with other pieces
        for col in bb.x1..bb.x2 {
            for row in bb.y1..bb.y2 {}
        }

        self.active_piece = attempt;
    }

    pub fn move_left(&mut self) {
        let mut attempt = self.active_piece;
        attempt.move_left();

        let bb = attempt.bounding_box();
        if !self.is_inside_board(&bb) {
            return;
        }

        // todo: check collision with other pieces on the board

        self.active_piece = attempt;
    }

    pub fn move_right(&mut self) {
        let mut attempt = self.active_piece;
        attempt.move_right();

        let bb = attempt.bounding_box();
        if !self.is_inside_board(&bb) {
            return;
        }

        // todo: check collision with other pieces on the board

        self.active_piece = attempt;
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

        let origin = self.active_piece.get_origin();
        self.active_piece.draw(
            context,
            self.origin_x + (origin.x * self.pixels_per_cell) as f64,
            self.origin_y + (origin.y * self.pixels_per_cell) as f64,
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
