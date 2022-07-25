/// Contains structs and functions that manages either
/// of the following data types:
///
/// (1) For airports fed by JS that we want them plotted on Google map, or
/// (2) For arrival/departure information fetched from FlightAware API.
/// (3) For airport database so that allows us to validate airports.

#[allow(clippy::module_inception)]
pub mod airport;
pub mod arrival;
pub mod flight;
pub mod reference;
