use siderust::{
    astro::JulianDate,
    bodies::{Jupiter, Mars, Mercury, Neptune, Saturn, Sun, Uranus, Venus},
    coordinates::{cartesian::Position, centers::Barycentric, frames::Ecliptic},
    targets::Target,
    units::AstronomicalUnit,
};

pub type BodyPosition = Target<Position<Barycentric, Ecliptic, AstronomicalUnit>>;
type PositionFn = Box<dyn Fn(JulianDate) -> BodyPosition>;

pub struct Body {
    pub name: &'static str,
    pub color: &'static str,
    pub position_fn: PositionFn,
}

impl Body {
    pub fn new(
        name: &'static str,
        color: &'static str,
        position_fn: impl Fn(JulianDate) -> Target<Position<Barycentric, Ecliptic, AstronomicalUnit>>
            + 'static,
    ) -> Self {
        Self {
            name,
            color,
            position_fn: Box::new(position_fn),
        }
    }
}

fn moon_wrapper(jd: JulianDate) -> BodyPosition {
    let moon_helio = siderust::bodies::Moon::vsop87a(jd);
    let sun_bary = siderust::bodies::Sun::vsop87e(jd);

    let (x, y, z) = (
        moon_helio.position.x() + sun_bary.position.x(),
        moon_helio.position.y() + sun_bary.position.y(),
        moon_helio.position.z() + sun_bary.position.z(),
    );

    siderust::targets::Target {
        position: siderust::coordinates::cartesian::Position::new(x, y, z),
        time: jd,
        proper_motion: None,
    }
}

pub fn get_bodies() -> Vec<Body> {
    vec![
        Body::new("Sun", "#FFFFFF", Sun::vsop87e),
        Body::new("Mercury", "#DBBC7F", Mercury::vsop87e),
        Body::new("Venus", "#E69875", Venus::vsop87e),
        Body::new("Mars", "#E67E80", Mars::vsop87e),
        Body::new("Jupiter", "#D3C6AA", Jupiter::vsop87e),
        Body::new("Saturn", "#D699B6", Saturn::vsop87e),
        Body::new("Uranus", "#83C092", Uranus::vsop87e),
        Body::new("Neptune", "#7FBBB3", Neptune::vsop87e),
        Body::new("Moon", "#9DA9A0", moon_wrapper),
    ]
}
