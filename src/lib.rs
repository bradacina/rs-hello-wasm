use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;

mod board;
mod colors;
mod pieces;

use board::Board;

const NUM_COLS: u32 = 10;
const NUM_ROWS: u32 = 20;
const PIXELS_PER_CELL: u32 = 30;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn context() -> web_sys::CanvasRenderingContext2d {
    let window = window();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context_with_context_options(
            "2d",
            &JsValue::from_serde(&vec![("alpha", false)]).unwrap(),
        )
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    web_sys::console::log_2(
        &JsValue::from(canvas.width()),
        &JsValue::from(canvas.height()),
    );

    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

    return context;
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let context = context();

    // setup board and its location
    let board_half_width: u32 = NUM_COLS * PIXELS_PER_CELL / 2;
    let half_screen: u32 = context.canvas().unwrap().width() / 2;

    let board = Board::new(
        NUM_ROWS,
        NUM_COLS,
        PIXELS_PER_CELL,
        (half_screen - board_half_width) as f64,
        0f64,
    );

    web_sys::console::log_1(&JsValue::from_serde(&board).unwrap());

    // setup request animation closure
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        draw(&context, time);
        board.draw(&context);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

pub fn draw(context: &web_sys::CanvasRenderingContext2d, time: f64) {
    static old_time: f64 = 0f64;

    context.set_fill_style(&JsValue::from_str("red"));

    let width = context.canvas().unwrap().width();
    let height = context.canvas().unwrap().height();

    context.fill_rect(0.0, 0.0, width as f64, height as f64);

    context.set_fill_style(&JsValue::from_str("black"));
    context.set_font("24px sans-serif");
    context.set_text_baseline("top");

    context.fill_text(&time.to_string(), 10.0, 10.0).unwrap();
}
