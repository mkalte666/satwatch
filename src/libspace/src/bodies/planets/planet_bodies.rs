use crate::bodies::body::Body;

pub const SUN_BODY: Body = Body {
    radius_mean: 696342.0,
    radius_equatorial: 696342.0,
    radius_polar: 696342.0,
    mass: 1.9885e30,
    angular_speed_per_second: 0.0,
    sidereal_period: 0.0,
};

pub const MERCURY_BODY: Body = Body {
    radius_mean: 2439.7,
    radius_equatorial: 2439.7,
    radius_polar: 2439.7,
    mass: 3.3011e23,
    angular_speed_per_second: 0.0,
    sidereal_period: 87.9691,
};

pub const VENUS_BODY: Body = Body {
    radius_mean: 6051.8,
    radius_equatorial: 6051.8,
    radius_polar: 6051.8,
    mass: 4.8675e24,
    angular_speed_per_second: 0.0,
    sidereal_period: 243.0226,
};

pub const EARTH_BODY: Body = Body {
    radius_mean: 6371.0,
    radius_equatorial: 6378.137,
    radius_polar: 6356.752,
    mass: 5.97237e24,
    angular_speed_per_second: 7.2921150e-5,
    sidereal_period: 365.256,
};

pub const MARS_BODY: Body = Body {
    radius_mean: 3389.5,
    radius_equatorial: 3396.2,
    radius_polar: 3376.2,
    mass: 6.4171e23,
    angular_speed_per_second: 0.0,
    sidereal_period: 779.96,
};

pub const JUPITER_BODY: Body = Body {
    radius_mean: 69911.0,
    radius_equatorial: 71492.0,
    radius_polar: 66854.0,
    mass: 1.8982e27,
    angular_speed_per_second: 0.0,
    sidereal_period: 4332.59,
};

pub const SATURN_BODY: Body = Body {
    radius_mean: 58232.0,
    radius_equatorial: 60268.0,
    radius_polar: 54364.0,
    mass: 5.6834e26,
    angular_speed_per_second: 0.0,
    sidereal_period: 10759.22,
};

pub const URANUS_BODY: Body = Body {
    radius_mean: 25362.0,
    radius_equatorial: 25559.0,
    radius_polar: 24973.0,
    mass: 8.6810e25,
    angular_speed_per_second: 0.0,
    sidereal_period: 30688.5,
};

pub const NEPTUNE_BODY: Body = Body {
    radius_mean: 24622.0,
    radius_equatorial: 24764.0,
    radius_polar: 24341.0,
    mass: 1.02413e26,
    angular_speed_per_second: 0.0,
    sidereal_period: 60195.0,
};
