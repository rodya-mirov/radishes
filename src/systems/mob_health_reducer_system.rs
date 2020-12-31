//! Describes how mobs move each tick. Very uniform for now; lots of constants will eventually
//! be moved out to configurable components as we have different kinds of mobs and so on.

use legion::{world::SubWorld, *};

use crate::components::*;

#[system]
#[read_component(TdMob)]
#[read_component(WaveState)]
#[write_component(MobHealth)]
// Note -- map is &mut because we have to verify the dijkstra map is fresh
pub(super) fn mobs_hurt_on_tick(world: &mut SubWorld) {
    let mut query = <(Write<MobHealth>, Read<TdMob>, Read<WaveState>)>::query();

    for (mut mob_health, _, wave_state) in query.iter_mut(world) {
        if !matches!(wave_state.wait_state, WaitState::Active) {
            continue;
        }

        mob_health.0 -= 1;
    }
}
