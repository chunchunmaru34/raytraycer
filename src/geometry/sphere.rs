use super::vec3::{Vec3};
use super::ray::{Ray};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: (u8, u8, u8),
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, color: (u8, u8, u8)) -> Self {
        Self { center, radius, color }
    }

    pub fn ray_intersect(&self, ray: &mut Ray) -> bool {
        let orig = &ray.origin;
        let dir = &ray.direction;
        let c = self.center.minus(orig);

        let tca = c.dot_product(dir);
        if tca < 0. {
            return false;
        }

        let projection = c.dot_product(&c) - tca * tca;

        if projection > self.radius * self.radius {
            return false;
        }

        let thc = (self.radius * self.radius - projection).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        
        if t0 < 0. {
            t0 = t1;
        }

        if t0 < 0. {
            return false;
        }

        ray.t = t0;

        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::sphere::{Sphere};
    use crate::geometry::vec3::{Vec3};
    use crate::geometry::ray::{Ray};

    #[test]
    fn ray_intersect() {
        let orig = Vec3::new(0., 0., 0.);
        let dir = Vec3::new(4., 0., 0.).normalize();
        let mut ray = Ray::new(orig, dir, std::f32::MAX);

        let sphere = Sphere::new(Vec3::new(4., 1., 0.), 2.5, (24, 24, 24));

        assert!(sphere.ray_intersect(&mut ray));
    }
}