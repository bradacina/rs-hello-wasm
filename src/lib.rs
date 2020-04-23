use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::Serialize;

#[wasm_bindgen]
pub fn hello() -> i32 {
    2+2
}

#[derive(Serialize)]
struct Val {
    hello: bool,
    something: i32,
    complex: String
}

#[wasm_bindgen]
pub fn val() -> JsValue {

    JsValue::from_serde(&
        ("hello", true,
        "something", 123,
        "complex", "hello there"
    )).unwrap()
}

#[wasm_bindgen]
pub fn draw() {
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
}