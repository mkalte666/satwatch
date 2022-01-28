// straight forwawrd implementation of https://ssd.jpl.nasa.gov/planets/approx_pos.html
// Archive version: https://web.archive.org/web/20211128162928/https://ssd.jpl.nasa.gov/planets/approx_pos.html

use crate::coordinate::*;
use crate::timebase::Timebase;

pub struct KeplerianElements {
    /// au
    pub semi_mayor_0: f64,
    /// au/century
    pub semi_mayor_cy: f64,
    ///  rad
    pub eccentricity_0: f64,
    /// rad/century
    pub eccentricity_cy: f64,
    /// degree
    pub inclination_0: f64,
    /// degree/century
    pub inclination_cy: f64,
    /// degree
    pub mean_longitude_0: f64,
    /// degree/century
    pub mean_longitude_cy: f64,
    /// degree
    pub long_perihelion_0: f64,
    /// degree/century
    pub long_perihelion_cy: f64,
    /// degree
    pub long_ascending_0: f64,
    /// degree/century
    pub long_ascending_cy: f64,
    /// correction term b
    pub b: f64,
    /// correction term c
    pub c: f64,
    /// correction term s
    pub s: f64,
    /// correction term f
    pub f: f64,
}

const KEPLER_TOLERANCE: f64 = 1e-6;
const KEPLER_MAX_STEPS: usize = 1024;

impl KeplerianElements {
    pub fn position_ecliptic_since_j2000(&self, time: f64) -> [f64; 3] {
        let t = time / 36525.0;

        // time dependent parameters
        let semi_mayor = self.semi_mayor_0 + self.semi_mayor_cy * t;
        let eccentricity = self.eccentricity_0 + self.eccentricity_cy * t;
        let inclination = (self.inclination_0 + self.inclination_cy * t).to_radians();
        let mean_longitude = (self.mean_longitude_0 + self.mean_longitude_cy * t).to_radians();
        let long_perihelion = (self.long_perihelion_0 + self.long_perihelion_cy * t).to_radians();
        let long_ascending = (self.long_ascending_0 + self.long_ascending_cy * t).to_radians();

        let argument_perihelion = long_perihelion - long_ascending;
        let mean_anomaly_no_modulo = mean_longitude - long_perihelion
            + self.b * t * t
            + self.c * ((self.f * t).to_radians()).cos()
            + self.s * ((self.f * t).to_radians()).sin();
        let mean_anomaly = ((mean_anomaly_no_modulo + std::f64::consts::PI)
            % (2.0 * std::f64::consts::PI))
            - std::f64::consts::PI;
        let eccentric_anomaly = self.solve_keplers_equation(mean_anomaly);

        let x_hel = semi_mayor * (eccentric_anomaly.cos() - eccentricity);
        let y_hel =
            semi_mayor * (1.0 - eccentricity * eccentricity).sqrt() * eccentric_anomaly.sin();

        let x_ecl = (argument_perihelion.cos() * long_ascending.cos()
            - argument_perihelion.sin() * long_ascending.sin() * inclination.cos())
            * x_hel
            + (-argument_perihelion.sin() * long_ascending.cos()
                - argument_perihelion.cos() * long_ascending.sin() * inclination.cos())
                * y_hel;
        let y_ecl = (argument_perihelion.cos() * long_ascending.sin()
            + argument_perihelion.sin() * long_ascending.cos() * inclination.cos())
            * x_hel
            + (-argument_perihelion.sin() * long_ascending.sin()
                + argument_perihelion.cos() * long_ascending.cos() * inclination.cos())
                * y_hel;
        let z_ecl = argument_perihelion.sin() * inclination.sin() * x_hel
            + argument_perihelion.cos() * inclination.sin() * y_hel;
        [x_ecl, y_ecl, z_ecl]
    }

    pub fn position_ecliptic(&self, timebase: &Timebase) -> [f64; 3] {
        let time = timebase.now_jd_j2000();
        self.position_ecliptic_since_j2000(time)
    }

    pub fn position_icrf_since_j2000(&self, time: f64) -> IcrfStateVector {
        let epsilon = 23.43928f64.to_radians();
        let cos_e = epsilon.cos();
        let sin_e = epsilon.sin();

        let [x_ecl, y_ecl, z_ecl] = self.position_ecliptic_since_j2000(time);
        let x_eq = x_ecl;
        let y_eq = cos_e * y_ecl - sin_e * z_ecl;
        let z_eq = sin_e * y_ecl + cos_e * z_ecl;
        IcrfStateVector {
            unit: CoordinateUnit::Au,
            position: DVec3::new(x_eq, y_eq, z_eq),
            velocity: DVec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn position_icrf(&self, timebase: &Timebase) -> IcrfStateVector {
        let time = timebase.now_jd_j2000();
        self.position_icrf_since_j2000(time)
    }

    fn solve_keplers_equation(&self, mean_anomaly: f64) -> f64 {
        let mut eccentric_anomaly = mean_anomaly - self.eccentricity_0 * mean_anomaly.sin();
        let mut delta_ea = 0.0;
        for _i in 0..KEPLER_MAX_STEPS {
            let delta_m =
                mean_anomaly - (eccentric_anomaly - self.eccentricity_0 * eccentric_anomaly.sin());
            delta_ea = delta_m / (1.0 - self.eccentricity_0 * eccentric_anomaly.cos());
            eccentric_anomaly = eccentric_anomaly + delta_ea;
            if delta_ea <= KEPLER_TOLERANCE {
                return eccentric_anomaly;
            }
        }
        log::warn!("Kepler Solver could find a solution for the eccentric anomaly. Ended with {}, which still has a delta-E of {}", eccentric_anomaly, delta_ea);
        eccentric_anomaly
    }
}
