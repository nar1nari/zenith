use crate::bodies::{Body, BodyPosition};
use siderust::astro::JulianDate;

pub fn calculate_alt_az(
    julian_date: JulianDate,
    body: &Body,
    earth: &BodyPosition,
    observer_lat: f64,
    lst: f64,
) -> (f64, f64) {
    let barycentric = (body.position_fn)(julian_date);
    let geo_vector = [
        (barycentric.position.x() - earth.position.x()).value(),
        (barycentric.position.y() - earth.position.y()).value(),
        (barycentric.position.z() - earth.position.z()).value(),
    ];
    let (ra, dec) = ecliptic_to_equatorial(geo_vector);
    equatorial_to_horizontal(ra, dec, observer_lat, lst)
}

pub fn ecliptic_to_equatorial(v: [f64; 3]) -> (f64, f64) {
    let obliquity = 23.43929111f64.to_radians();
    let (x, y, z) = (
        v[0],
        v[1] * obliquity.cos() - v[2] * obliquity.sin(),
        v[1] * obliquity.sin() + v[2] * obliquity.cos(),
    );
    let ra = y.atan2(x);
    let dec = (z / (x * x + y * y + z * z).sqrt()).asin();
    (ra, dec)
}

pub fn calculate_local_sidereal_time(jd: f64, longitude: f64) -> f64 {
    let days_since_j2000 = jd - 2451545.0;
    let gmst = (280.46061837 + 360.98564736629 * days_since_j2000).to_radians();
    (gmst + longitude.to_radians()).rem_euclid(2.0 * std::f64::consts::PI)
}

pub fn equatorial_to_horizontal(ra: f64, dec: f64, lat: f64, lst: f64) -> (f64, f64) {
    let ha = lst - ra;
    let lat_rad = lat.to_radians();
    let altitude = (dec.sin() * lat_rad.sin() + dec.cos() * lat_rad.cos() * ha.cos()).asin();
    // it just werks
    let azimuth = ha
        .sin()
        .atan2(ha.cos() * lat_rad.sin() - dec.tan() * lat_rad.cos());
    (azimuth.rem_euclid(2.0 * std::f64::consts::PI), altitude)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    #[test]
    fn test_ecliptic_to_equatorial_equator() {
        let (ra, dec) = ecliptic_to_equatorial([1.0, 0.0, 0.0]);
        assert!(approx_eq(ra, 0.0));
        assert!(approx_eq(dec, 0.0));
    }

    #[test]
    fn test_equatorial_to_horizontal_star_overhead() {
        let ra = 0.0;
        let dec = 0.0;
        let lat = 0.0;
        let lst = 0.0;

        let (_, alt) = equatorial_to_horizontal(ra, dec, lat, lst);

        assert!(approx_eq(alt.to_degrees(), 90.0));
    }

    #[test]
    fn test_sidereal_time_j2000() {
        let lst = calculate_local_sidereal_time(2451545.0, 0.0);
        let deg = lst.to_degrees();
        assert!((deg - 280.46061837).abs() < 1e-6);
    }
}
