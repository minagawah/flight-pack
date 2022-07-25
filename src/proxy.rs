/// `Proxy` works for `App`. Interestingly,
/// `Proxy` only provides static functions
/// When these static functions want their
/// own resources, they need to be fed with
/// their contexts, hence, `App` always
/// feeds `this` for these functions.
///
/// Also, notice all the member variables
/// of `Proxy` have their own struct instances.
/// If it had its own member variables,
/// we had to mutate these member variables,
/// then we would have also needed to implement
/// `Arc<Mutex<Proxy>>` for `App::proxy`.
/// However, rather than mutating these member
/// variables in `Proxy`, it would be simple
/// to not directly mutate on `Proxy`, but rather
/// have struct instances, and let them
/// mutate themseves. Hence, a simple `Arc<Proxy>`.
///
/// However, if we wanted to directly mutate
/// `Proxy` and its member variables, then
/// we can do this:
///
/// ```ignore
/// use wasm_mutex::Mutex;
/// use std::ops::Deref;
///
/// fn run(this: Arc<wasm_mutex::Mutex<Proxy>>) {
///   loop {
///     let this = this.lock().await;
///     let this: &Proxy = this.deref();
///   }
/// }
/// ```
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::JsValue;
use web_sys::{
    console,
    HtmlCanvasElement,
};

use crate::manager::Manager;
use crate::dimension::canvas::Canvas;
use crate::dimension::geo::LatLngBounds;
use crate::dimension::window::Window;
use crate::utils::{
    get_ctx,
    request_animation_frame_future,
    timer,
};

#[derive(Debug, Clone)]
pub struct Proxy {
    pub window: Rc<RefCell<Window>>,
    pub canvas: Rc<RefCell<Canvas>>,
    pub bounds: Rc<RefCell<LatLngBounds>>,
    pub manager: Rc<RefCell<Manager>>,
}

#[allow(clippy::await_holding_refcell_ref)]
impl Proxy {
    pub fn new(element: HtmlCanvasElement) -> Self {
        let ctx = get_ctx(&element).unwrap();

        let window = Rc::new(RefCell::new(Window::new()));
        let canvas = Rc::new(RefCell::new(Canvas::new(element, ctx)));
        let bounds = Rc::new(RefCell::new(LatLngBounds::default()));
        let manager = Rc::new(RefCell::new(Manager::new()));

        Proxy {
            window,
            canvas,
            bounds,
            manager,
        }
    }

    /// JS calls `App::prepare`, and this is called.
    /// Asks `Manager` to fetch arrivals/departures
    /// from FlightAware API based on `airports`
    /// given from JS. As it returns geo-coordinates
    /// extracted from the arrivals/departures,
    /// JS will utilize the coordinates
    /// to resize the map bounds.
    pub async fn prepare(
        this: Arc<Proxy>,
        airports: JsValue,
    ) -> Result<JsValue, JsValue> {
        console::log_1(&(
            "[proxy] ++++ prepare()".into()
        ));

        // For the JS given `airports`, calculate
        // for text's width, fetch arrivals/departures
        // from FlightAware API, and extract
        // geo-coordinates from the fetched data
        // to give it back to JS.
        this.manager
            .borrow_mut()
            .prepare(
                &this.canvas.borrow().ctx,
                airports,
            ).await
    }

    /// It runs when JS calls `App::start()`.
    /// Runs a loop for `request_animation_frame`.
    /// Tasks contain:
    ///
    /// (1) Updating canvas size information,
    /// (2) Updating Airports rendering, and
    /// (3) Updating Flights rendering.
    pub async fn run(this: Arc<Proxy>) {
        console::log_1(&(
            "[proxy] ++++ run()".into()
        ));

        // Instead of implementing a commonly
        // used pattern of having a recursive
        // `request_animation_frame()`,
        // we are using `loop` instead
        // to asynchronously wait for tasks!
        loop {
            timer(200).await.unwrap(); // setTimeout() of JS

            // Constantly checking the browser size.
            this.window.borrow_mut().update_size();
            this.canvas.borrow_mut().update_size();

            // We have `join_all` because we may
            // decide to add more async tasks
            // in the future.
            futures::future::join_all(vec![
                // Tell `Manager` for the size updates.
                Box::pin(
                    this.manager.borrow_mut().update(
                        this.canvas.clone(),
                        this.bounds.clone(),
                    )
                ) as Pin<Box<dyn Future<Output = ()>>>,
            ]).await;

            // You can't miss!
            request_animation_frame_future().await;
        }
    }

    /// This one is called whenever bounds change.
    /// (on JS-side)
    pub async fn set_bounds(this: Arc<Proxy>, bounds: JsValue) {
        let bounds: Result<LatLngBounds, serde_json::Error> = bounds.into_serde();
        let bounds: LatLngBounds = match bounds {
            Ok(bounds) => bounds,
            Err(err) => {
                panic!("[app] (bounds) {:?}", err);
            },
        };

        this.bounds.borrow_mut().set(
            bounds.north,
            bounds.east,
            bounds.south,
            bounds.west,
        );
    }
}
