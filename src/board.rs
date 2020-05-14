use crate::colors;
use crate::geometry::{Position, Rect};
use crate::pieces::bar::Bar;
use serde::Serialize;
use wasm_bindgen::prelude::*;

const DROP_TIME: f64 = 500f64;

enum Rotation {
    Left,
    Right
}

enum Move {
    Left,
    Right
}

#[derive(Debug, Serialize)]
pub struct Board {
    rows: i32,
    cols: i32,
    cells: Vec<Vec<bool>>, // indexes are [col][row]
    pixels_per_cell: i32,
    pixel_width: f64,
    pixel_height: f64,
    origin_x: f64, // x coord on context where the board resides
    origin_y: f64, // y coord on context where the board resides

    active_piece: Bar,

    keys: Vec<String>,

    is_paused: bool,

    last_drop: f64,
}

impl Board {
    pub fn new(rows: i32, cols: i32, pixels_per_cell: i32, origin_x: f64, origin_y: f64) -> Self {
        let cells: Vec<Vec<bool>> = (0..cols)
            .map(|_| (0..rows).map(|_| false).collect())
            .collect();

        Board {
            rows,
            cols,
            cells: cells,
            pixels_per_cell,
            pixel_width: (cols * pixels_per_cell) as f64,
            pixel_height: (rows * pixels_per_cell) as f64,
            origin_x,
            origin_y,

            active_piece: Bar::new(2, 15),
            keys: Vec::with_capacity(4),
            is_paused: Default::default(),
            last_drop: 0f64,
        }
    }

    pub fn set_origin(&mut self, x: f64, y: f64) {
        self.origin_x = x;
        self.origin_y = y;
    }

    pub fn keydown(&mut self, event: &web_sys::KeyboardEvent) {
        if self.is_paused {
            return;
        }

        self.keys.push(event.code());
    }

    pub fn process_input(&mut self) {
        let mut cp: Vec<String> = Vec::with_capacity(4);
        cp.append(&mut self.keys);

        for key in cp {
            match key.as_ref() {
                "ArrowUp" => self.rotate(Rotation::Left),
                "ArrowDown" => self.rotate(Rotation::Right),
                "ArrowLeft" => self.move_sideways(Move::Left),
                "ArrowRight" => self.move_sideways(Move::Right),
                "Enter" => self.place_piece(),
                _ => (),
            }
        }
    }

    fn place_piece(&mut self) {
        let mask = self.project_piece(&self.active_piece);
        for item in mask {
            self.cells[item.x as usize][item.y as usize] = true;
        }

        self.new_active_piece();
    }

    /// projects a piece down to the lowest point it can reach
    fn project_piece(&self, piece: &Bar) -> Vec<Position> {
        let mut placed_piece = *piece;
        let (origin_x, origin_y) = placed_piece.get_origin().into();

        for y in (origin_y..self.rows).rev() {
            placed_piece.set_origin(origin_x, y);

            let bb = placed_piece.bounding_box();

            if !self.is_inside_board(&bb) {
                continue;
            }

            let mask = placed_piece.mask();

            if self.is_colliding(&mask) {
                continue;
            }

            break;
        }

        return placed_piece.mask();
    }

    fn is_inside_board(&self, bb: &Rect<i32>) -> bool {
        if bb.x1 < 0 || bb.x2 >= self.cols || bb.y1 < 0 || bb.y2 >= self.rows {
            return false;
        }

        return true;
    }

    fn is_colliding(&self, mask: &Vec<Position>) -> bool {
        for item in mask {
            if self.cells[item.x as usize][item.y as usize] {
                return true;
            }
        }

        false
    }

    fn rotate(&mut self, rotation: Rotation) {
        let mut attempt = self.active_piece;

        match rotation {
            Rotation::Left => attempt.rotate_left(),
            Rotation::Right => attempt.rotate_right()
        }

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

        let mask = attempt.mask();

        if self.is_colliding(&mask) {
            return;
        }

        self.active_piece = attempt;
    }

    fn move_sideways(&mut self, direction: Move) {
        let mut attempt = self.active_piece;

        match direction {
            Move::Left => attempt.move_left(),
            Move::Right => attempt.move_right()
        }

        let bb = attempt.bounding_box();
        if !self.is_inside_board(&bb) {
            return;
        }

        let mask = attempt.mask();
        if self.is_colliding(&mask) {
            return;
        }

        self.active_piece = attempt;
    }

    pub fn update(&mut self, time: f64) {
        if time - self.last_drop > DROP_TIME {
            self.try_drop();
            self.last_drop = time;
        }
    }

    fn try_drop(&mut self) {
        let (x, y) = self.active_piece.get_origin().into();

        self.active_piece.set_origin(x, y + 1);

        let bb = self.active_piece.bounding_box();
        let mask = self.active_piece.mask();
        if self.is_inside_board(&bb) && !self.is_colliding(&mask) {
            return;
        }

        // undo piece drop and place piece
        self.active_piece.set_origin(x, y);
        let mask = self.active_piece.mask();
        for item in mask {
            self.cells[item.x as usize][item.y as usize] = true;
        }

        self.new_active_piece();
    }

    fn new_active_piece(&mut self) {
        self.active_piece = Bar::new(self.cols / 2, 1);
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

        // draw pieces on the board
        context.set_fill_style(&"blue".into());

        for (x, col) in self.cells.iter().enumerate() {
            for (y, val) in col.iter().enumerate() {
                if *val {
                    context.fill_rect(
                        self.relative_x((x * self.pixels_per_cell as usize) as f64),
                        self.relative_y((y * self.pixels_per_cell as usize) as f64),
                        self.pixels_per_cell as f64,
                        self.pixels_per_cell as f64,
                    );
                }
            }
        }

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

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }
}
