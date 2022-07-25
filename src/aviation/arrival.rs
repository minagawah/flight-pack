/// For airports given from JS, WASM app fetches arrival/departure
/// information from FlightAware API. This file provides
/// associated structs and functions.

use chrono::offset::{Utc, TimeZone};
use chrono::{
    DateTime,
    Datelike,
    Duration,
};
use serde::de;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::console;

use crate::aviation::reference::{
    AirportRefer,
    lookup_airport_database,
};
use crate::constants::{
    DUMMY,
    AERO_API_URL,
    AERO_API_KEY,
};
use crate::request::fetch;
use crate::utils::get_json;

/// Deserialiation rules for date/time in arrival/departure information.
fn from_rfc3339_z<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
where D: de::Deserializer<'de>,
{
    Deserialize::deserialize(d)
        .map(|opt: Option<&str>| {
            match opt {
                Some(s) => {
                    let parsed = s.parse::<DateTime<Utc>>();
                    match parsed {
                        Ok(dt) => Some(dt),
                        Err(_) => None,
                    }
                },
                _ => None,
            }
        })
}

/// Once extracted from `AeroArrivalsRawData`,
/// this is the data structure we want for the app.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirportArrival {
    pub id: String, // (ORIGINAL) fa_flight_id
    pub icao: String, // (ORIGINAL) ident_icao (in `Option`)
    pub iata: String, // (ORIGINAL) ident_iata (in `Option`)
    pub operator: String, // (ORIGINAL) operator (in `Option`)
    pub flight_number: String, // (ORIGINAL) flight_number (in `Option`)

    // (ORIGIN) origin (which was `AeroAirportRawData`)
    pub orig_airport: AirportRefer,

    // (ORIGIN) destination (which was `AeroAirportRawData`)
    pub dest_airport: AirportRefer,

    pub actual_out: DateTime<Utc>, // Gate -> Departure
    pub actual_off: DateTime<Utc>, // Runaway -> Departure

    pub scheduled_on: DateTime<Utc>, // Runaway -> Arrival
    pub estimated_on: DateTime<Utc>, // Runaway -> Arrival
    pub scheduled_in: DateTime<Utc>, // Gate -> Arrival
    pub estimated_in: DateTime<Utc>, // Gate -> Arrival

    pub progress_percent: i32,
    pub route_distance: i32, // (ORIGINAL) Option<route_distance>
}

/// This is how arrival/departure information look like
/// when received from FlightAware API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AeroArrivalsRawData {
    // links: AeroLinkRawData, // NOT IN USE
    // num_pages: i32, // NOT IN USE
    pub arrivals: Vec<AeroArrivalsActualRawData>,
}

