/// `App` does not do much. Instead, `Proxy` does
/// all the jobs for `App`. When we want to call
/// asynchronous functions (in WASM apps using
/// 'wasm-bindgen'), we spawn a thread. However,
/// say, you want move `self` of `App` into
/// the thread. Unfortunately, Rust does not
/// allow that... To move it into the thread,
/// you need to clone `self`. Yet, again,
/// that will cost too much... That's why
/// we have `proxy: Arc<Proxy>` which is
/// the only variable defined in `App`.
/// Cloning `Arc` costs you nothing because
/// it means to just prepare another reference
/// to the original. Notice `Proxy` exposes
/// static functions only. Whenever you want
/// some jobs done using `Proxy`, yfu simply
/// clone `proxy` instance (which will be
/// just a reference) in `App`, and you
/// pass it to these static functions
/// (we refer to it as `this` because it sounds
/// perfect for the name). When receiving `this`,
/// the static functions will use it for their
/// own contexts, looking up their own resources.
/// In another word, static functions begin
/// to behave just like any other member functions.
/// It is just that `App` holding onto the context.

use std::sync::Arc;
use js_sys::Promise;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, future_to_promise};
use web_sys::HtmlCanvasElement;

use crate::proxy::Proxy;

#[wasm_bindgen]
pub struct App {
    // Note: We could have `Arc<Mutex<Proxy>>`
    // to have a direct mutation on `proxy`,
    // but since we don't have anything
    // to mutate in `Proxy`, we simply use
    // `Arc<Proxy>`. Mutations occur not in `App`,
    // neither in `Proxy`, but only in
    // instantiated members of `Proxy`.
    // (e.g. `Proxy::canvas`)
    proxy: Arc<Proxy>,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(element: HtmlCanvasElement) -> Result<App, JsValue> {
        Ok(App {
            proxy: Arc::new(
                Proxy::new(element)
            ),
        })
    }

    /// Once `App` instance is created on JS-side,
    /// JS will run `App::prepare` which is fed with
    /// `airports`. For `airports`, we want to fetch
    /// arrivals/departures from FlightAware API.
    ///
    /// In order to return `Promise` back to JS, we use
    /// `wasm_bindgen_futures::future_to_promise`,
    /// and convert Rust _Future_ into JS _Promise_.
    ///
    /// Remember, `App` holds the context `proxy`,
    /// and static functions in `Proxy` only runs
    /// when they are fed with `proxy`.
    /// For any static functions in `Proxy`
    /// consume `proxy` as their own _context_.
    /// So, when running `Proxy::prepare(this)`
    /// `this` is the _context_.
    ///
    /// Also, by doing `self.proxy.clone()`,
    /// we are just cloning `std::sync::Arc`,
    /// and does not mean that we are cloning
    /// whatever inside.
    #[wasm_bindgen]
    pub fn prepare(&mut self, airports: &JsValue) -> Promise {
        let this = self.proxy.clone();
        let airports = airports.clone();

        future_to_promise(async move {
            match check_dotenv() {
                Ok(_) => Proxy::prepare(this, airports).await,
                Err(_) => {
                    Err(JsValue::from("Faild to read dotenv".to_string()))
                }
            }
        })
    }

    /// JS runs `App::start()` to start
    /// the animation loop. By using
    /// `wasm_bindgen_futures::spawn_local`,
    /// we can asynchronously run `Proxy::run()`.
    ///
    /// Remember, `App` holds the context `proxy`,
    /// and static functions in `Proxy` only runs
    /// when they are fed with `proxy`.
    /// For any static functions in `Proxy`
    /// consume `proxy` as their own _context_.
    /// So, when running `Proxy::prepare(this)`
    /// `this` is the _context_.
    ///
    /// Also, by doing `self.proxy.clone()`,
    /// we are just cloning `std::sync::Arc`,
    /// and does not mean that we are cloning
    /// whatever inside.
    #[wasm_bindgen(method)]
    pub fn start(&mut self) {
        let this = self.proxy.clone();

        spawn_local(async move {
            Proxy::run(this).await;
        });
    }

    /// Whenever JS receives `bounds_changed` events
    /// (of Google Map API), it will run `App::update()`.
    #[wasm_bindgen(method)]
    pub fn update(&mut self, bounds: &JsValue) {
        let bounds = bounds.clone();
        let this = self.proxy.clone();

        spawn_local(async move {
            Proxy::set_bounds(this, bounds).await;
        })
    }
}

fn check_dotenv() -> Result<(), String> {
    try_load_dotenv!();
    Ok(())
}
