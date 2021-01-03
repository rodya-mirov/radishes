use legion::*;

use wasm_bindgen::JsValue;

use crate::{
    canvas_util::CanvasState,
    resources::*,
    tile_helpers::{TILE_HEIGHT_PIXELS, TILE_WIDTH_PIXELS},
};

use super::map_render_helpers::{get_map_render_data, MapRenderData};

#[system]
pub(super) fn draw_map_tiles(
    #[state] canvas_state: &mut CanvasState,
    #[resource] camera: &TdCamera,
    #[resource] map: &Map,
    #[resource] hover_state: &TdTileSelect,
) {
    let MapRenderData {
        x_min_tile,
        y_min_tile,
        num_tiles_wide,
        num_tiles_tall,
        x_pixel_offset,
        y_pixel_offset,
    } = get_map_render_data(canvas_state, camera);

    let black = JsValue::from("#000000");
    let highlighted = JsValue::from("#DCFB3E");

    for x_ind in 0..num_tiles_wide + 1 {
        let tile_x = x_min_tile + x_ind;
        for y_ind in 0..num_tiles_tall + 1 {
            let tile_y = y_min_tile + y_ind;

            let x_left_pixel = x_pixel_offset + (x_ind * TILE_WIDTH_PIXELS);
            let y_top_pixel = y_pixel_offset + (y_ind * TILE_HEIGHT_PIXELS);

            let tile: Tile = map.get_tile(tile_x, tile_y);

            let color = match tile {
                Tile::Open => JsValue::from("#70e0e0"),
                Tile::Wall => JsValue::from("#008050"),
                Tile::Spawn => JsValue::from("#ff1587"),
                Tile::Core => JsValue::from("#1584ff"),
            };

            canvas_state.context.set_fill_style(&color);

            canvas_state.context.fill_rect(
                x_left_pixel as f64,
                y_top_pixel as f64,
                TILE_WIDTH_PIXELS as f64,
                TILE_HEIGHT_PIXELS as f64,
            );
        }
    }

    for x_ind in 0..num_tiles_wide + 1 {
        let tile_x = x_min_tile + x_ind;

        for y_ind in 0..num_tiles_tall + 1 {
            let tile_y = y_min_tile + y_ind;

            let x_left_pixel = x_pixel_offset + (x_ind * TILE_WIDTH_PIXELS);
            let y_top_pixel = y_pixel_offset + (y_ind * TILE_HEIGHT_PIXELS);

            match hover_state {
                TdTileSelect::Selected { x, y, structures: _ } if *x == tile_x && *y == tile_y => {
                    canvas_state.context.set_stroke_style(&highlighted);
                }
                _ => canvas_state.context.set_stroke_style(&black),
            }

            canvas_state.context.stroke_rect(
                x_left_pixel as f64 + 0.5,
                y_top_pixel as f64 + 0.5,
                TILE_WIDTH_PIXELS as f64 - 1.,
                TILE_HEIGHT_PIXELS as f64 - 1.,
            );
        }
    }
}
