use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::{components::*, resources::*};

#[system]
#[read_component(TryChangeTileType)]
pub(super) fn process_tile_changes(
    #[resource] map: &mut Map,
    #[resource] owned_resources: &mut OwnedResources,
    cmd: &mut CommandBuffer,
    world: &mut SubWorld,
) {
    let mut query = <(Entity, Read<TryChangeTileType>)>::query();

    for (entity, try_change) in query.iter_mut(world) {
        let TryChangeTileType {
            x,
            y,
            desired,
            ref costs,
        } = try_change;

        if owned_resources.can_pay(costs) && map.can_set_tile(*x, *y, *desired) {
            owned_resources.pay(costs);
            map.set_tile(*x, *y, *desired);
            cmd.remove(*entity);
        }
    }
}
