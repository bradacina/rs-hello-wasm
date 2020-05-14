use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;

mod board;
mod colors;
mod geometry;
mod pieces;

use board::Board;

const NUM_COLS: i32 = 10;
const NUM_ROWS: i32 = 20;
const PIXELS_PER_CELL: i32 = 30;

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
    let h = f.clone();
    let context = context();

    // setup board and its location
    let board_half_width: i32 = NUM_COLS * PIXELS_PER_CELL / 2;
    let half_screen: i32 = context.canvas().unwrap().width() as i32 / 2;

    let the_board = Rc::new(RefCell::new(Board::new(
        NUM_ROWS,
        NUM_COLS,
        PIXELS_PER_CELL,
        (half_screen - board_half_width) as f64,
        0f64,
    )));

    {
        let board1 = the_board.clone();
        let keydown_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            web_sys::console::log_2(&"got keypress".into(), &(&event).into());
            board1.borrow_mut().keydown(&event);
        }) as Box<dyn FnMut(_)>);

        let board2 = the_board.clone();
        let message_closure = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            web_sys::console::log_2(&"got message event".into(), &(&event).into());

            match event.data().as_string().unwrap().as_ref() {
                "stop" => board2.borrow_mut().pause(),
                "start" => {
                    board2.borrow_mut().resume();
                    request_animation_frame(h.borrow().as_ref().unwrap());
                }
                _ => (),
            }
        }) as Box<dyn FnMut(_)>);

        let document = window().document().unwrap();

        document
            .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
            .unwrap();
        keydown_closure.forget();

        document
            .add_event_listener_with_callback("message", message_closure.as_ref().unchecked_ref())
            .unwrap();
        message_closure.forget();
    }

    web_sys::console::log_1(&JsValue::from_serde(&the_board.clone().as_ref()).unwrap());

    // setup the request_animation_frame closure
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        let mut board = the_board.borrow_mut();
        if board.is_paused() {
            return;
        }

        board.process_input();
        board.update(time);
        draw_background(&context, time);
        board.draw(&context);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

pub fn draw_background(context: &web_sys::CanvasRenderingContext2d, time: f64) {
    context.set_fill_style(&JsValue::from_str("red"));

    let width = context.canvas().unwrap().width();
    let height = context.canvas().unwrap().height();

    context.fill_rect(0.0, 0.0, width as f64, height as f64);

    context.set_fill_style(&JsValue::from_str("black"));
    context.set_font("24px sans-serif");
    context.set_text_baseline("top");

    context.fill_text(&time.to_string(), 10.0, 10.0).unwrap();
}
