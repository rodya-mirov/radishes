use legion::*;

mod change_tile_system;

pub fn make_schedule() -> Schedule {
    Schedule::builder()
        .add_system(change_tile_system::process_tile_changes_system())
        .flush()
        .build()
}
