//! General module for executing and viewing the "tower defense" component (which is basically
//! an entire game, but the "view" is just a canvas, so that works well)

use yew::prelude::*;
use yew::services::{ConsoleService, IntervalService, Task};

use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::ECS;

pub(crate) struct TowerDefenseComponent {
    _link: ComponentLink<Self>,
    td_state: TDState,
    model: ECS,
}

struct TDState {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum TDMessage {}

impl TowerDefenseComponent {
    fn draw_canvas(&self) {
        let document: web_sys::Document = web_sys::window()
            .expect("Window should exist")
            .document()
            .expect("Document should exist");

        let canvas = document.get_element_by_id("td-canvas").unwrap();

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

        context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

        // lol
        context.set_fill_style(&wasm_bindgen::JsValue::from(&format!(
            "rgb({}, {}, {})",
            self.td_state.r, self.td_state.g, self.td_state.b
        )));

        context.fill_rect(
            10.,
            10.,
            canvas.width() as f64 - 10.,
            canvas.height() as f64 - 10.,
        );
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
            td_state: TDState { r: 255, g: 0, b: 0 },
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
