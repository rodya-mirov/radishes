pub const TILE_WIDTH_PIXELS: i32 = 32;
pub const TILE_HEIGHT_PIXELS: i32 = 32;

/// Given pixel coordinates x and y (already transformed to "world" pixels), transform them
/// into a tile coordinate. Note that if it's too close to the border, it will return None, to avoid
/// madness.
///
/// :param border_offset -- Used to say when the next tile starts. e.g. if given 1,
///     and the tile width is 30, this means that a tile "starts" on 1, 31, etc.
pub fn coords_to_tile_buffered(x: i32, y: i32, border_offset: i32) -> Option<(i32, i32)> {
    let tile_x = safe_div(x - border_offset, TILE_WIDTH_PIXELS, 2)?;

    let tile_y = safe_div(y - border_offset, TILE_HEIGHT_PIXELS, 2)?;

    Some((tile_x, tile_y))
}

pub fn coords_to_tile(x: i32, y: i32) -> Option<(i32, i32)> {
    let tile_x = safe_div(x, TILE_WIDTH_PIXELS, 0)?;

    let tile_y = safe_div(y, TILE_HEIGHT_PIXELS, 0)?;

    Some((tile_x, tile_y))
}

/// Transforms tile coordinates to pixel coordinates which are in the center of the tile
pub fn tile_to_pixel_coords(tile_x: i32, tile_y: i32) -> (i32, i32) {
    (
        tile_x * TILE_WIDTH_PIXELS + (TILE_WIDTH_PIXELS / 2),
        tile_y * TILE_HEIGHT_PIXELS + (TILE_HEIGHT_PIXELS / 2),
    )
}

// TODO: doc
fn safe_div(amt: i32, div: i32, tol: i32) -> Option<i32> {
    if tol < 0 {
        return safe_div(amt, div, -tol);
    }

    if div <= 0 {
        // TODO: logging about bad inputs
        return None;
    }

    let m = amt.rem_euclid(div);
    if m < tol || div - m - 1 < tol {
        return None;
    }

    let out = amt.div_euclid(div);
    Some(out)
}
