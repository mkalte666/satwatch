use crate::bodies::keplerian_elements::KeplerianElements;

pub struct Body {
    pub radius_mean: f64,
    pub radius_equatorial: f64,
    pub radius_polar: f64,
    pub mass: f64,
    pub angular_speed_per_second: f64,
    pub elements_short: KeplerianElements,
    pub elements_long: KeplerianElements,
}
