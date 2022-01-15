use crate::bodies::Planet;
use crate::coordinate::coordinate_unit::CoordinateUnit;
use crate::coordinate::IcrfStateVector;

use crate::timebase::Timebase;
use glam::f64::DVec3;
use sgp4::Prediction;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum PlanetaryReferenceFrame {
    Inertial,
    BodyFixed,
}

#[derive(Copy, Clone, Debug)]
pub struct PlanetaryStateVector {
    pub planet: Planet,
    pub reference_frame: PlanetaryReferenceFrame,
    pub unit: CoordinateUnit,
    pub position: DVec3,
    pub velocity: DVec3,
}

impl PlanetaryStateVector {
    pub fn as_unit(&self, new_unit: CoordinateUnit) -> Self {
        Self {
            planet: self.planet,
            reference_frame: self.reference_frame,
            unit: new_unit,
            position: self.unit.to(new_unit, &self.position),
            velocity: self.unit.to(new_unit, &self.velocity),
        }
    }

    pub fn transform_reference(&self, new_ref: PlanetaryReferenceFrame, time: &Timebase) -> Self {
        match self.reference_frame {
            PlanetaryReferenceFrame::Inertial => match new_ref {
                PlanetaryReferenceFrame::Inertial => self.clone(),
                PlanetaryReferenceFrame::BodyFixed => {
                    let a = -self.planet.angle_at(time);
                    let mut new = self.clone();
                    new.reference_frame = new_ref;
                    new.position.x = self.position.x * a.cos() - self.position.y * a.sin();
                    new.position.y = self.position.y * a.cos() + self.position.x * a.sin();
                    new
                }
            },
            PlanetaryReferenceFrame::BodyFixed => match new_ref {
                PlanetaryReferenceFrame::Inertial => {
                    let a = self.planet.angle_at(time);
                    let mut new = self.clone();
                    new.reference_frame = new_ref;
                    new.position.x = self.position.x * a.cos() - self.position.y * a.sin();
                    new.position.y = self.position.y * a.cos() + self.position.x * a.sin();
                    new
                }
                PlanetaryReferenceFrame::BodyFixed => self.clone(),
            },
        }
    }

    pub fn to_icrf(&self, time: &Timebase) -> IcrfStateVector {
        let as_inertial = self.transform_reference(PlanetaryReferenceFrame::Inertial, time);

        if self.planet == Planet::Sun {
            IcrfStateVector {
                unit: self.unit,
                position: as_inertial.position,
                velocity: as_inertial.velocity,
            }
        } else {
            let planet_icrf = self.planet.pos_icrf(time);
            IcrfStateVector {
                unit: self.unit,
                position: as_inertial.position
                    + planet_icrf.unit.to(self.unit, &planet_icrf.position),
                velocity: as_inertial.velocity
                    + planet_icrf.unit.to(self.unit, &planet_icrf.velocity),
            }
        }
    }

    pub fn from_icrf(
        icrf: IcrfStateVector,
        time: &Timebase,
        planet: Planet,
    ) -> PlanetaryStateVector {
        let planet_pos = planet.pos_icrf(time);
        match planet {
            Planet::Sun => PlanetaryStateVector {
                planet,
                reference_frame: PlanetaryReferenceFrame::Inertial,
                unit: icrf.unit,
                position: icrf.position,
                velocity: icrf.velocity,
            },
            _ => PlanetaryStateVector {
                planet,
                reference_frame: PlanetaryReferenceFrame::Inertial,
                unit: icrf.unit,
                position: icrf.position - planet_pos.unit.to(icrf.unit, &planet_pos.position),
                velocity: icrf.velocity - planet_pos.unit.to(icrf.unit, &planet_pos.velocity),
            },
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

impl Display for PlanetaryStateVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[x,y,z][vx,vy,vz]@{}-Centered {}: [{:+e},{:+e},{:+e}][{:+e},{:+e},{:+e}]",
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

impl From<Prediction> for PlanetaryStateVector {
    fn from(prediction: Prediction) -> Self {
        Self {
            planet: Planet::Earth,
            reference_frame: PlanetaryReferenceFrame::Inertial,
            unit: CoordinateUnit::KiloMeter,
            position: DVec3::from(prediction.position),
            velocity: DVec3::from(prediction.velocity),
        }
    }
}
