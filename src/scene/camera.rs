use crate::geometry::vec3::{Vec3};

pub struct Camera {
    pub position: Vec3,
}

impl Camera {
    pub fn new(position: Vec3) -> Self {
        Self {
            position
        }
    }

    pub fn move_by(&mut self, distance: &Vec3) {
        self.position = self.position.plus(distance);
    }
}