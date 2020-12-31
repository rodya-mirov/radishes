//! General module for executing and viewing the "tower defense" component (which is basically
//! an entire game, but the "view" is just a canvas, so that works well)

use std::sync::Arc;

use yew::prelude::*;

use wasm_bindgen::JsValue;
use web_sys::{KeyboardEvent, MouseEvent};

use crate::{
    assets::Assets,
    canvas_util::with_canvas,
    resources::*,
    tile_helpers::{coords_to_tile_buffered, TILE_HEIGHT_PIXELS, TILE_WIDTH_PIXELS},
    ECS,
};

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
    ArrowKeyDown(ArrowKey),
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

fn div_round_up(amt: i32, div: i32) -> i32 {
    let r = amt.rem_euclid(div);
    let d = amt.div_euclid(div);
    if r > 0 {
        d + 1
    } else {
        d
    }
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

            let canvas_width: i32 = canvas_state.bounding_rect.width() as i32;
            let canvas_height: i32 = canvas_state.bounding_rect.height() as i32;

            self.ecs.with(|_, resources| {
                let hover_state: TdTileSelect = *(resources.get_or_default());
                let map = resources.get::<Map>().unwrap();
                let camera = resources.get::<TdCamera>().unwrap();

                // start rendering here for the left column of tiles; this may be negative
                let x_pixel_offset = -(camera.left.rem_euclid(TILE_WIDTH_PIXELS));
                let x_min_pixel = camera.left + x_pixel_offset;
                let x_min_tile = x_min_pixel / TILE_WIDTH_PIXELS;
                let num_tiles_wide = div_round_up(canvas_width - x_pixel_offset, TILE_WIDTH_PIXELS);

                let y_pixel_offset = -(camera.top.rem_euclid(TILE_HEIGHT_PIXELS));
                let y_min_pixel = camera.top + y_pixel_offset;
                let y_min_tile = y_min_pixel / TILE_HEIGHT_PIXELS;
                let num_tiles_tall = div_round_up(canvas_height - y_pixel_offset, TILE_HEIGHT_PIXELS);

                let black = JsValue::from("#000000");
                let highlighted = JsValue::from("#DCFB3E");

                for x_ind in 0..num_tiles_wide + 1 {
                    let tile_x = x_min_tile + x_ind;
                    for y_ind in 0..num_tiles_tall + 1 {
                        let tile_y = y_min_tile + y_ind;

                        let x_left_pixel = x_pixel_offset + (x_ind * TILE_WIDTH_PIXELS);
                        let y_top_pixel = y_pixel_offset + (y_ind * TILE_HEIGHT_PIXELS);

                        let tile: Tile = map.get_tile(tile_x, tile_y);

                        let color = match tile {
                            Tile::Open => JsValue::from("#70e0e0"),
                            Tile::Wall => JsValue::from("#008050"),
                            Tile::Spawn => JsValue::from("#ff1587"),
                            Tile::Core => JsValue::from("#1584ff"),
                        };

                        canvas_state.context.set_fill_style(&color);

                        canvas_state.context.fill_rect(
                            x_left_pixel as f64,
                            y_top_pixel as f64,
                            TILE_WIDTH_PIXELS as f64,
                            TILE_HEIGHT_PIXELS as f64,
                        );
                    }
                }

                for x_ind in 0..num_tiles_wide + 1 {
                    let tile_x = x_min_tile + x_ind;

                    for y_ind in 0..num_tiles_tall + 1 {
                        let tile_y = y_min_tile + y_ind;

                        let x_left_pixel = x_pixel_offset + (x_ind * TILE_WIDTH_PIXELS);
                        let y_top_pixel = y_pixel_offset + (y_ind * TILE_HEIGHT_PIXELS);

                        if hover_state == (TdTileSelect::Selected { x: tile_x, y: tile_y }) {
                            canvas_state.context.set_stroke_style(&highlighted);
                        } else {
                            canvas_state.context.set_stroke_style(&black);
                        }

                        canvas_state.context.stroke_rect(
                            x_left_pixel as f64 + 0.5,
                            y_top_pixel as f64 + 0.5,
                            TILE_WIDTH_PIXELS as f64 - 1.,
                            TILE_HEIGHT_PIXELS as f64 - 1.,
                        );
                    }
                }
            });

            self.ecs.with(|world, resources| {
                crate::systems::canvas_render_schedule(canvas_state.clone()).execute(world, resources);
            });
        });
    }
}

#[derive(Properties, Clone)]
pub(crate) struct TDProps {
    pub(crate) ecs: ECS,
    pub(crate) assets: Arc<Assets>,
}

// TODO: probably make this configurable?
const KEYBOARD_MOVE_SPEED: i32 = 10;

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

                if let Some((tile_x, tile_y)) = coords_to_tile_buffered(x + left, y + top, 2) {
                    self.ecs.with(|_, r| {
                        *r.get_mut_or_default::<TdTileSelect>() = TdTileSelect::Selected { x: tile_x, y: tile_y };
                    });
                }
            }
            TDMessage::Cancel => {
                self.ecs.with(|_, r| {
                    *r.get_mut_or_default::<TdTileSelect>() = TdTileSelect::None;
                });
            }
            TDMessage::ArrowKeyDown(arrow_key) => {
                let (dx, dy) = match arrow_key {
                    ArrowKey::Down => (0, 1),
                    ArrowKey::Up => (0, -1),
                    ArrowKey::Left => (-1, 0),
                    ArrowKey::Right => (1, 0),
                };

                self.ecs.with(|_, r| {
                    let mut camera = r.get_mut::<TdCamera>().unwrap();
                    camera.top += dy * KEYBOARD_MOVE_SPEED;
                    camera.left += dx * KEYBOARD_MOVE_SPEED;
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
            "ArrowDown" | "KeyS" => TDMessage::ArrowKeyDown(ArrowKey::Down),
            "ArrowUp" | "KeyW" => TDMessage::ArrowKeyDown(ArrowKey::Up),
            "ArrowLeft" | "KeyA" => TDMessage::ArrowKeyDown(ArrowKey::Left),
            "ArrowRight" | "KeyD" => TDMessage::ArrowKeyDown(ArrowKey::Right),
            "Escape" => TDMessage::Cancel,
            _ => TDMessage::Nothing,
        });

        html! {
            <canvas id="td-canvas" tabIndex=1 onclick=click_cb onmousemove=hover_cb onkeydown=kd_cb />
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
