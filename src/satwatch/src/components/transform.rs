use glam::f32::*;
use libspace::coordinate::{CoordinateUnit, IcrfStateVector, PlanetaryStateVector};
use libspace::timebase::Timebase;

#[derive(Debug, Copy, Clone)]
pub struct WorldTransform {
    pub translation: Vec3,
    pub scale: Vec3,
    pub rotation: Quat,
}

impl Default for WorldTransform {
    fn default() -> Self {
        Self {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation: Quat::from_rotation_x(0.0),
        }
    }
}

impl WorldTransform {
    pub fn get_model_matrix(&self) -> Mat4 {
        return Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation);
    }

    pub fn from_icrf(
        coordinate: &IcrfStateVector,
        gl_origin: &IcrfStateVector,
        world_scale: f64,
        scale_unit: CoordinateUnit,
        old: Option<Self>,
    ) -> Self {
        let mut result = if old.is_some() {
            old.unwrap()
        } else {
            Self::default()
        };

        let p = coordinate.to_gl_coord(world_scale, scale_unit, gl_origin);

        result.translation = Vec3::new(p.x as f32, p.y as f32, p.z as f32);
        result
    }

    pub fn from_planet_vec(
        coordinate: &PlanetaryStateVector,
        gl_origin: &IcrfStateVector,
        world_scale: f64,
        scale_unit: CoordinateUnit,
        time: &Timebase,
        old: Option<Self>,
    ) -> Self {
        let icrf_pos = coordinate.to_icrf(time);
        Self::from_icrf(&icrf_pos, gl_origin, world_scale, scale_unit, old)
    }
}
