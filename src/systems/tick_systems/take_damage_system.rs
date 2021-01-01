//! General-purpose handler for any "take damage" events

use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::components::*;
use wasm_bindgen::__rt::std::collections::HashMap;

#[system]
#[write_component(MobHealth)]
#[read_component(TakeDamage)]
pub(super) fn take_damage(cmd: &mut CommandBuffer, world: &mut SubWorld) {
    let mut query_get_takes = <(Entity, Read<TakeDamage>)>::query();
    let mut damages = HashMap::new();

    for (damage_event_entity, take_damage) in query_get_takes.iter(world) {
        cmd.remove(*damage_event_entity);
        damages
            .entry(take_damage.target)
            .or_insert_with(|| Vec::with_capacity(1))
            .push(*take_damage);
    }

    for (damaged_entity, events) in damages {
        // err in this part probably means the mob is already dead, which is fine
        if let Ok(mut entity_mut) = world.entry_mut(damaged_entity) {
            let mob_health = entity_mut
                .get_component_mut::<MobHealth>()
                .expect("System should ensure targeted mobs have health");
            for event in events {
                mob_health.current_health -= event.amount;
            }
        }
    }
}
