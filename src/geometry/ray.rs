use super::vec3::{Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub t: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, t: f32) -> Self {
        Self { origin, direction, t }
    }
}