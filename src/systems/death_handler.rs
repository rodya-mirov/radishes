//! Describes how mobs move each tick. Very uniform for now; lots of constants will eventually
//! be moved out to configurable components as we have different kinds of mobs and so on.

use legion::{world::SubWorld, *};

use crate::{components::*, resources::*};

#[system]
#[read_component(OnDeath)]
#[read_component(Died)]
// Note -- map is &mut because we have to verify the dijkstra map is fresh
pub(super) fn death_handler(#[resource] owned: &mut OwnedResources, world: &mut SubWorld) {
    let mut query = <(Read<Died>, TryRead<OnDeath>)>::query();

    for (_, on_death) in query.iter_mut(world) {
        if let Some(on_death_ref) = on_death.as_ref() {
            // Type ascription because IJ is just lost
            let odr: &OnDeath = on_death_ref;
            for death_event in odr.events.iter() {
                match *death_event {
                    DeathEvent::GetResources(kind, amount) => {
                        owned.receive(kind, amount);
                    }
                }
            }
        }
    }
}
