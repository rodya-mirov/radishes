use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::{components::*, resources::*};

#[system]
#[read_component(UserKeyEvent)]
pub(super) fn process_key_input(#[resource] keys: &mut KeysPressed, cmd: &mut CommandBuffer, world: &SubWorld) {
    let mut query = <(Entity, Read<UserKeyEvent>)>::query();

    for (entity, uke) in query.iter(world) {
        let uke: &UserKeyEvent = uke;
        match *uke {
            UserKeyEvent::KeyDown(key) => *get_flag_mut(key, keys) = true,
            UserKeyEvent::KeyUp(key) => *get_flag_mut(key, keys) = false,
            UserKeyEvent::AllKeysUp => {
                keys.left = false;
                keys.right = false;
                keys.down = false;
                keys.up = false;
            }
        }

        cmd.remove(*entity);
    }
}

fn get_flag_mut(key: UserKey, keys: &mut KeysPressed) -> &mut bool {
    match key {
        UserKey::Up => &mut keys.up,
        UserKey::Down => &mut keys.down,
        UserKey::Left => &mut keys.left,
        UserKey::Right => &mut keys.right,
    }
}
