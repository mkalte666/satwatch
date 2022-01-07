pub struct PosMeta {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CoordinateSystem {
    Invalid,
    OpenGl,
    EarthCenteredInertial,
    EarthCenteredEarthFixed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LatLong {
    latitude: f32,
    longitude: f32,
    altitude: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinate {
    pub time: f64,
    pub system: CoordinateSystem,
    pub position: [f64; 3],
    pub accumulated_rotations: [f64; 3],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StateVector {
    pub coordinate: Coordinate,
    pub velocity: [f64; 3],
}

impl Coordinate {
    pub fn invalid() -> Self {
        Self {
            time: 0.0,
            system: CoordinateSystem::Invalid,
            position: [0.0, 0.0, 0.0],
            accumulated_rotations: [0.0, 0.0, 0.0],
        }
    }
    pub fn new(system: CoordinateSystem, position: [f64; 3]) -> Self {
        Self {
            time: 0.0,
            system,
            position,
            accumulated_rotations: [0.0, 0.0, 0.0],
        }
    }

    pub fn new_timed(time: f64, system: CoordinateSystem, position: [f64; 3]) -> Self {
        Self {
            time,
            system,
            position,
            accumulated_rotations: [0.0, 0.0, 0.0],
        }
    }

    pub fn transform(&self, target_system: CoordinateSystem) -> Self {
        match self.system {
            CoordinateSystem::Invalid => Self::invalid(),
            CoordinateSystem::OpenGl => self.transform_from_gl(target_system),
            CoordinateSystem::EarthCenteredInertial => self.transform_from_eci(target_system),
            CoordinateSystem::EarthCenteredEarthFixed => self.transform_from_ecef(target_system),
        }
    }

    fn earth_angle_at(&self) -> f64 {
        let t = (self.time / (60.0 * 24.0));
        (2.0 * std::f64::consts::PI * (0.7790572732640 + 1.00273781191135448 * t))
            % (2.0 * std::f64::consts::PI)
    }

    fn transform_from_gl(&self, target_system: CoordinateSystem) -> Self {
        match target_system {
            CoordinateSystem::Invalid => Self::invalid(),
            CoordinateSystem::OpenGl => self.clone(),
            CoordinateSystem::EarthCenteredInertial => Self {
                time: self.time,
                // in eci x is gl z, y is gl x and z is gl y
                system: target_system,
                position: [self.position[2], self.position[0], self.position[1]],
                accumulated_rotations: [
                    self.accumulated_rotations[2],
                    self.accumulated_rotations[0],
                    self.accumulated_rotations[1],
                ],
            },
            CoordinateSystem::EarthCenteredEarthFixed => {
                let in_eci = self.transform(CoordinateSystem::EarthCenteredInertial);
                in_eci.transform_from_eci(target_system)
            }
        }
    }

    fn transform_from_eci(&self, target_system: CoordinateSystem) -> Self {
        match target_system {
            CoordinateSystem::Invalid => Self::invalid(),
            CoordinateSystem::OpenGl => Self {
                time: self.time,
                // in gl, x is eci y, y is z and z is x
                system: target_system,
                position: [self.position[1], self.position[2], self.position[0]],
                accumulated_rotations: [
                    self.accumulated_rotations[1],
                    self.accumulated_rotations[2],
                    self.accumulated_rotations[0],
                ],
            },
            CoordinateSystem::EarthCenteredInertial => self.clone(),
            CoordinateSystem::EarthCenteredEarthFixed => {
                let a = -self.earth_angle_at();
                let mut new = self.clone();
                new.system = CoordinateSystem::EarthCenteredEarthFixed;
                new.accumulated_rotations[2] += a;
                new.position[0] = self.position[0] * a.cos() - self.position[1] * a.sin();
                new.position[1] = self.position[1] * a.cos() + self.position[0] * a.sin();
                new
            }
        }
    }

    fn transform_from_ecef(&self, target_system: CoordinateSystem) -> Self {
        let mut as_eci = self.clone();
        as_eci.system = CoordinateSystem::EarthCenteredInertial;
        let a = self.earth_angle_at();
        as_eci.accumulated_rotations[2] += a;
        as_eci.position[0] = self.position[0] * a.cos() - self.position[1] * a.sin();
        as_eci.position[1] = self.position[1] * a.cos() + self.position[0] * a.sin();
        as_eci.transform_from_eci(target_system)
    }
}

use crate::planets::earth::Earth;
use sgp4::Prediction;
use std::fmt::{Display, Formatter};

impl Display for CoordinateSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CoordinateSystem::Invalid => write!(f, "Invalid"),
            CoordinateSystem::OpenGl => write!(f, "OpenGL"),
            CoordinateSystem::EarthCenteredInertial => write!(f, "Earth-Centered Inertial"),
            CoordinateSystem::EarthCenteredEarthFixed => write!(f, "Earth-Centered Earth-Fixed"),
        }
    }
}

impl Default for StateVector {
    fn default() -> Self {
        Self {
            coordinate: Coordinate::invalid(),
            velocity: [0.0, 0.0, 0.0],
        }
    }
}

impl From<sgp4::Prediction> for StateVector {
    fn from(p: Prediction) -> Self {
        Self {
            coordinate: Coordinate {
                time: 0.0,
                system: CoordinateSystem::EarthCenteredInertial,
                position: p.position,
                accumulated_rotations: [0.0, 0.0, 0.0],
            },
            velocity: p.velocity,
        }
    }
}
