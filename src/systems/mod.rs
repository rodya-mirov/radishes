use legion::{
    systems::{Builder, ParallelRunnable},
    *,
};

use crate::canvas_util::CanvasState;

// user input systems
mod change_tile_system;
mod launch_wave_system;

// "every tick" systems
mod death_cleanup; // delete all mobs which have an associated death component
mod death_handler;
mod mob_core_system; // if a mob touches the core, deduct player health and destroy (not kill) the mob
mod mob_death_tracker; // if mob health <= 0, give them death component
mod mob_health_reducer_system; // every tick, mob loses 1 health TODO: delete this when we have traps
mod mob_movement_system; // mobs follow their movement AI
mod player_death_system; // if player dies, end the game
mod wave_update_system; // tick the wave counter and spawn enemies if appropriate // process on-death events for all dead things

pub fn make_tick_schedule() -> Schedule {
    Schedule::builder()
        .add_system_and_flush(change_tile_system::process_tile_changes_system())
        .add_system_and_flush(launch_wave_system::process_wave_launch_system())
        .add_system_and_flush(wave_update_system::update_wave_state_system())
        .add_system_and_flush(mob_movement_system::move_mobs_system())
        .add_system_and_flush(mob_core_system::mob_core_hits_system())
        .add_system_and_flush(player_death_system::player_death_system())
        .add_system_and_flush(mob_health_reducer_system::mobs_hurt_on_tick_system())
        .add_system_and_flush(mob_death_tracker::mobs_die_at_no_health_system())
        .add_system_and_flush(death_handler::death_handler_system())
        .add_system_and_flush(death_cleanup::death_cleanup_system())
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
