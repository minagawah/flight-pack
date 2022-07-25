/// `Manager` is in charge of 3 jobs:
///
/// (1) Managing `AirportGraphics`,
/// (2) Convert `airports` fed by JS, and
/// (3) Fetching arrival/departure info from FlightAware API.

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{
    console,
    CanvasRenderingContext2d,
};

use crate::aviation::airport::{
    TargetAirportRawData,
    Airport,
};
use crate::aviation::arrival::{
    AeroArrivalsRawData,
    AirportArrival,
    fetch_arrivals,
};
use crate::aviation::flight::Flight;
use crate::dimension::canvas::Canvas;
use crate::dimension::geo::{
    LatLngBounds,
    // GeoCoordTrait,
    GeoCoord,
};
use crate::dimension::Size;

type ArrivalsResponse = Result<AeroArrivalsRawData, String>;
type ArrivalsResponsePinBox = Pin<Box<dyn Future<Output = ArrivalsResponse>>>;

#[derive(Debug)]
pub struct Manager {
    canvas: Size,
    bounds: LatLngBounds,
    airports: Vec<Airport>,
    airport_icaos: Vec<String>,
    arrivals: Vec<AirportArrival>,
    flights: Vec<Flight>,
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            canvas: Size::new(1.0, 1.0), // !!!
            bounds: LatLngBounds::default(),
            airports: vec![],
            airport_icaos: vec![],
            arrivals: vec![],
            flights: vec![],
        }
    }

    /// JS is calling `App::prepare`, and is calling `Proxy::prepare`,
    /// and this is called. Following tasks are carried out:
    ///
    /// (1) Convert `airports` into Rust data.
    /// (2) For `airports`, fetch arrival/departure information.
    pub async fn prepare(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        airports: JsValue,
    ) -> Result<JsValue, JsValue> {
        console::log_1(&(
            "[manager] ++++ prepare()".into()
        ));
        self.set_airports(airports);
        self.set_text_width(ctx);
        self.set_arrivals().await;
        self.set_flights();
        self.get_coords_from_arrivals()
    }

    /// Convert the JS given `airports` into Rust data.
    /// Also, calculate for texts' width prior to the actual render.
    pub fn set_airports(&mut self, airports: JsValue) {
        console::log_1(&(
            "[manager] ++++ set_airports()".into()
        ));
        let airports: Result<Vec<TargetAirportRawData>, serde_json::Error> =
            airports.into_serde();

        let airports: Vec<TargetAirportRawData> = airports
            .map_or_else(
                |err| panic!("[manager] (airports) {:?}", err),
                |airport| airport,
            );

        self.airports = airports.iter()
            .map(|raw| {
                self.airport_icaos.push(raw.icao.clone());
                Airport::new(raw.clone())
            })
            .collect::<Vec<Airport>>();
    }

    /// Runs a set of tests to check the text width for airport names.
    fn set_text_width(&mut self, ctx: &CanvasRenderingContext2d) {
        self.airports.iter_mut().for_each(|p| {
            p.set_text_width(ctx);
        });
    }

    /// For `airports` (which is JS given), we will fetch
    /// arrivals/departures from FlightAware API.
    async fn set_arrivals(&mut self) {
        console::log_1(&(
            "[manager] ++++ set_arrivals()".into()
        ));
        let mut request: Vec<Box<dyn Fn() -> ArrivalsResponsePinBox>> = vec![];

        // In the next line, no actual fetchings are executed,
        // but just constructing Rust's `Future`s.
        // They will later be executed using `await`.
        self.airports.iter().for_each(|p| {
            request.push(
                Box::new(move || Box::pin(
                    fetch_arrivals(
                        p.icao.clone()
                    )
                ))
            );
        });

        // Let's fetch using `await`!
        // (we have `join_all` so that we could add others in the future)
        let response: Vec<Result<AeroArrivalsRawData, String>> =
            futures::future::join_all(
                request.into_iter().map(|r| r())
            ).await;

        // Ignore errors and OK all.
        let rawdata: Vec<AeroArrivalsRawData> = response.iter()
            .filter_map(|res| res.clone().ok())
            .collect();

        // Set only the arrivals/departures that are valid.
        rawdata.iter().for_each(|airport| {
            airport.arrivals.iter().for_each(|arrival| {
                if let Some(res) = arrival.extract(&self.airport_icaos) {
                    self.arrivals.push(res);
                }
            });
        });

        self.arrivals.iter().enumerate().for_each(|(i, arrival)| {
            console::log_1(&(
                "[manager] ---------------".into()
            ));
            console::log_1(&(
                format!(
                    "[manager] [{}] (departure) {} ({}, {})",
                    i,
                    arrival.orig_airport.name,
                    arrival.orig_airport.city,
                    arrival.orig_airport.country,
                ).into()
            ));

            console::log_1(&(
                format!(
                    "[manager] [{}] (arrival) {} ({}, {})",
                    i,
                    arrival.dest_airport.name,
                    arrival.dest_airport.city,
                    arrival.dest_airport.country,
                ).into()
            ));
        });

        console::log_1(&(
            format!(
                "[manager] Total Arrivals: {}",
                self.arrivals.len()
            ).into()
        ));
    }

    fn set_flights(&mut self) {
        console::log_1(&(
            "[manager] ++++ set_flights()".into()
        ));
        self.arrivals.clone().iter()
            .for_each(|arrival| {
                self.flights.push(
                    Flight::new(arrival.clone())
                );
            });
    }

    fn _get_coords_from_airports(&self) -> Result<JsValue, JsValue> {
        let coords: Vec<GeoCoord> =
            self.airports
            .iter()
            .map(|p| p.coord)
            .collect();

        serde_json::to_string(&coords)
            .map(JsValue::from)
            .map_err(|_| JsValue::from(
                "Failed to serialize arrivals data".to_string()
            ))
    }

    fn get_coords_from_arrivals(&self) -> Result<JsValue, JsValue> {
        let mut coords: Vec<GeoCoord> = vec![];

        self.arrivals.iter().for_each(|arrival| {
            coords.push(arrival.orig_airport.coord);
            coords.push(arrival.orig_airport.coord);
        });

        serde_json::to_string(&coords)
            .map(JsValue::from)
            .map_err(|_| JsValue::from(
                "Failed to serialize arrivals data".to_string()
            ))
    }

    fn is_update_needed(
        &self,
        canvas: &Canvas,
        bounds: &LatLngBounds,
    ) -> bool {
        canvas.width != self.canvas.width ||
            canvas.height != self.canvas.height ||
            bounds.north != self.bounds.north ||
            bounds.east != self.bounds.east ||
            bounds.south != self.bounds.south ||
            bounds.west != self.bounds.west
    }

    fn set_canvas(&mut self, canvas: &Canvas) {
        self.canvas.width = canvas.width;
        self.canvas.height = canvas.height;
    }

    fn set_bounds(&mut self, bounds: &LatLngBounds) {
        self.bounds.north = bounds.north;
        self.bounds.east = bounds.east;
        self.bounds.south = bounds.south;
        self.bounds.west = bounds.west;
    }

    // `Proxy::run()` has a loop, and it constantly calls
    // this function from there. Basically, whenever
    // information for `canvas` or `bounds` changes,
    // we will be updating its own, and will run
    // `AirportGraphics::update()` for each airport.
    pub async fn update(
        &mut self,
        canvas: Rc<RefCell<Canvas>>,
        bounds: Rc<RefCell<LatLngBounds>>,
    ) {
        let clone = canvas.clone();
        let canvas = canvas.borrow();
        let bounds = bounds.borrow();

        if self.is_update_needed(&canvas, &bounds) {
            self.set_canvas(&canvas);
            self.set_bounds(&bounds);

            self.airports.iter_mut().for_each(|p| {
                p.update(&canvas, &bounds);
            });

            self.flights.iter_mut().for_each(|f| {
                f.update(&canvas, &bounds);
            });
        }

        self.draw(&clone.borrow().ctx);
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        self.airports.iter().for_each(|p| {
            p.draw(ctx);
        });
        self.flights.iter().for_each(|f| {
            f.draw(ctx);
        });
        ctx.restore();
    }
}
