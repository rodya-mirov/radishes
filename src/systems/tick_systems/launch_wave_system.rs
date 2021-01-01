use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::{components::*, resources::*, tile_helpers::tile_to_pixel_coords};

#[system]
#[read_component(TryLaunchWave)]
pub(super) fn process_wave_launch(
    #[resource] next_wave_state: &mut NextWaveState,
    #[resource] map: &Map,
    cmd: &mut CommandBuffer,
    world: &SubWorld,
) {
    let mut query = <(Entity, Read<TryLaunchWave>)>::query();

    for (entity, _try_change) in query.iter(world) {
        if next_wave_state.delay_ticks == 0 {
            let spawns = map.all_spawns();

            let wave_delay = launch_wave(cmd, &spawns);

            next_wave_state.next_wave += 1;
            next_wave_state.delay_ticks = wave_delay;
        }

        cmd.remove(*entity);
    }

    safe_reduce(&mut next_wave_state.delay_ticks);
}

fn safe_reduce(m: &mut usize) {
    if *m > 0 {
        *m -= 1;
    }
}

fn launch_wave(cmd: &mut CommandBuffer, spawns: &[(i32, i32)]) -> usize {
    let mut spawn_idx = 0;
    let mut max_delay = 0;
    for delay in 0..10 {
        let delay_ticks = delay * 20;
        let (tile_x, tile_y) = spawns[spawn_idx];
        let (x, y) = tile_to_pixel_coords(tile_x, tile_y);
        spawn_idx = (spawn_idx + 1) % spawns.len();

        cmd.push((
            Position { x, y },
            TdMob,
            WaveState {
                wave_num: 0,
                wait_state: WaitState::Waiting {
                    ticks_remaining: delay_ticks,
                },
            },
            Renderable::Geometry(RenderGeometry::Circle { radius: 10 }),
            MobHealth {
                current_health: 100,
                max_health: 100,
            },
            Breathes,
            OnDeath {
                events: vec![DeathEvent::GetResources(OwnedResource::Money, 5)],
            },
            Hidden,
        ));

        max_delay = max_delay.max(delay_ticks);
    }

    max_delay + 20
}
