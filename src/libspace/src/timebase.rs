use chrono::{DateTime, NaiveDateTime, Utc};
use spice::c::ConstSpiceChar;
use spice::SpiceLock;
use std::ffi::{CStr, CString};

use std::os::raw::c_char;
use std::path::PathBuf;

use std::time::*;

#[derive(Copy, Clone, Debug)]
pub struct Timebase {
    running: bool,
    realtime: bool,
    now: f64,
    acceleration: f64,
}

pub const SPICE_LSK_URL: &'static str =
    "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/lsk/latest_leapseconds.tls";
pub const SPICE_LSK_FILENAME: &'static str = "latest_leapseconds.tls";

pub fn date_time_to_spice_str(t: DateTime<Utc>) -> String {
    t.format("%F %T.%f").to_string()
}

fn parse_spice_utc(s: &str) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(NaiveDateTime::parse_from_str(s, "%FT%T.%f").unwrap(), Utc)
}

pub fn date_time_to_et(t: DateTime<Utc>) -> f64 {
    let sl = SpiceLock::acquire().expect("mutex poisoned. im sad now");
    sl.str2et(&date_time_to_spice_str(t))
}

pub fn now_utc_str() -> String {
    let now: DateTime<Utc> = SystemTime::now().into();
    date_time_to_spice_str(now)
}

pub fn now_et() -> f64 {
    let lock = SpiceLock::acquire().unwrap();
    lock.str2et(&now_utc_str())
}

impl Timebase {
    pub fn new() -> Self {
        let lock = SpiceLock::acquire().unwrap();
        Self {
            running: true,
            realtime: true,
            now: lock.str2et(&now_utc_str()),
            acceleration: 1.0,
        }
    }

    pub fn now(&self) -> f64 {
        self.now
    }

    pub fn now_utc(&self) -> DateTime<Utc> {
        let iso_date = unsafe {
            let lock = SpiceLock::acquire().unwrap();
            let format_c = CString::new("ISOC").unwrap().into_raw();
            let mut dst_bytes = Vec::<c_char>::with_capacity(128);
            spice::c::et2utc_c(
                self.now as spice::c::SpiceDouble,
                format_c as *mut ConstSpiceChar,
                5,
                128,
                dst_bytes.as_mut_ptr() as *mut spice::c::SpiceChar,
            );
            drop(CString::from_raw(format_c));
            let iso_str_c = CStr::from_ptr(dst_bytes.as_ptr());
            iso_str_c.to_str().unwrap().to_owned()
        };

        parse_spice_utc(&iso_date)
    }

    pub fn now_jd_j2000(&self) -> f64 {
        self.now / 86400.0
    }

    pub fn now_jd(&self) -> f64 {
        self.now_jd_j2000() + 2451545.0
    }

    pub fn seconds_since(&self, epoch: f64) -> f64 {
        epoch - self.now
    }

    pub fn minutes_since(&self, epoch: f64) -> f64 {
        self.seconds_since(epoch) / 60.0
    }

    pub fn seconds_since_dt(&self, epoch: chrono::DateTime<Utc>) -> f64 {
        let then = date_time_to_et(epoch);
        then - self.now
    }

    pub fn minutes_since_dt(&self, epoch: chrono::DateTime<Utc>) -> f64 {
        self.seconds_since_dt(epoch) / 60.0
    }

    pub fn minutes_since_j2000(&self) -> f64 {
        self.now / 60.0
    }

    pub fn tick(&mut self, interval: f64) {
        if self.running {
            if self.realtime {
                self.now = now_et();
            } else {
                let secs = interval * self.acceleration;
                self.now += secs;
            }
        }
    }

    pub fn set_now(&mut self, now: f64) {
        self.now = now;
    }

    pub fn set_realtime(&mut self, realtime: bool) {
        self.realtime = realtime;
    }

    pub fn realtime(&self) -> bool {
        self.realtime
    }

    pub fn set_running(&mut self, running: bool) {
        self.running = running;

        if running && self.realtime {
            self.now = now_et();
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn set_acceleration(&mut self, acceleration: f64) {
        self.acceleration = acceleration;
    }

    pub fn acceleration(&self) -> f64 {
        self.acceleration
    }

    pub fn load_lsk() {
        let lock = SpiceLock::acquire().unwrap();
        lock.furnsh(Self::lsk_file().to_str().unwrap());
    }

    pub fn lsk_file() -> PathBuf {
        let filename = crate::utility::init_dirs()
            .expect("Cannot read from home?")
            .join(SPICE_LSK_FILENAME);
        filename
    }
}
