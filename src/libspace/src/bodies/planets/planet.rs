use crate::bodies::body::{Body, Orbit};
use crate::bodies::planets::earth;
use crate::timebase::Timebase;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum Planet {
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
            Planet::Mercury => {
                todo!()
            }
            Planet::Venus => {
                todo!()
            }
            Planet::Earth => &earth::EARTH_BODY,
            Planet::Mars => {
                todo!()
            }
            Planet::Jupiter => {
                todo!()
            }
            Planet::Saturn => {
                todo!()
            }
            Planet::Uranus => {
                todo!()
            }
            Planet::Neptune => {
                todo!()
            }
        }
    }

    pub fn orbit(&self) -> &Orbit {
        match self {
            Planet::Mercury => {
                todo!()
            }
            Planet::Venus => {
                todo!()
            }
            Planet::Earth => &earth::EARTH_ORBIT,
            Planet::Mars => {
                todo!()
            }
            Planet::Jupiter => {
                todo!()
            }
            Planet::Saturn => {
                todo!()
            }
            Planet::Uranus => {
                todo!()
            }
            Planet::Neptune => {
                todo!()
            }
        }
    }

    pub fn angle_at(&self, time: &Timebase) -> f64 {
        match self {
            Planet::Mercury => {
                todo!()
            }
            Planet::Venus => {
                todo!()
            }
            Planet::Earth => {
                let t = time.now_julian_since_j2000();
                (2.0 * std::f64::consts::PI * (0.7790572732640 + 1.00273781191135448 * t))
                    % (2.0 * std::f64::consts::PI)
            }
            Planet::Mars => {
                todo!()
            }
            Planet::Jupiter => {
                todo!()
            }
            Planet::Saturn => {
                todo!()
            }
            Planet::Uranus => {
                todo!()
            }
            Planet::Neptune => {
                todo!()
            }
        }
    }
}

impl Display for Planet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
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