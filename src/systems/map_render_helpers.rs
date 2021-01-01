use crate::{
    canvas_util::CanvasState,
    resources::TdCamera,
    tile_helpers::{TILE_HEIGHT_PIXELS, TILE_WIDTH_PIXELS},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct MapRenderData {
    // these four fields tell which tiles to render at all
    pub x_min_tile: i32,
    pub y_min_tile: i32,
    pub num_tiles_wide: i32, // x_ind in 0 .. num_tiles_wide+1
    pub num_tiles_tall: i32, // y_ind in 0 .. num_tiles_tall+1
    // used to convert from world coordinates to
    pub x_pixel_offset: i32,
    pub y_pixel_offset: i32,
}

pub fn get_map_render_data(canvas_state: &CanvasState, camera: &TdCamera) -> MapRenderData {
    let canvas_width: i32 = canvas_state.bounding_rect.width() as i32;
    let canvas_height: i32 = canvas_state.bounding_rect.height() as i32;

    // start rendering here for the left column of tiles; this may be negative
    let x_pixel_offset = -(camera.left.rem_euclid(TILE_WIDTH_PIXELS));
    let x_min_pixel = camera.left + x_pixel_offset;
    let x_min_tile = x_min_pixel / TILE_WIDTH_PIXELS;
    let num_tiles_wide = div_round_up(canvas_width - x_pixel_offset, TILE_WIDTH_PIXELS);

    let y_pixel_offset = -(camera.top.rem_euclid(TILE_HEIGHT_PIXELS));
    let y_min_pixel = camera.top + y_pixel_offset;
    let y_min_tile = y_min_pixel / TILE_HEIGHT_PIXELS;
    let num_tiles_tall = div_round_up(canvas_height - y_pixel_offset, TILE_HEIGHT_PIXELS);

    MapRenderData {
        x_min_tile,
        x_pixel_offset,
        y_min_tile,
        y_pixel_offset,
        num_tiles_wide,
        num_tiles_tall,
    }
}

fn div_round_up(amt: i32, div: i32) -> i32 {
    let r = amt.rem_euclid(div);
    let d = amt.div_euclid(div);
    if r > 0 {
        d + 1
    } else {
        d
    }
}
