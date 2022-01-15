use crate::bodies::body::Body;
use crate::bodies::keplerian_elements::KeplerianElements;
use crate::bodies::orbit::Orbit;

pub const MERCURY_BODY: Body = Body {
    radius_mean: 2439.7,
    radius_equatorial: 2439.7,
    radius_polar: 2439.7,
    mass: 3.3011e23,
    angular_speed_per_second: 0.0,
    sidereal_period: 87.9691,
};

const MERCURY_KEPLER_SHORT: KeplerianElements = KeplerianElements {
    semi_mayor_0: 0.38709927,
    semi_mayor_cy: 0.00000037,
    eccentricity_0: 0.20563593,
    eccentricity_cy: 0.00001906,
    inclination_0: 7.00497902,
    inclination_cy: -0.00594749,
    mean_longitude_0: 252.25032350,
    mean_longitude_cy: 149472.67411175,
    long_perihelion_0: 77.45779628,
    long_perihelion_cy: 0.16047689,
    long_ascending_0: 48.33076593,
    long_ascending_cy: -0.12534081,
    b: 0.0,
    c: 0.0,
    s: 0.0,
    f: 0.0,
};

const MERCURY_KEPLER_LONG: KeplerianElements = KeplerianElements {
    semi_mayor_0: 0.38709843,
    semi_mayor_cy: 0.00000000,
    eccentricity_0: 0.20563661,
    eccentricity_cy: -0.00002123,
    inclination_0: -7.00559432,
    inclination_cy: -0.00590158,
    mean_longitude_0: 252.25166724,
    mean_longitude_cy: 149472.67486623,
    long_perihelion_0: 77.45771895,
    long_perihelion_cy: 0.15940013,
    long_ascending_0: 48.33961819,
    long_ascending_cy: -0.12214182,
    b: 0.0,
    c: 0.0,
    s: 0.0,
    f: 0.0,
};

pub const MERCURY_ORBIT: Orbit = Orbit {
    elements_short: MERCURY_KEPLER_SHORT,
    elements_long: MERCURY_KEPLER_LONG,
};