/// The above raw data has a nested structure, but we
/// only want `arrivals`, and this is how it looks like.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AeroArrivalsActualRawData {
    // Either the operator code followed by the flight number
    // for the flight (for commercial flights)
    // or the aircraft's registration (for general aviation).
    pub ident: String,

    // The ICAO operator code followed by the flight number
    // for the flight (for commercial flights)
    pub ident_icao: Option<String>,

    // The IATA operator code followed by the flight number
    // for the flight (for commercial flights)
    pub ident_iata: Option<String>,

    // Unique identifier assigned by Aero for this
    // specific flight. If the flight is diverted, the new leg
    // of the flight will have a different fa_flight_id.
    pub fa_flight_id: String,

    // ICAO code, if exists, of the operator of the flight,
    // otherwise the IATA code
    pub operator: Option<String>,

    // ICAO code of the operator of the flight.
    pub operator_icao: Option<String>,

    // IATA code of the operator of the flight.
    pub operator_iata: Option<String>,

    // Bare flight number of the flight.
    pub flight_number: Option<String>,

    // Aircraft registration (tail number) of the aircraft, when known.
    pub registration: Option<String>,

    // The ident of the flight for Air Traffic Control purposes,
    // when known and different than ident.
    pub atc_ident: Option<String>,

    // Unique identifier assigned by Aero for
    // the previous flight of the aircraft serving this flight.
    pub inbound_fa_flight_id: Option<String>,

    // List of any ICAO codeshares operating on this flight.
    pub codeshares: Vec<String>,

    // List of any IATA codeshares operating on this flight.
    pub codeshares_iata: Vec<String>,

    // Flag indicating whether this flight is blocked from public viewing.
    pub blocked: bool,

    // Flag indicating whether this flight was diverted.
    pub diverted: bool,

    // Flag indicating that the flight is no longer being tracked
    // by Aero. There are a number of reasons this could
    // happen including cancellation by the airline, but that
    // will not always be the case.
    pub cancelled: bool,

    // Flag indicating whether this flight has a flight plan,
    // schedule, or other indication of intent available.
    pub position_only: bool,

    // Information for this flight's origin airport.
    pub origin: AeroAirportRawData,

    //  Information for this flight's destination airport.
    pub destination: AeroAirportRawData,

    // Arrival delay (in seconds) based on either actual
    // or estimated gate arrival time. If gate time
    // is unavailable then based on runway arrival time.
    // A negative value indicates the flight is early.
    pub departure_delay: Option<i32>,

    // Departure delay (in seconds) based on either actual
    // or estimated gate departure time. If gate time
    // is unavailable then based on runway departure time.
    // A negative value indicates the flight is early.
    pub arrival_delay: Option<i32>,

    // Runway-to-runway filed duration (seconds).
    pub filed_ete: Option<i32>,

    // Scheduled gate departure time.
    #[serde(deserialize_with = "from_rfc3339_z")]
    pub scheduled_out: Option<DateTime<Utc>>, // OR NULL

    // Estimated gate departure time.
    #[serde(deserialize_with = "from_rfc3339_z")]
    pub estimated_out: Option<DateTime<Utc>>, // OR NULL

    // Actual gate departure time.
    #[serde(deserialize_with = "from_rfc3339_z")]
    pub actual_out: Option<DateTime<Utc>>, // OR NULL

    // Scheduled runway departure time.
    #[serde(deserialize_with = "from_rfc3339_z")]
    pub scheduled_off: Option<DateTime<Utc>>, // OR NULL

    // Estimated runway departure time.
    #[serde(deserialize_with = "from_rfc3339_z")]
    pub estimated_off: Option<DateTime<Utc>>, // OR NULL

    // Actual runway departure time.
    #[serde(deserialize_with = "from_rfc3339_z")]
    pub actual_off: Option<DateTime<Utc>>, // OR NULL

    // Scheduled runway arrival time.
    #[serde(deserialize_with = "from_rfc3339_z")]
    pub scheduled_on: Option<DateTime<Utc>>, // OR NULL

    // Estimated runway arrival time.
    #[serde(deserialize_with = "from_rfc3339_z")]
    pub estimated_on: Option<DateTime<Utc>>, // OR NULL

    // Actual runway arrival time.
    #[serde(deserialize_with = "from_rfc3339_z")]
    pub actual_on: Option<DateTime<Utc>>, // OR NULL

    // Scheduled gate arrival time.
    #[serde(deserialize_with = "from_rfc3339_z")]
    pub scheduled_in: Option<DateTime<Utc>>, // OR NULL

    // Estimated gate arrival time.
    #[serde(deserialize_with = "from_rfc3339_z")]
    pub estimated_in: Option<DateTime<Utc>>, // OR NULL

    // Actual gate arrival time.
    #[serde(deserialize_with = "from_rfc3339_z")]
    pub actual_in: Option<DateTime<Utc>>, // OR NULL

    // The percent completion of a flight, based on
    // runway departure/arrival. Null for en route position-only flights.
    // Constraints: Min 0┃Max 100
    pub progress_percent: Option<i32>,

    // Human-readable summary of flight status.
    pub status: String,

    // Aircraft type will generally be ICAO code, but IATA code
    // will be given when the ICAO code is not known.
    pub aircraft_type: Option<String>,

    // Planned flight distance (statute miles) based on the filed route.
    // May vary from actual flown distance.
    pub route_distance: Option<i32>,

    // Filed IFR airspeed (knots).
    pub filed_airspeed: Option<i32>,

    // Filed IFR altitude (100s of feet).
    pub filed_altitude: Option<i32>,

    // The textual description of the flight's route.
    pub route: Option<String>,

    // Baggage claim location at the destination airport.
    pub baggage_claim: Option<String>,

    // Number of seats in the business class cabin.
    pub seats_cabin_business: Option<i32>,

    // Number of seats in the coach cabin.
    pub seats_cabin_coach: Option<i32>,

    // Number of seats in the first class cabin.
    pub seats_cabin_first: Option<i32>,

    // Departure gate at the origin airport.
    pub gate_origin: Option<String>,

    // Arrival gate at the destination airport.
    pub gate_destination: Option<String>,

    // Departure terminal at the origin airport.
    pub terminal_origin: Option<String>,

    // Arrival terminal at the destination airport.
    pub terminal_destination: Option<String>,
}

