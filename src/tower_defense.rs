//! General module for executing and viewing the "tower defense" component (which is basically
//! an entire game, but the "view" is just a canvas, so that works well)

use yew::prelude::*;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, DomRect, HtmlCanvasElement, MouseEvent};

use crate::{resources::*, ECS};
use yew::services::ConsoleService;

pub(crate) struct TowerDefenseComponent {
    _link: ComponentLink<Self>,
    model: ECS,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum TDMessage {
    MousedOver { x: i32, y: i32 },
    MouseExit,
}

struct CanvasState {
    bounding_rect: DomRect,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

fn with_canvas<T, F: FnOnce(&mut CanvasState) -> T>(f: F) -> T {
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

const X_MIN: i32 = -10; // incl
const X_MAX: i32 = 11; // excl

const Y_MIN: i32 = -10;
const Y_MAX: i32 = 11;

fn tile_width(canvas_state: &CanvasState) -> i32 {
    (canvas_state.canvas.width() as i32) / (X_MAX - X_MIN)
}

fn tile_height(canvas_state: &CanvasState) -> i32 {
    (canvas_state.canvas.height() as i32) / (Y_MAX - Y_MIN)
}

fn coords_to_tile(x: i32, y: i32, canvas_state: &CanvasState) -> Option<(i32, i32)> {
    let w = tile_width(canvas_state);
    let tile_x = safe_div(x, w, 1)? + X_MIN;

    let h = tile_height(canvas_state);
    let tile_y = safe_div(y, h, 1)? + Y_MIN;

    Some((tile_x, tile_y))
}

fn safe_div(amt: i32, div: i32, tol: i32) -> Option<i32> {
    if tol < 0 {
        return safe_div(amt, div, -tol);
    }

    if div <= 0 {
        // TODO: logging about bad inputs
        return None;
    }

    let m = amt.rem_euclid(div);
    if m < tol || div - m - 1 < tol {
        return None;
    }

    let out = amt.div_euclid(div);
    Some(out)
}

impl TowerDefenseComponent {
    fn draw_canvas(&self) {
        with_canvas(|canvas_state| {
            canvas_state
                .context
                .set_global_composite_operation("source-over")
                .expect("Setting GCO should work");

            canvas_state.context.set_fill_style(&JsValue::from("white"));

            canvas_state.context.clear_rect(
                0.,
                0.,
                canvas_state.canvas.width() as f64 + 2.,
                canvas_state.canvas.height() as f64 + 2.,
            );

            let (_world, resources) = &*self.model.lock().unwrap();
            let map = resources.get::<Map>().unwrap();

            let tile_width = tile_width(canvas_state);

            let tile_height = tile_height(canvas_state);

            let black = JsValue::from("#000000");

            for x in X_MIN..X_MAX {
                for y in Y_MIN..Y_MAX {
                    let tile: Tile = map.get_tile(x, y);

                    let x_left = (x - X_MIN) * tile_width;
                    let y_top = (y - Y_MIN) * tile_height;

                    let color = match tile {
                        Tile::Open => JsValue::from("#70e0e0"),
                        Tile::Wall => JsValue::from("#008050"),
                        Tile::Spawn => JsValue::from("#ff1587"),
                        Tile::Core => JsValue::from("#1584ff"),
                    };

                    canvas_state.context.set_fill_style(&color);

                    canvas_state.context.fill_rect(
                        x_left as f64,
                        y_top as f64,
                        tile_width as f64,
                        tile_height as f64,
                    );
                }
            }

            for x in X_MIN..X_MAX {
                for y in Y_MIN..Y_MAX {
                    let x_left = (x - X_MIN) * tile_width;
                    let y_top = (y - Y_MIN) * tile_height;

                    canvas_state.context.set_stroke_style(&black);

                    canvas_state.context.stroke_rect(
                        x_left as f64 + 0.5,
                        y_top as f64 + 0.5,
                        tile_width as f64,
                        tile_height as f64,
                    );
                }
            }
        });
    }
}

#[derive(Properties, Clone)]
pub(crate) struct TDProps {
    pub(crate) ecs: ECS,
}

impl Component for TowerDefenseComponent {
    type Message = TDMessage;
    type Properties = TDProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            _link: link,
            model: props.ecs,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            TDMessage::MousedOver { x, y } => {
                let mouse_over_state = {
                    if let Some((tile_x, tile_y)) = with_canvas(|cs| coords_to_tile(x, y, cs)) {
                        TdMouseOver::MousedOver {
                            x: tile_x,
                            y: tile_y,
                        }
                    } else {
                        TdMouseOver::None
                    }
                };

                ConsoleService::log(&format!("Mouse on {:?}", mouse_over_state));

                let (_, r) = &mut *self.model.lock().unwrap();
                *r.get_mut_or_default::<TdMouseOver>() = mouse_over_state;

                false
            }
            TDMessage::MouseExit => {
                let (_, r) = &mut *self.model.lock().unwrap();
                *r.get_mut_or_default::<TdMouseOver>() = TdMouseOver::None;
                ConsoleService::log(&format!("Mouse leave"));
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        // we never "re-draw" like this; we use imperative code to
        // draw stuff on the canvas, instead
        self.draw_canvas();
        false
    }

    fn view(&self) -> Html {
        let hover_cb = self._link.callback(|mouse_event: MouseEvent| {
            let (x, y) = with_canvas(|cs| {
                let x = mouse_event.client_x() - cs.bounding_rect.left() as i32;
                let y = mouse_event.client_y() - cs.bounding_rect.top() as i32;

                (x, y)
            });

            TDMessage::MousedOver { x, y }
        });

        let leave_cb = self._link.callback(|_: MouseEvent| TDMessage::MouseExit);

        // TODO: find a way to set these attributes in css? note canvas.width and canvas.style.width are
        // inequivalent and problematic
        html! {
            <canvas id="td-canvas" onmousemove=hover_cb onmouseleave=leave_cb />
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        with_canvas(|cs| {
            let style_w = cs.bounding_rect.width() as u32;
            let style_h = cs.bounding_rect.height() as u32;

            let actual_w = cs.canvas.width();
            if style_w != actual_w {
                cs.canvas.set_width(style_w);
            }

            let actual_h = cs.canvas.height();
            if style_h != actual_h {
                cs.canvas.set_height(style_h);
            }
        })
    }
}
