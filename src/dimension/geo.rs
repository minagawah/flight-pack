use geoutils::Location;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use crate::dimension::Size;
use crate::dimension::point::PointCoord;
use crate::utils::deg_to_rad;

pub trait GeoCoordTrait {
    fn get_coord(&self) -> GeoCoord;
    fn lat(&self) -> f64 { self.get_coord().lat }
    fn lng(&self) -> f64 { self.get_coord().lng }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GeoCoord {
    pub lat: f64,
    pub lng: f64,
}

impl Default for GeoCoord {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl GeoCoord {
    pub fn new(lat: f64, lng: f64) -> Self {
        GeoCoord { lat, lng }
    }

    pub fn geo_coord_from_location(location: Location) -> Self {
        GeoCoord::new(
            location.latitude(),
            location.longitude(),
        )
    }

    pub fn location_from_geo_coord(coord: GeoCoord) -> Location {
        Location::new(
            coord.lat(),
            coord.lng(),
        )
    }

    pub fn lat(&self) -> f64 { self.lat }
    pub fn lng(&self) -> f64 { self.lng }

    pub fn location(&self) -> Location {
        Location::new(
            self.lat(),
            self.lng(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LatLngBounds {
    pub north: f64,
    pub east: f64,
    pub south: f64,
    pub west: f64,
}

impl LatLngBounds {
    pub fn new(
        north: f64,
        east: f64,
        south: f64,
        west: f64,
    ) -> Self {
        LatLngBounds {
            north,
            east,
            south,
            west,
        }
    }

    pub fn set(
        &mut self,
        north: f64,
        east: f64,
        south: f64,
        west: f64,
    ) {
        self.north = north;
        self.east = east;
        self.south = south;
        self.west = west;
    }
}

impl Default for LatLngBounds {
    fn default() -> Self {
        LatLngBounds {
            north: 0.0,
            east: 0.0,
            south: 0.0,
            west: 0.0,
        }
    }
}

pub fn get_center_from_coords(
    coords: Vec<GeoCoord>
) -> GeoCoord {
    let locations: Vec<Box<Location>> = coords
        .iter()
        .map(|coord| Box::new(coord.location()))
        .collect();

    let locations: Vec<&Location> = locations
        .iter()
        .map(Box::as_ref)
        .collect();

    let center: Location = Location::center(locations.as_slice());
    GeoCoord::geo_coord_from_location(center)
}

fn mercator_y(lat: f64) -> f64 {
    (
        ((lat / 2.0) + PI / 4.0).tan()
    ).ln()
}

pub fn get_mercator_position(
    size: &Size,
    bounds: &LatLngBounds,
    coord: &GeoCoord,
) -> PointCoord {
    let width = size.width;
    let height = size.height;

    let north = deg_to_rad(bounds.north);
    let south = deg_to_rad(bounds.south);
    let east = deg_to_rad(bounds.east);
    let west = deg_to_rad(bounds.west);

    let lat = deg_to_rad(coord.lat);
    let lng = deg_to_rad(coord.lng);

    let y_min = mercator_y(south);
    let y_max = mercator_y(north);

    let x_factor = width / (east - west);
    let y_factor = height / (y_max - y_min);

    let x = (lng - west) * x_factor;
    let y = (y_max - mercator_y(lat)) * y_factor;

    PointCoord::new(x, y)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn checking_get_center_from_coords() {
//         let coords: Vec<GeoCoord> = vec![
//             GeoCoord::new(52.518611, 13.408056), // berlin
//             GeoCoord::new(55.751667, 37.617778), // moscow
//         ];
//         let res = get_center_from_coords(coords);
//         assert_eq!(res.latitude(), 10.0);
//         assert_eq!(res.longitude(), 10.0);
//     }
// }
