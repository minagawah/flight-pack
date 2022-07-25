#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate load_dotenv;

pub mod app;
pub mod aviation;
pub mod constants;
pub mod dimension;
pub mod manager;
pub mod proxy;
pub mod request;
pub mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use js_sys::Array;

use crate::dimension::geo::{
    GeoCoord,
    get_center_from_coords,
};

// #[wasm_bindgen(module = "/sleep.js")]
// extern "C" {
//     fn sleep(ms: i32) -> Promise;
// }

// pub async fn timer(ms: i32) -> Result<(), JsValue> {
//     let promise = sleep(ms);
//     let js_fut = JsFuture::from(promise);
//     js_fut.await?;
//     Ok(())
// }

// `wee_alloc` is a tiny allocator designed for WebAssembly
// that has a (pre-compression) code-size footprint of only
// a single kilobyte. When the `wee_alloc` feature is enabled,
// this uses `wee_alloc` as the global allocator. If you don't
// want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    #[cfg(debug_assertions)]
    web_sys::console::log_1(&JsValue::from_str("debug"));

    #[cfg(not(debug_assertions))]
    web_sys::console::log_1(&JsValue::from_str("release"));

    Ok(())
}

#[wasm_bindgen]
pub fn find_geo_center(coords: &JsValue) -> Array {
    let coords: Result<Vec<GeoCoord>, serde_json::Error> =
        coords.into_serde();

    match coords {
        Ok(coords) => {
            let center: GeoCoord =
                get_center_from_coords(coords);

            [center.lat(), center.lng()]
                .iter()
                .map(|v| JsValue::from_f64(*v))
                .collect::<Array>()
        },
        Err(_) => Array::new(),
    }
}
