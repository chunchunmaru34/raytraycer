use crate::geometry::vec3::{Vec3};

pub struct Light {
    pub position: Vec3,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vec3, intensity: f32) -> Self {
        Self { position, intensity }
    }
}