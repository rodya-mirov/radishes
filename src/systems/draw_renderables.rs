use std::sync::Arc;

use legion::{world::SubWorld, *};

use wasm_bindgen::JsValue;

use crate::{
    assets::{Assets, ImageBitmapExt},
    canvas_util::CanvasState,
    components::*,
    resources::*,
    tile_helpers::{TILE_HEIGHT_PIXELS, TILE_WIDTH_PIXELS},
};

#[system]
#[read_component(Position)]
#[read_component(Renderable)]
#[read_component(Hidden)]
#[read_component(MobHealth)]
pub(super) fn draw_renderables(
    #[state] canvas_state: &mut CanvasState,
    #[state] assets: &mut Arc<Assets>,
    #[resource] camera: &TdCamera,
    world: &SubWorld,
) {
    let ctx = &canvas_state.context;
    let mut query = <(Read<Position>, Read<Renderable>, TryRead<Hidden>, TryRead<MobHealth>)>::query();

    let camera_bounds = BoundingBox {
        xmin: camera.left,
        xmax: camera.left + canvas_state.bounding_rect.width() as i32,
        ymin: camera.top,
        ymax: camera.top + canvas_state.bounding_rect.height() as i32,
    };

    for (pos, rend, hidden, maybe_health) in query.iter(world) {
        if hidden.is_some() {
            continue;
        }

        let render_bounds = get_render_bounds(*pos, *rend);

        if !intersects(render_bounds, camera_bounds) {
            continue;
        }

        match *rend {
            Renderable::Geometry(geom) => match geom {
                RenderGeometry::Circle { radius } => {
                    ctx.set_stroke_style(&JsValue::from("goldenrod"));

                    ctx.begin_path();
                    ctx.arc(
                        (pos.x - camera.left).into(),
                        (pos.y - camera.top).into(),
                        radius.into(),
                        0.,
                        std::f64::consts::TAU,
                    )
                    .expect("Arc should be drawable");
                    ctx.stroke();

                    if let Some(health) = maybe_health.copied() {
                        draw_health_bar(canvas_state, health, pos.x - camera.left - radius, pos.y - camera.top - radius);
                    }
                }
            },
            Renderable::Bitmap { dx, dy, bitmap } => {
                let (bitmap, sx, sy, sw, sh) = match bitmap {
                    // TODO: these constants should be in the Assets struct itself somehow
                    RenderBitmap::GasImage => (&assets.gas_image, 0, 0, TILE_WIDTH_PIXELS, TILE_HEIGHT_PIXELS),
                };

                bitmap
                    .render_to_canvas(
                        &canvas_state.context,
                        sx,
                        sy,
                        dx + pos.x - camera.left,
                        dy + pos.y - camera.top,
                        sw,
                        sh,
                    )
                    .expect("Image should render");

                if let Some(health) = maybe_health.copied() {
                    draw_health_bar(canvas_state, health, dx + pos.x - camera.left, dy + pos.y - camera.top);
                }
            }
        }
    }
}

const HEALTH_BAR_WIDTH: i32 = 5;
const HEALTH_BAR_HEIGHT: i32 = 16;

fn draw_health_bar(canvas_state: &CanvasState, health: MobHealth, left: i32, top: i32) {
    let ctx = &canvas_state.context;

    // draw a rectangle
    ctx.set_stroke_style(&JsValue::from_str("goldenrod"));
    ctx.stroke_rect(
        left as f64 + 0.5,
        top as f64 + 0.5,
        HEALTH_BAR_WIDTH as f64,
        HEALTH_BAR_HEIGHT as f64,
    );

    // partially fill it up
    let fill_height = health.current_health * (HEALTH_BAR_HEIGHT - 2) / health.max_health;
    ctx.set_stroke_style(&JsValue::from_str("red"));
    ctx.fill_rect(
        left as f64 + 1.,
        (top + (HEALTH_BAR_HEIGHT - 2) - fill_height + 1) as f64,
        HEALTH_BAR_WIDTH as f64 - 1.,
        fill_height as f64 + 1.,
    );
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct BoundingBox {
    xmin: i32, // incl
    xmax: i32, // excl
    ymin: i32, // incl
    ymax: i32, // excl
}

#[inline]
fn intersects_1d(amin: i32, amax: i32, bmin: i32, bmax: i32) -> bool {
    amin < bmax && bmin < amax
}

#[inline]
fn intersects(a: BoundingBox, b: BoundingBox) -> bool {
    intersects_1d(a.xmin, a.xmax, b.xmin, b.xmax) && intersects_1d(a.ymin, a.ymax, b.ymin, b.ymax)
}

fn get_render_bounds(pos: Position, rend: Renderable) -> BoundingBox {
    match rend {
        Renderable::Bitmap { dx, dy, bitmap } => match bitmap {
            RenderBitmap::GasImage => BoundingBox {
                xmin: pos.x + dx,
                ymin: pos.y + dy,
                xmax: pos.x + dx + TILE_WIDTH_PIXELS,
                ymax: pos.y + dy + TILE_HEIGHT_PIXELS,
            },
        },
        Renderable::Geometry(geometry) => match geometry {
            RenderGeometry::Circle { radius } => BoundingBox {
                xmin: pos.x - radius,
                ymin: pos.y - radius,
                xmax: pos.x + radius + 1,
                ymax: pos.y + radius + 1,
            },
        },
    }
}
