//! Describes how mobs move each tick. Very uniform for now; lots of constants will eventually
//! be moved out to configurable components as we have different kinds of mobs and so on.

use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::components::*;

#[system]
#[read_component(Died)]
pub(super) fn death_cleanup(cmd: &mut CommandBuffer, world: &mut SubWorld) {
    let mut query = <(Entity, Read<Died>)>::query();

    for (entity, _) in query.iter_mut(world) {
        cmd.remove(*entity);
    }
}
