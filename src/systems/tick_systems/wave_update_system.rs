//! Describes how mobs move each tick. Very uniform for now; lots of constants will eventually
//! be moved out to configurable components as we have different kinds of mobs and so on.

use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::{components::*, resources::*};

#[system]
#[write_component(WaveState)]
pub(super) fn update_wave_state(#[resource] next_wave_state: &mut NextWaveState, cmd: &mut CommandBuffer, world: &mut SubWorld) {
    if next_wave_state.delay_ticks == 0 && next_wave_state.auto_launch {
        cmd.push((TryLaunchWave,));
    }

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
