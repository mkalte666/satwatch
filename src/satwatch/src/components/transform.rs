use glam::f32::*;
use libspace::coordinates::{Coordinate, CoordinateSystem};

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

    pub fn from_coordinate(
        coordinate: &Coordinate,
        gl_origin: &Coordinate,
        world_scale: f64,
    ) -> Self {
        let mut result = Self::default();

        let a = coordinate.transform(CoordinateSystem::OpenGl);
        let b = gl_origin.transform(CoordinateSystem::OpenGl);
        result.translation = Vec3::new(
            ((a.position[0] - b.position[0]) / world_scale) as f32,
            ((a.position[1] - b.position[1]) / world_scale) as f32,
            ((a.position[2] - b.position[2]) / world_scale) as f32,
        );

        result.rotation = Quat::from_rotation_y(a.accumulated_rotations[1] as f32)
            * Quat::from_rotation_x(a.accumulated_rotations[0] as f32)
            * Quat::from_rotation_z(a.accumulated_rotations[2] as f32);
        result
    }
}
