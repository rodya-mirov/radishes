use legion::{systems::Builder, *};

use super::ScheduleBuilderExt;

// user input systems
mod build_structure_system;
mod change_tile_system;
mod keyboard_system;
mod launch_wave_system;
mod sell_structure_system;
mod user_click_system;

// "every tick" systems
mod breathe_gas_system; // breathers should take damage if they're near / on gas
mod camera_move_system; // move the camera in line with the key state
mod death_cleanup; // delete all mobs which have an associated death component
mod death_handler; // process on-death events for all dead things
mod gas_dispersal; // gas should spread out
mod gas_trap_run_system; // gas traps generate poison gas
mod mob_core_system; // if a mob touches the core, deduct player health and destroy (not kill) the mob
mod mob_death_tracker; // if mob health <= 0, give them death component
mod mob_movement_system; // mobs follow their movement AI
mod player_death_system; // if player dies, end the game
mod take_damage_system; // handle "take damage events"
mod wave_update_system; // tick the wave counter and spawn enemies if appropriate

fn add_input_systems(builder: &mut Builder) -> &mut Builder {
    builder
        .add_system_and_flush(user_click_system::process_tile_clicks_system())
        .add_system_and_flush(change_tile_system::process_tile_changes_system())
        .add_system_and_flush(sell_structure_system::sell_structures_system())
        .add_system_and_flush(build_structure_system::build_structures_system())
        .add_system_and_flush(launch_wave_system::process_wave_launch_system())
        .add_system_and_flush(keyboard_system::process_key_input_system())
}

fn add_auto_systems(builder: &mut Builder) -> &mut Builder {
    builder
        .add_system_and_flush(camera_move_system::camera_move_system())
        .add_system_and_flush(wave_update_system::update_wave_state_system())
        .add_system_and_flush(gas_trap_run_system::gas_traps_make_gas_system())
        .add_system_and_flush(gas_dispersal::disperse_gas_system())
        .add_system_and_flush(mob_movement_system::move_mobs_system())
        .add_system_and_flush(breathe_gas_system::breathe_gas_system())
        .add_system_and_flush(mob_core_system::mob_core_hits_system())
        .add_system_and_flush(player_death_system::player_death_system())
        .add_system_and_flush(take_damage_system::take_damage_system())
        .add_system_and_flush(mob_death_tracker::mobs_die_at_no_health_system())
        .add_system_and_flush(death_handler::death_handler_system())
        .add_system_and_flush(death_cleanup::death_cleanup_system())
}

pub fn make_tick_schedule() -> Schedule {
    let mut builder = Schedule::builder();
    add_input_systems(&mut builder);
    add_auto_systems(&mut builder);
    builder.build()
}
