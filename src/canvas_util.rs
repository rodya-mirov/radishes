use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, DomRect, HtmlCanvasElement};

#[derive(Clone)]
pub struct CanvasState {
    pub bounding_rect: DomRect,
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
}

pub fn with_canvas<T, F: FnOnce(&mut CanvasState) -> T>(f: F) -> T {
    let window = web_sys::window().expect("Window should exist");

    let document: web_sys::Document = window.document().expect("Document should exist");

    let canvas = document.get_element_by_id("td-canvas").unwrap();

    let bounding_rect: DomRect = canvas.get_bounding_client_rect();

    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context.set_transform(1., 0., 0., 1., 0., 0.).unwrap();

    context.save();

    let mut state = CanvasState {
        bounding_rect,
        canvas,
        context,
    };

    let t = f(&mut state);

    state.context.restore();

    t
}
