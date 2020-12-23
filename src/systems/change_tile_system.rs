use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::{components::*, resources::*};

#[system]
#[read_component(TryChangeTileType)]
pub(super) fn process_tile_changes(
    #[resource] map: &mut Map,
    cmd: &mut CommandBuffer,
    world: &mut SubWorld,
) {
    let mut query = <(Entity, Read<TryChangeTileType>)>::query();

    for (entity, try_change) in query.iter_mut(world) {
        let TryChangeTileType { x, y, desired } = *try_change;
        map.set_tile(x, y, desired);
        cmd.remove(*entity);
    }
}
