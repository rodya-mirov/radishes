use legion::*;

use crate::canvas_util::CanvasState;

mod change_tile_system;
mod mob_movement_system;

pub fn make_tick_schedule() -> Schedule {
    Schedule::builder()
        .add_system(change_tile_system::process_tile_changes_system())
        .flush()
        .add_system(mob_movement_system::move_mobs_system())
        .build()
}

mod render_td_mobs_system;

pub fn canvas_render_schedule(canvas_state: CanvasState) -> Schedule {
    Schedule::builder()
        .add_thread_local(render_td_mobs_system::render_mobs_system(canvas_state))
        .build()
}
