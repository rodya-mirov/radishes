use std::sync::Arc;

use legion::{
    systems::{Builder, ParallelRunnable},
    *,
};

use crate::{assets::Assets, canvas_util::CanvasState};

// TODO: split out UI systems from "natural" systems

mod tick_systems;

pub use tick_systems::make_tick_schedule;

mod map_render_helpers;

// TODO: make a render systems folder

mod draw_renderables;
mod render_gas_system;
mod render_map_system;

// TODO: don't recreate this every time, somehow
pub fn canvas_render_schedule(canvas_state: &CanvasState, assets: &Arc<Assets>) -> Schedule {
    Schedule::builder()
        .add_thread_local(render_map_system::draw_map_tiles_system(canvas_state.clone()))
        .add_thread_local(draw_renderables::draw_renderables_system(canvas_state.clone(), assets.clone()))
        .add_thread_local(render_gas_system::draw_gas_system(canvas_state.clone(), assets.clone()))
        .build()
}

trait ScheduleBuilderExt {
    fn add_system_and_flush<T: ParallelRunnable + 'static>(&mut self, system: T) -> &mut Self;
}

impl ScheduleBuilderExt for Builder {
    fn add_system_and_flush<T: ParallelRunnable + 'static>(&mut self, system: T) -> &mut Self {
        self.add_system(system).flush()
    }
}
