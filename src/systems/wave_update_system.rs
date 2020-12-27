//! Describes how mobs move each tick. Very uniform for now; lots of constants will eventually
//! be moved out to configurable components as we have different kinds of mobs and so on.

use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::components::*;

#[system]
#[write_component(WaveState)]
// Note -- map is &mut because we have to verify the dijkstra map is fresh
pub(super) fn update_wave_state(cmd: &mut CommandBuffer, world: &mut SubWorld) {
    let mut query = <(Entity, Write<WaveState>)>::query();

    for (entity, wave_state) in query.iter_mut(world) {
        let (next_state, remove_hidden) = match wave_state.wait_state {
            WaitState::Active => (WaitState::Active, false),
            WaitState::Waiting { ticks_remaining } => {
                if ticks_remaining > 0 {
                    (
                        WaitState::Waiting {
                            ticks_remaining: ticks_remaining - 1,
                        },
                        false,
                    )
                } else {
                    (WaitState::Active, true)
                }
            }
        };

        if remove_hidden {
            cmd.remove_component::<Hidden>(*entity);
        }

        wave_state.wait_state = next_state;
    }
}
