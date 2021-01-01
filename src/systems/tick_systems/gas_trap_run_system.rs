//! Every tick, gas traps generate a gas entity at their current position

use legion::{world::SubWorld, *};

use crate::{components::*, resources::*, tile_helpers::coords_to_tile};

#[system]
#[read_component(PoisonGasTrap)]
#[read_component(Position)]
pub(super) fn gas_traps_make_gas(#[resource] map: &mut Map, world: &mut SubWorld) {
    let mut query = <(Read<PoisonGasTrap>, Read<Position>)>::query();

    for (gas_trap, pos) in query.iter_mut(world) {
        let (tile_x, tile_y) = coords_to_tile(pos.x, pos.y);
        let amount: i32 = gas_trap.amount;

        map.add_gas_to_tile(tile_x, tile_y, amount);
    }
}
