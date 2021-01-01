//! Entities that breathe and share a tile with gas should take appropriate damage

use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::{components::*, resources::*, tile_helpers::coords_to_tile};

#[system]
#[read_component(Position)]
#[read_component(Breathes)]
pub(super) fn breathe_gas(#[resource] map: &Map, cmd: &mut CommandBuffer, world: &SubWorld) {
    let mut query = <(Entity, Read<Position>, Read<Breathes>)>::query();

    for (entity, pos, _) in query.iter(world) {
        let (tile_x, tile_y) = coords_to_tile(pos.x, pos.y);
        let gas_amount = map.get_gas_amount(tile_x, tile_y);

        if gas_amount > 0 {
            cmd.push((TakeDamage {
                target: *entity,
                // since we can't communicate how much gas there is to the player,
                // enemies just take a flat amount of damage from "any gas", which is a perfectly
                // fine gameplay mechanic and more transparent for the user
                amount: 5,
            },));
        }
    }
}
