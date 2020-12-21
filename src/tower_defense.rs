//! General module for executing and viewing the "tower defense" component (which is basically
//! an entire game, but the "view" is just a canvas, so that works well)

use yew::prelude::*;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{resources::*, ECS};

pub(crate) struct TowerDefenseComponent {
    _link: ComponentLink<Self>,
    model: ECS,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum TDMessage {}

fn with_canvas<F: FnOnce(&mut HtmlCanvasElement, &mut CanvasRenderingContext2d)>(f: F) {
    let window = web_sys::window().expect("Window should exist");

    let document: web_sys::Document = window.document().expect("Document should exist");

    let canvas = document.get_element_by_id("td-canvas").unwrap();

    let mut canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let mut context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context.save();

    f(&mut canvas, &mut context);

    context.restore();
}

impl TowerDefenseComponent {
    fn draw_canvas(&self) {
        with_canvas(|canvas, context| {

            context
                .set_global_composite_operation("source-over")
                .expect("Setting GCO should work");

            context.set_fill_style(&JsValue::from("white"));

            context.clear_rect(
                0.,
                0.,
                canvas.width() as f64 + 2.,
                canvas.height() as f64 + 2.,
            );

            let (_world, resources) = &*self.model.lock().unwrap();
            let map = resources.get::<Map>().unwrap();

            const X_MIN: i32 = -10; // incl
            const X_MAX: i32 = 11; // excl
            let tile_width: i32 = (canvas.width() as i32) / (X_MAX - X_MIN);

            const Y_MIN: i32 = -10;
            const Y_MAX: i32 = 11;
            let tile_height: i32 = (canvas.height() as i32) / (Y_MAX - Y_MIN);

            let black = JsValue::from("#000000");

            for x in X_MIN..X_MAX {
                for y in Y_MIN..Y_MAX {
                    let tile: Tile = map.get_tile(x, y);

                    let x_left = (x - X_MIN) * tile_width;
                    let y_top = (y - Y_MIN) * tile_height;

                    let color = match tile {
                        Tile::Open => JsValue::from("#70e0e0"),
                        Tile::Wall => JsValue::from("#008050"),
                    };

                    context.set_fill_style(&color);

                    context.fill_rect(
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

                    context.set_stroke_style(&black);

                    context.stroke_rect(
                        x_left as f64 + 0.5,
                        y_top as f64 + 0.5,
                        tile_width as f64,
                        tile_height as f64 ,
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
        match msg {}
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        // we never "re-draw" like this; we use imperative code to
        // draw stuff on the canvas, instead
        self.draw_canvas();
        false
    }

    fn view(&self) -> Html {
        html! {
            <canvas id="td-canvas" />
        }
    }
}
