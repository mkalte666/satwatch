use glam::f32::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec4,
    pub ambient: f32,
}
