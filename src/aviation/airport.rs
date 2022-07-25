/// JS calls `app.prepare(airports)` where `airports`
/// is a list of airports to be plotted on Google Map.
/// This file provides associated structs.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use wasm_bindgen::prelude::JsValue;
use web_sys::{
    // console,
    CanvasRenderingContext2d,
    TextMetrics,
};

use crate::constants::{
    FONT_SIZE,
    FONT_COLOR,
    FONT_FAMILY,
    AIRPORT_DOT_LINE_COLOR,
    AIRPORT_DOT_LINE_WIDTH,
    AIRPORT_TEXT_WIDTH_DEFAULT,
    AIRPORT_TEXT_HEIGHT_DEFAULT,
};
use crate::dimension::canvas::Canvas;
use crate::dimension::geo::{
    GeoCoordTrait,
    GeoCoord,
    LatLngBounds,
};
use crate::dimension::point::PointCoord;
use crate::dimension::geo::get_mercator_position;
use crate::dimension::{Size, get_window_size};
use crate::utils::lazy_round;

const AIRPORT_DOT_START: f64 = 0.0;
const AIRPORT_DOT_END: f64 = PI * 2.0;

/// When `airport` (airports to plot on Google Map)
/// is given from JS, this is how it looks.
/// For them, we will later fetch arrival/departure
/// information from FlightAware API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetAirportRawData {
    pub icao: String, // VHHH
    pub iata: String, // HKG
    pub name: String, // Hong Kong International
    pub city: String, // Hong Kong
    pub country: String, // Hong Kong
    pub coord: GeoCoord,
}

/// Once converted from `TargetAirportRawData`,
/// this is the data structure we want for the app.
#[derive(Debug)]
pub struct Airport {
    pub icao: String,
    pub iata: String,
    pub name: String,
    pub city: String,
    pub country: String,
    pub coord: GeoCoord,
    pub radius: f64,
    pub pos: PointCoord,
    pub text_width: f64,
    pub text_height: f64,
    pub text_pos: PointCoord,
    pub font_size: u8,
}

impl GeoCoordTrait for Airport {
    fn get_coord(&self) -> GeoCoord { self.coord }
}

impl Airport {
    pub fn new(raw: TargetAirportRawData) -> Self {
        Airport {
            icao: raw.icao,
            iata: raw.iata,
            name: raw.name,
            city: raw.city,
            country: raw.country,
            coord: raw.coord,
            radius: 0_f64,
            pos: PointCoord::default(),
            text_width: 0_f64,
            text_height: 0_f64,
            text_pos: PointCoord::default(),
            font_size: FONT_SIZE,
        }
    }

    pub fn get_airport_text(&self) -> String {
        format!(
            "{} ({})",
            self.name.as_str(),
            self.city.as_str(),
        )
    }

    // This is called only once in `Manager::new()`.
    // We want to run a test to check text width
    // for airport names to be later rendered.
    pub fn set_text_width(
        &mut self,
        ctx: &CanvasRenderingContext2d,
    ) {
        let window_size: Size = get_window_size();
        let font_size: u8 = if window_size.width < 468.0 {
            ((FONT_SIZE as f64) * 2.8) as u8
        } else {
            FONT_SIZE
        };
        self.font_size = font_size;

        ctx.save();
        ctx.set_font(
            format!("{}px {}", font_size, FONT_FAMILY).as_str()
        );

        let (width, height): (f64, f64) = ctx
            .measure_text(&self.get_airport_text())
            .map_or(
                (
                    AIRPORT_TEXT_WIDTH_DEFAULT,
                    AIRPORT_TEXT_HEIGHT_DEFAULT
                ),
                |metrics: TextMetrics| {
                    let h = metrics.actual_bounding_box_ascent() +
                        metrics.actual_bounding_box_descent();
                    let w = metrics.width();
                    (w, h)
                }
            );

        ctx.restore();

        self.text_width = lazy_round(width);
        self.text_height = lazy_round(height);
    }

    // Updating the airport positions.
    // Positions need Mercator conversions.
    // This is called in `Manager::update()`.
    pub fn update(
        &mut self,
        canvas: &Canvas,
        bounds: &LatLngBounds,
    ) {
        self.pos = get_mercator_position(
            &canvas.size(),
            bounds,
            &self.coord,
        );

        let mut x: f64 = self.pos.x - (self.text_width * 0.8);

        if x < 0.0 {
            x = 20.0;
        }

        if (x + self.text_width) > canvas.width {
            x = canvas.width - self.text_width - 20.0;
        }

        self.text_pos.x = x;
        self.text_pos.y = self.pos.y - self.text_height;
    }

    // Called in `Manager::draw()`.
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let font_style: String =
            format!("{}px {}", self.font_size, FONT_FAMILY);

        // Airport Names
        ctx.save();
        ctx.set_fill_style(&JsValue::from_str(FONT_COLOR));
        ctx.set_font(font_style.as_str());
        ctx.fill_text(
            &self.get_airport_text(),
            self.text_pos.x,
            self.text_pos.y
        ).unwrap_or(());
        ctx.restore();

        // Airport Dots
        ctx.save();
        ctx.set_stroke_style(
            &JsValue::from_str(AIRPORT_DOT_LINE_COLOR)
        );
        ctx.set_line_width(AIRPORT_DOT_LINE_WIDTH);
        ctx.begin_path();
        ctx.arc(
            self.pos.x,
            self.pos.y,
            self.radius,
            AIRPORT_DOT_START,
            AIRPORT_DOT_END,
        ).unwrap_or(());
        ctx.stroke();
        ctx.restore();
    }
}
