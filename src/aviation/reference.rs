/// When retrieved arrivals/departures from FlightAware API,
/// we want to check if they are valid airports.
/// The file provides a  lookup table for validation,
/// and associated structs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use web_sys::console;

use crate::dimension::geo::{GeoCoordTrait, GeoCoord};
use crate::utils::get_json;

/// Airport information stored in our Airport Database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirportRefer {
    pub icao: String,
    pub iata: String,
    pub name: String,
    pub city: String,
    pub country: String,
    pub coord: GeoCoord,
}

impl GeoCoordTrait for AirportRefer {
    fn get_coord(&self) -> GeoCoord { self.coord }
}

impl Default for AirportRefer {
    fn default() -> Self {
        AirportRefer {
            icao: String::from(""),
            iata: String::from(""),
            name: String::from(""),
            city: String::from(""),
            country: String::from(""),
            coord: GeoCoord::default(),
        }
    }
}

lazy_static! {
    /// Airport lookup table against which you can check
    /// whether the specified airport really exists.
    #[derive(Debug)]
    pub static ref AIRPORT_REFERENCE: HashMap<String, AirportRefer> = {
        let json = &include_str!("../../json/airports.json");
        let data = get_json::<Vec<AirportRefer>>(json);

        let mut hashmap = HashMap::new();
        data.iter().for_each(|item| {
            hashmap.insert(
                item.icao.clone(),
                item.clone(),
            );
        });

        hashmap
    };
}

/// See if the specified airport is in the table.
pub fn lookup_airport_database(icao: &str) -> Option<AirportRefer> {
    AIRPORT_REFERENCE.get(icao).cloned()
}
