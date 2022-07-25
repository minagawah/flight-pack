use wasm_bindgen::prelude::JsValue;
use web_sys::{
    // console,
    CanvasRenderingContext2d,
};

use crate::aviation::arrival::AirportArrival;
use crate::aviation::reference::AirportRefer;
use crate::constants::{
    FLIGHT_LINE_COLOR,
    FLIGHT_LINE_WIDTH,
};
use crate::dimension::canvas::Canvas;
use crate::dimension::geo::LatLngBounds;
use crate::dimension::point::PointCoord;
use crate::dimension::geo::get_mercator_position;

const DEFAULT_ACCEL: f64 = 0.01;
const DEFAULT_DECEL: f64 = 0.96;

#[derive(Debug)]
pub struct Flight {
    pub icao: String,
    pub iata: String,
    pub operator: String,
    pub flight_number: String,
    pub orig_airport: AirportRefer,
    pub dest_airport: AirportRefer,
    pub index: usize,
    pub orig: PointCoord,
    pub dest: PointCoord,
    pub velocity: PointCoord,
    pub vmax: f64,
    pub speed: f64,
    pub angle: f64,
    pub accel: f64,
    pub decel: f64,
    pub dest_index: u8,
    pub approaching: bool,
    pub holding: bool,
}

impl Flight {
    pub fn new(arrival: AirportArrival) -> Self {
        Flight {
            icao: arrival.icao,
            iata: arrival.iata,
            operator: arrival.operator,
            flight_number: arrival.flight_number,
            orig_airport: arrival.orig_airport,
            dest_airport: arrival.dest_airport,
            index: 0_usize,
            orig: PointCoord::default(),
            dest: PointCoord::default(),
            velocity: PointCoord::default(),
            vmax: 1_f64,
            speed: 0_f64,
            angle: 0_f64,
            accel: DEFAULT_ACCEL,
            decel: DEFAULT_DECEL,
            dest_index: 0_u8,
            approaching: false,
            holding: false,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        canvas: &Canvas,
        bounds: &LatLngBounds,
    ) {
        self.orig = get_mercator_position(
            &canvas.size(),
            bounds,
            &self.orig_airport.coord,
        );

        self.dest = get_mercator_position(
            &canvas.size(),
            bounds,
            &self.dest_airport.coord,
        );
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        ctx.set_stroke_style(
            &JsValue::from_str(FLIGHT_LINE_COLOR)
        );
        ctx.set_line_width(FLIGHT_LINE_WIDTH);
        ctx.begin_path();
        ctx.move_to(self.orig.x, self.orig.y);
        ctx.line_to(self.dest.x, self.dest.y);
        ctx.stroke();
        ctx.restore();
    }
}
