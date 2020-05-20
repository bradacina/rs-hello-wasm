use crate::colors;
use crate::geometry::{Position, Rect};
use crate::pieces::{Bar, LPieceLeft, LPieceRight, Piece, Square, ZPieceLeft, ZPieceRight};
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

const DROP_TIME: f64 = 50000f64;

enum Rotation {
    Left,
    Right,
}

enum Move {
    Left,
    Right,
}

#[derive(Serialize)]
pub struct Board {
    rows: i32,
    cols: i32,
    cells: Vec<Vec<bool>>, // indexes are [row][col]
    pixels_per_cell: i32,
    pixel_width: f64,
    pixel_height: f64,
    origin_x: f64, // x coord on context where the board resides
    origin_y: f64, // y coord on context where the board resides

    active_piece: Box<dyn Piece>,

    keys: Vec<String>,

    is_paused: bool,

    last_drop: f64,
}

impl Board {
    pub fn new(rows: i32, cols: i32, pixels_per_cell: i32, origin_x: f64, origin_y: f64) -> Self {
        let cells: Vec<Vec<bool>> = (0..rows)
            .map(|_| (0..cols).map(|_| false).collect())
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

            active_piece: Box::new(Bar::new(2, 15)),
            keys: Vec::with_capacity(4),
            is_paused: Default::default(),
            last_drop: 0f64,
        }
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

    // sends the active piece to the bottom
    fn place_piece(&mut self) {
        let mask = self.project_piece(&self.active_piece);
        for item in mask {
            self.cells[item.y as usize][item.x as usize] = true;
        }

        self.new_active_piece();
    }

    // projects a piece down to the lowest point it can reach
    fn project_piece(&self, piece: &Box<dyn Piece>) -> Vec<Position> {
        let mut placed_piece = piece.clone();
        let (origin_x, origin_y) = placed_piece.get_origin().into();
        // todo: optimize this by projecting the mask down on the board
        // until we encounter a piece or the edge

        for y in origin_y..self.rows {
            placed_piece.set_origin(origin_x, y);

            let bb = placed_piece.bounding_box();
            let mask = placed_piece.mask();

            if self.is_inside_board(&bb) && !self.is_colliding(&mask) {
                continue;
            }

            assert!(y - 1 >= origin_y);

            placed_piece.set_origin(origin_x, y - 1);

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
            if self.cells[item.y as usize][item.x as usize] {
                return true;
            }
        }

        false
    }

    fn rotate(&mut self, rotation: Rotation) {
        let mut attempt = self.active_piece.clone();

        match rotation {
            Rotation::Left => attempt.rotate_left(),
            Rotation::Right => attempt.rotate_right(),
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
        let mut attempt = self.active_piece.clone();

        match direction {
            Move::Left => attempt.move_left(),
            Move::Right => attempt.move_right(),
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
        if self.last_drop == 0f64 {
            self.last_drop = time;
        } else if time - self.last_drop > DROP_TIME {
            self.try_drop();
            self.last_drop = time;
        }

        // check for completed rows
        let mut complete_rows: Vec<usize> = Vec::with_capacity(20);
        for y in (0..(self.rows as usize)).rev() {
            let is_complete = self.cells[y].iter().all(|val| *val);
            if is_complete {
                complete_rows.push(y);
            }
        }

        for to_remove in complete_rows {
            self.cells.remove(to_remove);
            self.cells
                .insert(0, (0..self.cols).map(|_| false).collect());
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
            self.cells[item.y as usize][item.x as usize] = true;
        }

        self.new_active_piece();
    }

    fn new_active_piece(&mut self) {
        let mut rng = thread_rng();
        let next = rng.gen_range(0, 300);
        if next > 250 {
            self.active_piece = Box::new(ZPieceRight::new(self.cols / 2, 1));
        } else if next > 200 {
            self.active_piece = Box::new(ZPieceLeft::new(self.cols / 2, 1));
        } else if next > 150 {
            self.active_piece = Box::new(Bar::new(self.cols / 2, 1));
        } else if next > 100 {
            self.active_piece = Box::new(Square::new(self.cols / 2, 1));
        } else if next > 50 {
            self.active_piece = Box::new(LPieceLeft::new(self.cols / 2, 1));
        } else {
            self.active_piece = Box::new(LPieceRight::new(self.cols / 2, 1));
        }
        self.last_drop = 0f64;
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        // draw border
        context.set_stroke_style(&colors::BORDER.into());
        context.set_line_width(1.0);
        context
            .set_line_dash(&JsValue::from_serde(&([] as [i32; 0])).unwrap())
            .unwrap();
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
        context
            .set_line_dash(&JsValue::from_serde(&([] as [i32; 0])).unwrap())
            .unwrap();

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

        for (y, row) in self.cells.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
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

        // draw active piece
        let origin = self.active_piece.get_origin();
        self.active_piece.draw(
            context,
            self.origin_x + (origin.x * self.pixels_per_cell) as f64,
            self.origin_y + (origin.y * self.pixels_per_cell) as f64,
            self.pixels_per_cell as f64,
        );

        // draw the projection
        context.set_stroke_style(&colors::PROJECTION_STROKE.into());
        context
            .set_line_dash(&JsValue::from_serde(&vec![3, 3]).unwrap())
            .unwrap();
        context.begin_path();

        let mask = self.project_piece(&self.active_piece);

        for item in mask {
            context.move_to(
                self.origin_x + (item.x * self.pixels_per_cell) as f64,
                self.origin_y + (item.y * self.pixels_per_cell) as f64,
            );
            context.line_to(
                self.origin_x + ((item.x + 1) * self.pixels_per_cell) as f64,
                self.origin_y + (item.y * self.pixels_per_cell) as f64,
            );
            context.line_to(
                self.origin_x + ((item.x + 1) * self.pixels_per_cell) as f64,
                self.origin_y + ((item.y + 1) * self.pixels_per_cell) as f64,
            );
            context.line_to(
                self.origin_x + (item.x * self.pixels_per_cell) as f64,
                self.origin_y + ((item.y + 1) * self.pixels_per_cell) as f64,
            );
            context.line_to(
                self.origin_x + (item.x * self.pixels_per_cell) as f64,
                self.origin_y + (item.y * self.pixels_per_cell) as f64,
            );
        }

        context.stroke();
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
