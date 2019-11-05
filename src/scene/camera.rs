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

#[cfg(test)]
mod tests {
    use crate::scene::camera::Camera;
    use crate::geometry::vec3::Vec3;

    #[test]
    fn move_by() {
        let mut camera = Camera::new(Vec3::new(0., 0., 0.));

        camera.move_by(&Vec3::new(1., 2., 3.));
        camera.move_by(&Vec3::new(3., 2., 2.));

        assert_eq!(camera.position.x, 4.);
        assert_eq!(camera.position.y, 4.);
        assert_eq!(camera.position.z, 5.);
    }
}