impl AeroArrivalsActualRawData {
    /// For retrieved arrivals/departures, we want to
    /// iterate each, validating the key-values there, and:
    ///
    /// (1) Check if the origin/destination airports are valid,
    /// (2) Check if these airports are the ones plotted on Google Map, and
    /// (3) Check if the arrival/departure falls under the current time window.
    #[allow(clippy::unnecessary_unwrap)]
    pub fn extract(
        &self,
        airport_icaos: &[String],
    ) -> Option<AirportArrival> {
        let now: DateTime<Utc> = Utc::now();

        let sec_1: i64 = Utc.ymd(
            now.year(),
            now.month(),
            now.day(),
        ).and_hms(0, 0, 0).timestamp();

        let mut result: Option<AirportArrival> = None;

        let orig_airport: Option<AirportRefer> =
            if self.origin.code_icao.is_some() {
                lookup_airport_database(
                    &self.clone().origin.code_icao.unwrap()
                )
            } else {
                None
            };

        let dest_airport: Option<AirportRefer> =
            if self.destination.code_icao.is_some() {
                lookup_airport_database(
                    &self.clone().destination.code_icao.unwrap()
                )
            } else {
                None
            };

        // General checks on the fields.
        if !self.cancelled &&
            self.ident_icao.is_some() &&
            self.ident_iata.is_some() &&
            orig_airport.is_some() &&
            dest_airport.is_some() &&
            // self.actual_out.is_some() &&
            // self.actual_off.is_some() &&
            self.scheduled_on.is_some() &&
            // self.estimated_on.is_some() &&
            self.scheduled_in.is_some() &&
            // self.estimated_in.is_some() &&
            self.operator.is_some() &&
            self.flight_number.is_some() &&
            self.route_distance.is_some()
        {
            let orig_airport: AirportRefer = orig_airport.unwrap();
            let dest_airport: AirportRefer = dest_airport.unwrap();

            let mut delta = Duration::seconds(0);

            // For DUMMY, we manipulate time.
            if DUMMY {
                let d: DateTime<Utc> = self.scheduled_out.unwrap();
                let sec_0: i64 = Utc.ymd(
                    d.year(),
                    d.month(),
                    d.day(),
                ).and_hms(0, 0, 0).timestamp();
                delta = Duration::seconds(sec_1 - sec_0);
            }

            // let actual_out = self.actual_out.unwrap() + delta;
            // let actual_off = self.actual_off.unwrap() + delta;
            let actual_out = self.scheduled_out.unwrap() + delta;
            let actual_off = self.scheduled_off.unwrap() + delta;
            let scheduled_on = self.scheduled_on.unwrap() + delta;
            // let estimated_on = self.estimated_on.unwrap() + delta;
            let estimated_on = self.scheduled_on.unwrap() + delta;
            let scheduled_in = self.scheduled_in.unwrap() + delta;
            // let estimated_in = self.estimated_in.unwrap() + delta;
            let estimated_in = self.scheduled_in.unwrap() + delta;

            // Check if origin/destination airports are
            // in the JS given list of airports. Also,
            // check if arrival/departure falls under
            // the current time window.
            if airport_icaos.to_owned().iter().any(|icao| {
                icao == orig_airport.icao.as_str()
            }) && airport_icaos.to_owned().iter().any(|icao| {
                icao == dest_airport.icao.as_str()
            }) {
                // actual_out < now && scheduled_in > now

                let clone = self.clone();

                let id: String = clone.fa_flight_id;
                let icao: String = clone.ident_icao.unwrap();
                let iata: String = clone.ident_iata.unwrap();
                let operator: String = clone.operator.unwrap();
                let flight_number: String = clone.flight_number.unwrap();
                let progress_percent: i32 =
                    clone.progress_percent.unwrap_or(0);
                let route_distance: i32 = clone.route_distance.unwrap();

                result = Some(
                    AirportArrival {
                        id,
                        icao,
                        iata,
                        operator,
                        flight_number,
                        orig_airport,
                        dest_airport,
                        actual_out,
                        actual_off,
                        scheduled_on,
                        estimated_on,
                        scheduled_in,
                        estimated_in,
                        progress_percent,
                        route_distance,
                    }
                );
            }
            // END OF: if (2)
        }
        // END OF: if (1)

        result
    }
    // END OF: extract()
}
// END OF: impl AeroArrivalsActualRawData

