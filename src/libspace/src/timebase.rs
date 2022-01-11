use chrono::{DateTime, TimeZone, Utc};
use std::ops::Add;
use std::time::*;

#[derive(Copy, Clone, Debug)]
pub struct Timebase {
    running: bool,
    realtime: bool,
    now: DateTime<Utc>,
    acceleration: f64,
}

impl Timebase {
    pub fn new() -> Self {
        Self {
            running: true,
            realtime: true,
            now: SystemTime::now().into(),
            acceleration: 1.0,
        }
    }

    pub fn j2000_epoch() -> DateTime<Utc> {
        let epoch: DateTime<Utc> = chrono::Utc.ymd(2000, 1, 1).and_hms(12, 0, 0);
        epoch
    }

    pub fn now(&self) -> DateTime<Utc> {
        self.now
    }

    pub fn now_julian_since_j2000(&self) -> f64 {
        self.duration_since(Self::j2000_epoch()).num_milliseconds() as f64
            / (1000.0 * 60.0 * 60.0 * 24.0)
    }

    pub fn now_julian(&self) -> f64 {
        self.now_julian_since_j2000() + 2451545.0
    }

    pub fn duration_since(&self, epoch: chrono::DateTime<Utc>) -> chrono::Duration {
        self.now.signed_duration_since(epoch)
    }

    pub fn duration_since_minutes(&self, epoch: chrono::DateTime<Utc>) -> f64 {
        self.duration_since(epoch).num_milliseconds() as f64 / 60000.0
    }

    pub fn now_since_j2000_minutes(&self) -> f64 {
        self.duration_since_minutes(Self::j2000_epoch())
    }

    pub fn tick(&mut self, interval: Duration) {
        if self.running {
            if self.realtime {
                self.now = SystemTime::now().into();
            } else {
                let secs = interval.as_secs_f64() * self.acceleration;
                if secs >= 0.0 {
                    self.now = self
                        .now
                        .add(chrono::Duration::milliseconds((secs * 1000.0) as i64));
                } else {
                    self.now = self
                        .now
                        .add(chrono::Duration::milliseconds((secs * 1000.0) as i64));
                }
            }
        }
    }

    pub fn set_now(&mut self, now: DateTime<Utc>) {
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
            self.now = SystemTime::now().into();
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
}
