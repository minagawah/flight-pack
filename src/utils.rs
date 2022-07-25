use futures::FutureExt;
use futures::future::LocalBoxFuture;
use num::{Float, NumCast};
use serde::Deserialize;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    CanvasRenderingContext2d,
    HtmlCanvasElement,
    Window,
};

pub fn exit(message: &str) {
    let v = JsValue::from_str(message);
    web_sys::console::log_1(&("panic".into()));
    web_sys::console::exception_1(&v);
    std::process::abort();
}

pub async fn timer(msec: i32) -> Result<(), JsValue> {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        get_window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve, msec
            )
            .unwrap();
    });
    let future = wasm_bindgen_futures::JsFuture::from(promise);
    future.await?;
    Ok(())
}

pub fn get_json<'a, T: Deserialize<'a>>(json: &'a str) -> T {
    match serde_json::from_str(json) {
        Ok(json) => json,
        Err(err) => panic!("Error: {}", err),
    }
}

pub fn get_window() -> Result<Window, String> {
    web_sys::window().ok_or_else(|| "No window".into())
}

pub fn device_pixel_ratio() -> f64 {
    get_window().map_or(1_f64, |w| w.device_pixel_ratio())
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    get_window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Failed to start request_animation_frame");
}

pub fn request_animation_frame_future() -> LocalBoxFuture<'static, ()> {
    let f = callback_future::CallbackFuture::new(|complete| {
        get_window()
            .expect("Should have window")
            .request_animation_frame(
                Closure::once_into_js(move || { complete(()) })
                    .as_ref()
                    .unchecked_ref()
            )
            .expect("should register `requestAnimationFrame` OK");
    });
    f.boxed_local()
}

pub fn get_ctx(canvas: &HtmlCanvasElement) ->
    Result<CanvasRenderingContext2d, String>
{
    // let c_0 = &*canvas;
    // let ctx = c_0
    let ctx = canvas
        .get_context("2d")
        .map_err(|_| "Failed get 2D Context".to_string())?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    Ok(ctx)
}

pub fn rad_to_deg(rad: f64) -> f64 { rad * (180.0 / PI) }
pub fn deg_to_rad(deg: f64) -> f64 { deg * (PI / 180.0) }

pub fn fixed_decimals<F: Float>(value: F, digits: F) -> F {
    (value * digits).round() / digits
}

pub fn lazy_round<F: Float>(value: F) -> F {
    fixed_decimals(value, NumCast::from(100.00).unwrap())
}

/// Get the norm for `val` between `min` and `max`.
/// Ex. norm(75, 0, 100) ---> 0.75
pub fn norm(val: f64, min: f64, max: f64) -> f64 {
    (val - min) / (max - min)
}

/// Apply `norm` (the linear interpolate value) to the range
/// between `min` and `max` (usually between `0` and `1`).
/// Ex. lerp(0.5, 0, 100) ---> 50
pub fn lerp(norm: f64, min: f64, max: f64) -> f64 {
    min + (max - min) * norm
}

pub fn f64_from_js(js: JsValue) -> f64 {
    js.as_f64().unwrap_or_default()
}

pub fn f64_cmp(a: &f64, b: &f64) -> std::cmp::Ordering {
    a.partial_cmp(b).unwrap()
}
