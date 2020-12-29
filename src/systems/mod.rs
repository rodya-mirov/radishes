use legion::{
    systems::{Builder, ParallelRunnable},
    *,
};

use crate::canvas_util::CanvasState;

// user input systems
mod change_tile_system;
mod launch_wave_system;

// "every tick" systems
mod mob_core_system;
mod mob_movement_system;
mod player_death_system;
mod wave_update_system;

pub fn make_tick_schedule() -> Schedule {
    Schedule::builder()
        .flush()
        .add_system_and_flush(change_tile_system::process_tile_changes_system())
        .add_system_and_flush(launch_wave_system::process_wave_launch_system())
        .add_system_and_flush(wave_update_system::update_wave_state_system())
        .add_system_and_flush(mob_movement_system::move_mobs_system())
        .add_system_and_flush(mob_core_system::mob_core_hits_system())
        .add_system_and_flush(player_death_system::player_death_system())
        .build()
}

mod render_td_mobs_system;

pub fn canvas_render_schedule(canvas_state: CanvasState) -> Schedule {
    Schedule::builder()
        .add_thread_local(render_td_mobs_system::render_mobs_system(canvas_state))
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
