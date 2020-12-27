use legion::{world::SubWorld, *};

use wasm_bindgen::JsValue;

use crate::{canvas_util::CanvasState, components::*, resources::*};

#[system]
#[read_component(Position)]
#[read_component(Renderable)]
pub(super) fn render_mobs(#[state] canvas_state: &mut CanvasState, #[resource] camera: &TdCamera, world: &SubWorld) {
    let ctx = &canvas_state.context;
    let mut query = <(Read<Position>, Read<Renderable>)>::query();

    ctx.set_stroke_style(&JsValue::from("goldenrod"));

    for (pos, rend) in query.iter(world) {
        match rend {
            &Renderable::Circle { radius } => {
                let xmin = pos.x - radius;
                let xmax = pos.x + radius;
                let ymin = pos.y - radius;
                let ymax = pos.y + radius;

                if xmin > camera.left + (canvas_state.bounding_rect.width() as i32) {
                    continue;
                }
                if xmax < camera.left {
                    continue;
                }
                if ymin > camera.top + (canvas_state.bounding_rect.height() as i32) {
                    continue;
                }
                if ymax < camera.top {
                    continue;
                }

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
            }
        }
    }
}
