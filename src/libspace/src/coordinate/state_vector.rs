use crate::bodies::Planet;
use crate::coordinate::coordinate_unit::CoordinateUnit;

use glam::f64::DVec3;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum PlanetaryReferenceFrame {
    Inertial,
    BodyFixed,
}

#[derive(Copy, Clone, Debug)]
pub struct PlanetStateVector {
    pub planet: Planet,
    pub reference_frame: PlanetaryReferenceFrame,
    pub unit: CoordinateUnit,
    pub position: DVec3,
    pub velocity: DVec3,
}

impl PlanetStateVector {
    pub fn as_unit(&self, new_unit: CoordinateUnit) -> Self {
        Self {
            planet: self.planet,
            reference_frame: self.reference_frame,
            unit: new_unit,
            position: self.unit.to(new_unit, &self.position),
            velocity: self.unit.to(new_unit, &self.velocity),
        }
    }
}

impl Display for PlanetaryReferenceFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanetaryReferenceFrame::Inertial => write!(f, "Inertial"),
            PlanetaryReferenceFrame::BodyFixed => write!(f, "Body-Fixed"),
        }
    }
}

impl Display for PlanetStateVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[x,y,z][vx,vy,vz]@{}-Centered {}: [{},{},{}][{},{},{}",
            self.planet,
            self.reference_frame,
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z
        )
    }
}
