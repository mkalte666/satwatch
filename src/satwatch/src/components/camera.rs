use crate::components::WorldTransform;
use glam::f32::*;
use glam::EulerRot;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(fov: f32, near: f32, far: f32) -> Self {
        Self { fov, near, far }
    }

    pub fn get_view_projection(&self, aspect: f32, transform: &WorldTransform) -> Mat4 {
        let projection = Mat4::perspective_rh_gl(
            self.fov / 180.0 * std::f32::consts::PI,
            aspect,
            self.near,
            self.far,
        );
        let rot_trans = Mat4::from_rotation_translation(transform.rotation, transform.translation);
        let view = rot_trans.inverse();
        return projection * view;
    }
}
