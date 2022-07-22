use wasm_bindgen::prelude::*;

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
    // Disabled for production
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&JsValue::from_str("Hello"));
    web_sys::console::log_1(&("Hello2".into()));

    Ok(())
}