/// `AeroArrivalsActualRawData` has `origin`
/// and `destination`, and this is the structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AeroAirportRawData {
    pub code: Option<String>,
    pub code_icao: Option<String>,
    pub code_iata: Option<String>,
    pub code_lid: Option<String>,
    pub airport_info_url: Option<String>,
}

lazy_static! {
    #[derive(Debug)]
    pub static ref DUMMY_ARRIVALS: HashMap<String, AeroArrivalsRawData> = {
        let mut hashmap = HashMap::new();

        hashmap.insert(
            "VVTS".into(), // SGN (Tan Son Nhat, Saigon)
            get_json::<AeroArrivalsRawData>(
                include_str!("../../json/arrivals_saigon.json")
            )
        );

        hashmap.insert(
            "RCTP".into(), // TPE (Taiwan Taoyuan, Taipei)
            get_json::<AeroArrivalsRawData>(
                include_str!("../../json/arrivals_taiwan.json")
            )
        );

        hashmap.insert(
            "VHHH".into(), // (Hong Kong)
            get_json::<AeroArrivalsRawData>(
                include_str!("../../json/arrivals_hongkong.json")
            )
        );

        hashmap.insert(
            "WSSS".into(), // SIN (Changi, Singapore)
            get_json::<AeroArrivalsRawData>(
                include_str!("../../json/arrivals_changi.json")
            )
        );

        hashmap.insert(
            "RPLL".into(), // MNL (Manila, Philippines)
            get_json::<AeroArrivalsRawData>(
                include_str!("../../json/arrivals_manila.json")
            )
        );

        hashmap
    };
}

pub async fn fetch_arrivals(icao: String) ->
    Result<AeroArrivalsRawData, String>
{
    let icao = icao.as_str();

    if DUMMY {
        match DUMMY_ARRIVALS.get(icao) {
            Some(arrival) => {
                Ok(arrival.clone())
            },
            None => {
                Err("Failed to get \"DUMMY_ARRIVALS\"".into())
            },
        }
    } else {
        let url: String = format!(
            "{}/airports/{}/flights/arrivals",
            AERO_API_URL,
            icao,
        );

        let mut headers: HashMap<String, String> = HashMap::new();

        headers.insert(
            "X-Apikey".into(),
            AERO_API_KEY.to_string(),
        );

        match fetch(
            url.as_str(),
            Some(headers),
        ).await {
            Ok(json) => {
                Ok(json.into_serde().unwrap())
            }
            Err(e) => {
                let default_err: String = String::from("Error");
                let err: String = e.as_string().unwrap_or(default_err);
                console::error_1(&(
                    format!("{} for: {}", err, url).into()
                ));
                Err(err)
            }
        }
    }
}
