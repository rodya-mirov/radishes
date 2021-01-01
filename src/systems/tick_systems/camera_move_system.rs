use legion::*;

use crate::resources::*;

// NB: tiles are 32 px wide and we run the sim at 10 fps
const KEYBOARD_MOVE_SPEED: i32 = 8;

#[system]
pub(super) fn camera_move(#[resource] keys: &KeysPressed, #[resource] camera: &mut TdCamera) {
    if keys.right {
        camera.left += KEYBOARD_MOVE_SPEED;
    }
    if keys.left {
        camera.left -= KEYBOARD_MOVE_SPEED;
    }
    if keys.up {
        camera.top -= KEYBOARD_MOVE_SPEED;
    }
    if keys.down {
        camera.top += KEYBOARD_MOVE_SPEED;
    }
}
