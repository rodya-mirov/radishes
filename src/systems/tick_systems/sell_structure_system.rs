use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::{components::*, resources::*};

#[system]
#[read_component(TrySellStructure)]
#[read_component(SellValue)]
pub(super) fn sell_structures(#[resource] owned_resources: &mut OwnedResources, cmd: &mut CommandBuffer, world: &SubWorld) {
    let mut query = <(Entity, Read<TrySellStructure>)>::query();

    for (entity, try_sell) in query.iter(world) {
        cmd.remove(*entity);

        let to_sell: Entity = try_sell.to_sell;

        if let Ok(existing) = world.entry_ref(to_sell) {
            if let Ok(sell_value) = existing.get_component::<SellValue>() {
                owned_resources.receive_all(&sell_value.0);
            }

            cmd.remove(to_sell);
        }
    }
}
