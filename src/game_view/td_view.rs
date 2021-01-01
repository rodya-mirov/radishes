//! General module for executing and viewing the "tower defense" component (which is basically
//! an entire game, but the "view" is just a canvas, so that works well)

use std::sync::Arc;

use yew::prelude::*;

use wasm_bindgen::JsValue;
use web_sys::{KeyboardEvent, MouseEvent};

use crate::{assets::Assets, canvas_util::with_canvas, components::*, resources::*, tile_helpers::coords_to_tile_buffered, ECS};

pub(crate) struct TowerDefenseComponent {
    link: ComponentLink<Self>,
    ecs: ECS,
    // TODO: at some point use images for tiles
    #[allow(unused)]
    assets: Arc<Assets>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum TDMessage {
    ClickedPixel { x: i32, y: i32 },
    Cancel,
    KeyUp(ArrowKey),
    KeyDown(ArrowKey),
    FocusLost,
    // callbacks can't conditionally return things, so we need an "actually nevermind"
    Nothing,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum ArrowKey {
    Down,
    Left,
    Right,
    Up,
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

            self.ecs.with(|world, resources| {
                crate::systems::canvas_render_schedule(&canvas_state, &self.assets).execute(world, resources);
            });
        });
    }
}

#[derive(Properties, Clone)]
pub(crate) struct TDProps {
    pub(crate) ecs: ECS,
    pub(crate) assets: Arc<Assets>,
}

impl Component for TowerDefenseComponent {
    type Message = TDMessage;
    type Properties = TDProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            ecs: props.ecs,
            assets: props.assets,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            TDMessage::Nothing => {}
            TDMessage::ClickedPixel { x, y } => {
                let (left, top) = self.ecs.with(|_, r| {
                    let camera = r.get_or_default::<TdCamera>();
                    (camera.left, camera.top)
                });

                // TODO: probably make this a component and handle it with a system
                if let Some((tile_x, tile_y)) = coords_to_tile_buffered(x + left, y + top, 2) {
                    self.ecs.with(|_, r| {
                        *r.get_mut_or_default::<TdTileSelect>() = TdTileSelect::Selected { x: tile_x, y: tile_y };
                    });
                }
            }
            TDMessage::Cancel => {
                // TODO: probably make this a component and handle it with a system
                self.ecs.with(|_, r| {
                    *r.get_mut_or_default::<TdTileSelect>() = TdTileSelect::None;
                });
            }
            TDMessage::KeyDown(arrow_key) => {
                self.ecs.with(|w, _| match arrow_key {
                    ArrowKey::Down => w.push((UserKeyEvent::KeyDown(UserKey::Down),)),
                    ArrowKey::Left => w.push((UserKeyEvent::KeyDown(UserKey::Left),)),
                    ArrowKey::Right => w.push((UserKeyEvent::KeyDown(UserKey::Right),)),
                    ArrowKey::Up => w.push((UserKeyEvent::KeyDown(UserKey::Up),)),
                });
            }
            TDMessage::KeyUp(arrow_key) => {
                self.ecs.with(|w, _| match arrow_key {
                    ArrowKey::Down => w.push((UserKeyEvent::KeyUp(UserKey::Down),)),
                    ArrowKey::Left => w.push((UserKeyEvent::KeyUp(UserKey::Left),)),
                    ArrowKey::Right => w.push((UserKeyEvent::KeyUp(UserKey::Right),)),
                    ArrowKey::Up => w.push((UserKeyEvent::KeyUp(UserKey::Up),)),
                });
            }
            TDMessage::FocusLost => {
                self.ecs.with(|w, _| {
                    w.push((UserKeyEvent::AllKeysUp,));
                });
            }
        }

        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        // we never "re-draw" like this; we use imperative code to
        // draw stuff on the canvas, instead
        self.draw_canvas();
        false
    }

    fn view(&self) -> Html {
        let hover_cb = self.link.callback(|_: MouseEvent| {
            with_canvas(|cs| {
                cs.canvas.focus().unwrap();
            });

            TDMessage::Nothing
        });

        let click_cb = self.link.callback(|mouse_event: MouseEvent| {
            let (x, y) = with_canvas(|cs| {
                cs.canvas.focus().unwrap();

                let x = mouse_event.client_x() - cs.bounding_rect.left() as i32;
                let y = mouse_event.client_y() - cs.bounding_rect.top() as i32;

                (x, y)
            });

            TDMessage::ClickedPixel { x, y }
        });

        let kd_cb = self.link.callback(|e: KeyboardEvent| match e.code().as_str() {
            "ArrowDown" | "KeyS" => TDMessage::KeyDown(ArrowKey::Down),
            "ArrowUp" | "KeyW" => TDMessage::KeyDown(ArrowKey::Up),
            "ArrowLeft" | "KeyA" => TDMessage::KeyDown(ArrowKey::Left),
            "ArrowRight" | "KeyD" => TDMessage::KeyDown(ArrowKey::Right),
            "Escape" => TDMessage::Cancel,
            _ => TDMessage::Nothing,
        });

        let ku_cb = self.link.callback(|e: KeyboardEvent| match e.code().as_str() {
            "ArrowDown" | "KeyS" => TDMessage::KeyUp(ArrowKey::Down),
            "ArrowUp" | "KeyW" => TDMessage::KeyUp(ArrowKey::Up),
            "ArrowLeft" | "KeyA" => TDMessage::KeyUp(ArrowKey::Left),
            "ArrowRight" | "KeyD" => TDMessage::KeyUp(ArrowKey::Right),
            "Escape" => TDMessage::Cancel,
            _ => TDMessage::Nothing,
        });

        let focus_lost_cb = self.link.callback(|_| TDMessage::FocusLost);

        html! {
            <canvas id="td-canvas" tabIndex=1 onclick=click_cb onmousemove=hover_cb onkeydown=kd_cb onkeyup=ku_cb onblur=focus_lost_cb />
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
