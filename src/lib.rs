use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use serde::Serialize;
use std::rc::Rc;
use std::cell::RefCell;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
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

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time:f64| {
        draw(time);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));

   request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

pub fn draw(time: f64) {
    let width = 800f64;
    let height = 600f64;

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context_with_context_options("2d", &JsValue::from_serde(&vec![("alpha", false)]).unwrap())
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.set_fill_style(&JsValue::from_str("red"));
    context.fill_rect(0.0 ,0.0, width, height);

    context.set_fill_style(&JsValue::from_str("black"));

    context.fill_text(&time.to_string(), 10.0, 10.0).unwrap();

    // web_sys::console::log_1(&"hello".into());
    
}

