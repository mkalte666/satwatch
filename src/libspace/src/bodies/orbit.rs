use crate::bodies::keplerian_elements::KeplerianElements;
use crate::coordinate::IcrfStateVector;
use crate::timebase::Timebase;

pub struct Orbit {
    pub elements_short: KeplerianElements,
    pub elements_long: KeplerianElements,
}

impl Orbit {
    pub fn position_icrf(&self, timebase: &Timebase) -> IcrfStateVector {
        if timebase.now_jd_j2000().abs() > 365.0 * 200.0 {
            self.elements_long.position_icrf(timebase)
        } else {
            self.elements_short.position_icrf(timebase)
        }
    }

    pub fn position_icrf_since_j2000(&self, time: f64) -> IcrfStateVector {
        if time > 365.0 * 200.0 {
            self.elements_long.position_icrf_since_j2000(time)
        } else {
            self.elements_short.position_icrf_since_j2000(time)
        }
    }
}
