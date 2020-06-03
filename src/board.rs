use crate::animations::{Animation, Flash};
use crate::colors;
use crate::geometry::{Position, Rect};
use crate::pieces::{
    LPieceLeft, LPieceRight, LinePiece, Piece, SquarePiece, TrianglePiece, ZPieceLeft, ZPieceRight,
};
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

const DROP_TIME: f64 = 50000f64;
const ROW_SCORE: u32 = 1000;

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
    pixel_width: f64,  // width of board in pixels
    pixel_height: f64, // height of board in pixels

    origin_x: f64, // x coord on context where the board resides
    origin_y: f64, // y coord on context where the board resides

    active_piece: Box<dyn Piece>, // the piece that the player is manipulating

    keys: Vec<String>, // a buffer of key presses since we last processed input

    is_paused: bool,
    paused_rendered: bool,
    paused_at: f64, // the game time when the game was paused - used to calculate reminder of last_drop
    last_processed_tick: f64, // the last game time when we performed an update
    is_game_over: bool,
    game_over_rendered: bool,
    pub score: u32,

    last_drop: f64, // what was the game time when the active_piece was last dropped by 1 square

    animations: Vec<Box<dyn Animation>>,
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

            active_piece: Box::new(LinePiece::new(2, 15)),
            keys: Vec::with_capacity(4),
            is_paused: Default::default(),
            paused_rendered: false,
            paused_at: 0f64,
            last_processed_tick: 0f64,
            last_drop: 0f64,
            animations: Vec::with_capacity(40),
            score: 0,
            is_game_over: false,
            game_over_rendered: false,
        }
    }

    pub fn keydown(&mut self, event: &web_sys::KeyboardEvent) {
        if self.is_paused || self.is_game_over {
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

    /// Sends the active piece to the bottom
    fn place_piece(&mut self) {
        let mask = self.project_piece(&self.active_piece);
        for item in mask {
            self.cells[item.y as usize][item.x as usize] = true;
        }

        self.new_active_piece();
    }

    /// Projects a piece down to the lowest point it can reach
    fn project_piece(&self, piece: &Box<dyn Piece>) -> Vec<Position<i32>> {
        let mut placed_piece = piece.clone();
        let (origin_x, origin_y) = placed_piece.get_origin().into();
        // todo: optimize this by projecting the mask down on the board
        // until we encounter a piece or the bottom

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

    /// Is the bounding box bb of a piece inside the board
    fn is_inside_board(&self, bb: &Rect<i32>) -> bool {
        if bb.x1 < 0 || bb.x2 >= self.cols || bb.y1 < 0 || bb.y2 >= self.rows {
            return false;
        }

        return true;
    }

    /// Is the mask of a piece colding with any existing pieces on the board
    fn is_colliding(&self, mask: &Vec<Position<i32>>) -> bool {
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
        self.last_processed_tick = time;
        if self.is_paused || self.is_game_over {
            return;
        }

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
                self.score += ROW_SCORE;
            }
        }

        for to_remove in complete_rows {
            for i in 0..10 {
                self.animations.push(Box::new(Flash::new(
                    self.origin_x + i as f64 * self.pixels_per_cell as f64,
                    self.origin_y + to_remove as f64 * self.pixels_per_cell as f64,
                    time,
                    1500.0,
                )));
            }
            self.cells.remove(to_remove);
            self.cells
                .insert(0, (0..self.cols).map(|_| false).collect());
        }

        for animation in self.animations.iter_mut() {
            animation.update(time);
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
        let next = rng.gen_range(0, 350);
        let next_active_piece: Box<dyn Piece>;
        if next > 300 {
            next_active_piece = Box::new(TrianglePiece::new(self.cols / 2, 1));
        } else if next > 250 {
            next_active_piece = Box::new(ZPieceRight::new(self.cols / 2, 1));
        } else if next > 200 {
            next_active_piece = Box::new(ZPieceLeft::new(self.cols / 2, 1));
        } else if next > 150 {
            next_active_piece = Box::new(LinePiece::new(self.cols / 2, 1));
        } else if next > 100 {
            next_active_piece = Box::new(SquarePiece::new(self.cols / 2, 1));
        } else if next > 50 {
            next_active_piece = Box::new(LPieceLeft::new(self.cols / 2, 1));
        } else {
            next_active_piece = Box::new(LPieceRight::new(self.cols / 2, 1));
        }

        self.last_drop = 0f64;

        let mask = next_active_piece.mask();
        if self.is_colliding(&mask) {
            self.is_game_over = true;
        } else {
            self.active_piece = next_active_piece;
        }
    }

    pub fn draw(&mut self, context: &web_sys::CanvasRenderingContext2d) {
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
        if !self.is_game_over {
            let origin = self.active_piece.get_origin();
            self.active_piece.draw(
                context,
                self.origin_x + (origin.x * self.pixels_per_cell) as f64,
                self.origin_y + (origin.y * self.pixels_per_cell) as f64,
                self.pixels_per_cell as f64,
            );
        }

        // draw the projection
        if !self.is_game_over {
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

        // draw animations
        for animation in &self.animations {
            animation.draw(context, 0.0, 0.0, self.pixels_per_cell as f64);
        }

        if self.is_paused {
            self.paused_rendered = true;
            web_sys::console::log_1(&"drawing pause".into());
            context.set_fill_style(&"white".into());
            context.set_font("24px sans-serif");
            context
                .fill_text("Paused", self.relative_x(10.0), self.relative_y(10.0))
                .unwrap();
        }
        if self.is_game_over {
            self.game_over_rendered = true;
            context.set_fill_style(&"white".into());
            context.set_font("24px sans-serif");
            context
                .fill_text("Game Over", self.relative_x(10.0), self.relative_y(10.0))
                .unwrap();
        }
    }

    fn relative_x(&self, x: f64) -> f64 {
        self.origin_x + x
    }

    fn relative_y(&self, y: f64) -> f64 {
        self.origin_y + y
    }

    pub fn pause(&mut self) {
        if self.is_game_over {
            return;
        }
        self.is_paused = true;
        self.paused_at = self.last_processed_tick;
    }

    pub fn resume(&mut self) {
        if self.is_game_over {
            return;
        }
        self.is_paused = false;
        self.last_drop = self.last_processed_tick - (self.paused_at - self.last_drop);
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused && self.paused_rendered
    }

    pub fn is_game_over(&self) -> bool {
        self.is_game_over && self.game_over_rendered
    }
}
