// use chrono::offset::Utc;
// use chrono::DateTime;

pub const DUMMY: bool = true;

pub const HOST: &str = env!("HOST");
pub const ALLOW_ORIGIN: &str = env!("ACCESS_CONTROL_ALLOW_ORIGIN");

pub const AERO_API_URL: &str = "https://aeroapi.flightaware.com/aeroapi";
pub const AERO_API_KEY: &str = env!("AERO_API_KEY");

pub const FONT_SIZE: u8 = 16;
pub const FONT_COLOR: &str = "#ffffff";
pub const FONT_FAMILY: &str = "Work Sans, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Oxygen, Ubuntu, Cantarell, Fira Sans, Droid Sans, Helvetica Neue, sans-serif";

pub const AIRPORT_DOT_RADIUS: f64 = 6.0;
pub const AIRPORT_DOT_LINE_WIDTH: f64 = 1.0;
pub const AIRPORT_DOT_LINE_COLOR: &str = "#ffffff";
pub const AIRPORT_TEXT_WIDTH_DEFAULT: f64 = 50_f64;
pub const AIRPORT_TEXT_HEIGHT_DEFAULT: f64 = 30_f64;

pub const FLIGHT_LINE_WIDTH: f64 = 1.0;
pub const FLIGHT_LINE_COLOR: &str = "#ffffff";
