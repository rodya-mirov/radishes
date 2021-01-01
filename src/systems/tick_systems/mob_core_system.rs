//! Any TdMob that touched the core is deleted, wooo

use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::{components::*, resources::*};

#[system]
#[read_component(TdMob)]
#[read_component(TouchedCore)]
pub(super) fn mob_core_hits(#[resource] player_health: &mut PlayerHealth, cmd: &mut CommandBuffer, world: &mut SubWorld) {
    let mut query = <(Entity, Read<TdMob>, Read<TouchedCore>)>::query();

    for (entity, _, _) in query.iter(world) {
        cmd.remove(*entity);
        player_health.health -= 1;
    }
}
