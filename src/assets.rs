use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use web_sys::{Blob, CanvasRenderingContext2d, ImageBitmap, Request, RequestInit, RequestMode, Response, Window};

pub struct Assets {
    pub gas_trap: ImageBitmap,
    pub gas_image: ImageBitmap,
}

pub trait ImageBitmapExt {
    fn render_to_canvas(&self, ctx: &CanvasRenderingContext2d, sx: i32, sy: i32, dx: i32, dy: i32, sw: i32, sh: i32)
        -> Result<(), JsValue>;

    fn render_to_canvas_tile(&self, ctx: &CanvasRenderingContext2d, sx: i32, sy: i32, dx: i32, dy: i32) -> Result<(), JsValue> {
        self.render_to_canvas(
            ctx,
            sx,
            sy,
            dx,
            dy,
            crate::tile_helpers::TILE_WIDTH_PIXELS,
            crate::tile_helpers::TILE_HEIGHT_PIXELS,
        )
    }
}

impl ImageBitmapExt for ImageBitmap {
    fn render_to_canvas(
        &self,
        ctx: &CanvasRenderingContext2d,
        sx: i32,
        sy: i32,
        dx: i32,
        dy: i32,
        sw: i32,
        sh: i32,
    ) -> Result<(), JsValue> {
        ctx.draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &self, sx as f64, sy as f64, sw as f64, sh as f64, dx as f64, dy as f64, sw as f64, sh as f64,
        )
    }
}

pub async fn load_assets() -> Result<Assets, JsValue> {
    let window = web_sys::window().expect("Should have a window");

    let assets = Assets {
        gas_image: load_image(&window, "/assets/images/gas-frame.png").await?,
        gas_trap: load_image(&window, "/assets/images/gas-trap.png").await?,
    };

    Ok(assets)
}

async fn load_image(window: &Window, url: &str) -> Result<ImageBitmap, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let text: JsValue = JsFuture::from(resp.blob()?).await?;

    assert!(text.is_instance_of::<Blob>());
    let blob: Blob = text.dyn_into().unwrap();

    let image = JsFuture::from(window.create_image_bitmap_with_blob(&blob)?).await?;

    assert!(image.is_instance_of::<ImageBitmap>());
    let image: ImageBitmap = image.dyn_into().unwrap();

    Ok(image)
}
