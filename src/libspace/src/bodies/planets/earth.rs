use crate::bodies::body::Body;
use crate::bodies::keplerian_elements::KeplerianElements;
use crate::bodies::orbit::Orbit;

pub const EARTH_BODY: Body = Body {
    radius_mean: 6371.0,
    radius_equatorial: 6378.137,
    radius_polar: 6356.752,
    mass: 5.97237e24,
    angular_speed_per_second: 7.2921150e-5,
    sidereal_period: 365.256,
};

const EARTH_KEPLER_SHORT: KeplerianElements = KeplerianElements {
    semi_mayor_0: 1.00000261,
    semi_mayor_cy: 0.00000562,
    eccentricity_0: 0.01671123,
    eccentricity_cy: -0.00004392,
    inclination_0: -0.00001531,
    inclination_cy: -0.01294668,
    mean_longitude_0: 100.46457166,
    mean_longitude_cy: 35999.37244981,
    long_perihelion_0: 102.93768193,
    long_perihelion_cy: 0.32327364,
    long_ascending_0: 0.0,
    long_ascending_cy: 0.0,
    b: 0.0,
    c: 0.0,
    s: 0.0,
    f: 0.0,
};

const EARTH_KEPLER_LONG: KeplerianElements = KeplerianElements {
    semi_mayor_0: 1.00000018,
    semi_mayor_cy: -0.00000003,
    eccentricity_0: 0.01673163,
    eccentricity_cy: -0.00003661,
    inclination_0: -0.00054346,
    inclination_cy: -0.01337178,
    mean_longitude_0: 100.46691572,
    mean_longitude_cy: 35999.37306329,
    long_perihelion_0: 102.93005885,
    long_perihelion_cy: 0.31795260,
    long_ascending_0: -5.11260389,
    long_ascending_cy: -0.24123856,
    b: 0.0,
    c: 0.0,
    s: 0.0,
    f: 0.0,
};

pub const EARTH_ORBIT: Orbit = Orbit {
    elements_short: EARTH_KEPLER_SHORT,
    elements_long: EARTH_KEPLER_LONG,
};
