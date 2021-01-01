//! Describes how mobs move each tick. Very uniform for now; lots of constants will eventually
//! be moved out to configurable components as we have different kinds of mobs and so on.

use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::components::*;

#[system]
#[read_component(TdMob)]
#[read_component(WaveState)]
#[read_component(MobHealth)]
pub(super) fn mobs_die_at_no_health(cmd: &mut CommandBuffer, world: &mut SubWorld) {
    let mut query = <(Entity, Read<MobHealth>, Read<TdMob>, Read<WaveState>)>::query();

    for (entity, mob_health, _, wave_state) in query.iter_mut(world) {
        if !matches!(wave_state.wait_state, WaitState::Active) {
            continue;
        }

        if mob_health.current_health <= 0 {
            cmd.add_component(*entity, Died);
        }
    }
}
