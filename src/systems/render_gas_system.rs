use std::sync::Arc;

use legion::*;

use web_sys::ImageBitmap;

use crate::{
    assets::{Assets, ImageBitmapExt},
    canvas_util::CanvasState,
    resources::*,
    tile_helpers::{TILE_HEIGHT_PIXELS, TILE_WIDTH_PIXELS},
};

use super::map_render_helpers::{get_map_render_data, MapRenderData};

const MAX_GAS_ALPHA: f64 = 0.6;

#[system]
pub(super) fn draw_gas(
    #[state] canvas_state: &mut CanvasState,
    #[state] assets: &mut Arc<Assets>,
    #[resource] camera: &TdCamera,
    #[resource] map: &Map,
) {
    let MapRenderData {
        x_min_tile,
        y_min_tile,
        num_tiles_wide,
        num_tiles_tall,
        x_pixel_offset,
        y_pixel_offset,
    } = get_map_render_data(canvas_state, camera);

    let bitmap: &ImageBitmap = &assets.gas_image;

    let old_alpha: f64 = canvas_state.context.global_alpha();

    for x_ind in 0..num_tiles_wide + 1 {
        let tile_x = x_min_tile + x_ind;
        for y_ind in 0..num_tiles_tall + 1 {
            let tile_y = y_min_tile + y_ind;

            let x_left_pixel = x_pixel_offset + (x_ind * TILE_WIDTH_PIXELS);
            let y_top_pixel = y_pixel_offset + (y_ind * TILE_HEIGHT_PIXELS);

            let gas_amount = map.get_gas_amount(tile_x, tile_y);
            if gas_amount > 0 {
                let opacity = gas_opacity(gas_amount);
                canvas_state.context.set_global_alpha(opacity);
                bitmap
                    .render_to_canvas_tile(&canvas_state.context, 0, 0, x_left_pixel, y_top_pixel)
                    .unwrap();
            }
        }
    }

    canvas_state.context.set_global_alpha(old_alpha);
}

fn gas_opacity(amount: i32) -> f64 {
    // basically 0 -> 0+ep, 100 -> 1-ep, everything else sort of smooth
    if amount <= 0 {
        return 0.;
    }

    MAX_GAS_ALPHA / (1. + f64::exp(amount as f64 / -4.))
}
