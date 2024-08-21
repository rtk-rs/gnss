/*
 * gnss.rs
 * Define new planetary datasets to enhance ANISE datasets
 *
 * rtk-rs @github.com
 */
use std::path::PathBuf;

use anise::{
    //constants::celestial_objects::EARTH,
    constants::frames::EARTH_J2000,
    prelude::MetaAlmanac,
    structure::planetocentric::ellipsoid::Ellipsoid,
};

fn ellipsoid(flatenning: f64, semi_major_km: f64, semi_minor_km: f64) -> Ellipsoid {
    let r = (semi_major_km + semi_minor_km) / 2.0;
    Ellipsoid {
        polar_radius_km: r - flatenning * r,
        semi_major_equatorial_radius_km: semi_major_km,
        semi_minor_equatorial_radius_km: semi_minor_km,
    }
}

fn main() {
    let gps_wgs84 = ellipsoid(298.257223563_f64, 6378.137_f64, 6356.7523142_f64);

    let mut almanac =
        MetaAlmanac::latest().unwrap_or_else(|e| panic!("MetaAlmanac::latest() error: {}", e));

    for (frame_id, frame_name, shape) in [(3990001, "GPS WGS84", gps_wgs84)] {
        let mut earth_inertial = almanac
            .frame_from_uid(EARTH_J2000)
            .unwrap_or_else(|e| panic!("failed to retrieve Earth from_uid: {}", e));

        earth_inertial.shape = Some(shape);

        almanac
            .planetary_data
            .push(earth_inertial, Some(frame_id), Some(frame_name))
            .unwrap_or_else(|e| panic!("planetary_data.push() error: {}", e));

        let path = PathBuf::new().join("pck_gps_wgs84.pca");
        almanac
            .planetary_data
            .save_as(&path, true)
            .unwrap_or_else(|e| panic!("planetary_data.save_as() error: {}", e));
    }
}
