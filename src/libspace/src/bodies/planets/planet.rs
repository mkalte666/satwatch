use crate::bodies::body::Body;
use crate::bodies::orbit::Orbit;
use crate::bodies::planets::planet_bodies::*;
use crate::coordinate::{CoordinateUnit, IcrfStateVector};
use crate::timebase::Timebase;
use glam::Quat;
use std::fmt::{Display, Formatter};

use crate::bodies::keplerian_elements::KeplerianElements;
include!(concat!(env!("OUT_DIR"), "/kepler_short.rs"));
include!(concat!(env!("OUT_DIR"), "/kepler_long.rs"));
include!(concat!(env!("OUT_DIR"), "/kepler_orbits.rs"));

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Planet {
    Sun,
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

impl Planet {
    pub fn body(&self) -> &Body {
        match self {
            Planet::Sun => &SUN_BODY,
            Planet::Mercury => &MERCURY_BODY,
            Planet::Venus => &VENUS_BODY,
            Planet::Earth => &EARTH_BODY,
            Planet::Mars => &MARS_BODY,
            Planet::Jupiter => &JUPITER_BODY,
            Planet::Saturn => &SATURN_BODY,
            Planet::Uranus => &URANUS_BODY,
            Planet::Neptune => &NEPTUNE_BODY,
        }
    }

    pub fn pos_icrf(&self, timebase: &Timebase) -> IcrfStateVector {
        // specal case sun
        if *self == Planet::Sun {
            IcrfStateVector {
                unit: CoordinateUnit::Au,
                position: Default::default(),
                velocity: Default::default(),
            }
        } else {
            self.orbit().position_icrf(timebase)
        }
    }

    pub fn rough_pos_list(&self, timebase: &Timebase) -> Vec<IcrfStateVector> {
        if *self == Planet::Sun {
            Vec::new()
        } else {
            let mut results = Vec::new();
            for i in 0..((self.body().sidereal_period.ceil() + 1.0) as usize) {
                let t = timebase.now_jd_j2000() + i as f64;
                results.push(self.orbit().position_icrf_since_j2000(t));
            }

            results
        }
    }

    pub fn orbit(&self) -> &Orbit {
        match self {
            Planet::Sun => {
                panic!("Sun has no orbit");
            }
            Planet::Mercury => &MERCURY_ORBIT,
            Planet::Venus => &VENUS_ORBIT,
            Planet::Earth => &EARTH_ORBIT,
            Planet::Mars => &MARS_ORBIT,
            Planet::Jupiter => &JUPITER_ORBIT,
            Planet::Saturn => &SATURN_ORBIT,
            Planet::Uranus => &URANUS_ORBIT,
            Planet::Neptune => &NEPTUNE_ORBIT,
        }
    }

    pub fn angle_at(&self, time: &Timebase) -> f64 {
        match self {
            Planet::Sun => 0.0,
            Planet::Mercury => 0.0,
            Planet::Venus => 0.0,
            Planet::Earth => {
                let t = time.now_jd_j2000();
                (2.0 * std::f64::consts::PI * (0.7790572732640 + 1.00273781191135448 * t))
                    % (2.0 * std::f64::consts::PI)
            }
            Planet::Mars => 0.0,
            Planet::Jupiter => 0.0,
            Planet::Saturn => 0.0,
            Planet::Uranus => 0.0,
            Planet::Neptune => 0.0,
        }
    }

    pub fn gl_rotation_at(&self, timebase: &Timebase) -> Quat {
        match self {
            Planet::Sun => Quat::default(),
            Planet::Mercury => Quat::default(),
            Planet::Venus => Quat::default(),
            Planet::Earth => Quat::from_rotation_y(self.angle_at(timebase) as f32),
            Planet::Mars => Quat::default(),
            Planet::Jupiter => Quat::default(),
            Planet::Saturn => Quat::default(),
            Planet::Uranus => Quat::default(),
            Planet::Neptune => Quat::default(),
        }
    }
}

impl Display for Planet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Planet::Sun => write!(f, "Sun"),
            Planet::Mercury => write!(f, "Mercury"),
            Planet::Venus => write!(f, "Venus"),
            Planet::Earth => write!(f, "Earth"),
            Planet::Mars => write!(f, "Mars"),
            Planet::Jupiter => write!(f, "Jupiter"),
            Planet::Saturn => write!(f, "Saturn"),
            Planet::Uranus => write!(f, "Uranus"),
            Planet::Neptune => write!(f, "Neptune"),
        }
    }
}
