//! Describes how mobs move each tick. Very uniform for now; lots of constants will eventually
//! be moved out to configurable components as we have different kinds of mobs and so on.

use legion::{world::SubWorld, *};

use crate::{
    components::*,
    resources::*,
    tile_helpers::{coords_to_tile, tile_to_pixel_coords},
};

const MOVE_SPEED: i32 = 2;
const DIAG_MOVE_SPEED: i32 = 1;

#[system]
#[write_component(Position)]
#[read_component(TdMob)]
// Note -- map is &mut because we have to verify the dijkstra map is fresh
pub(super) fn move_mobs(#[resource] map: &mut Map, world: &mut SubWorld) {
    let mut query = <(Write<Position>, Read<TdMob>)>::query();

    for (mut pos, _) in query.iter_mut(world) {
        // at the moment, the position is the center of the mob, and is used to compute which tile
        // they're on, for the purpose of pathing. They figure out their goal tile and move toward
        // the center of it.
        if let Some((tile_x, tile_y)) = coords_to_tile(pos.x, pos.y) {
            let (next_x, next_y) = map.move_toward_spawn(tile_x, tile_y);
            let (next_x, next_y) = tile_to_pixel_coords(next_x, next_y);

            let dx = unit_diff(pos.x, next_x);
            let dy = unit_diff(pos.y, next_y);

            let speed = if dx != 0 && dy != 0 { DIAG_MOVE_SPEED } else { MOVE_SPEED };

            pos.x += dx * speed;
            pos.y += dy * speed;
        }
    }
}

fn unit_diff(start: i32, end: i32) -> i32 {
    if start < end {
        1
    } else if start > end {
        -1
    } else {
        0
    }
}
