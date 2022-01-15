use crate::coordinate::coordinate_unit::CoordinateUnit;
use glam::f64::DVec3;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub struct IcrfStateVector {
    pub unit: CoordinateUnit,
    pub position: DVec3,
    pub velocity: DVec3,
}

impl IcrfStateVector {
    pub fn as_unit(&self, new_unit: CoordinateUnit) -> Self {
        Self {
            unit: new_unit,
            position: self.unit.to(new_unit, &self.position),
            velocity: self.unit.to(new_unit, &self.velocity),
        }
    }

    pub fn to_gl_coord(&self, scale: f64, scale_unit: CoordinateUnit, gl_origin: &Self) -> DVec3 {
        let moved = self.position - gl_origin.unit.to(self.unit, &gl_origin.position);
        let scaled = self.unit.to(scale_unit, &moved) / scale;
        let swapped = DVec3::new(scaled.y, scaled.z, scaled.x);
        swapped
    }

    pub fn from_gl_coord(
        coord: &DVec3,
        scale: f64,
        scale_unit: CoordinateUnit,
        gl_origin: &Self,
    ) -> Self {
        let swapped = DVec3::new(coord.z, coord.x, coord.y);
        let scaled = swapped * scale;
        Self {
            unit: scale_unit,
            position: scaled + gl_origin.unit.to(scale_unit, &gl_origin.position),
            velocity: DVec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Display for IcrfStateVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[x,y,z][vx,vy,vz] ICRF: [{:+e},{:+e},{:+e}][{:+e},{:+e},{:+e}]",
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z
        )
    }
}
