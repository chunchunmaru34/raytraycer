use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
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
    use crate::geometry::material::Material;
    use crate::geometry::ray::Ray;
    use crate::geometry::sphere::Sphere;
    use crate::geometry::vec3::Vec3;
    use crate::utils::rgb::RGB;

    #[test]
    fn ray_intersect() {
        let orig = Vec3::new(0., 0., 0.);
        let dir = Vec3::new(4., 0., 0.).normalize();
        let mut ray = Ray::new(orig, dir, std::f32::MAX);

        let material = Material {
            color: RGB::new(24, 24, 24),
            specular_exponent: 1.,
            albedo: (1., 1., 1., 1.),
            refractive_index: 0.,
        };

        let sphere = Sphere::new(Vec3::new(4., 1., 0.), 2.5, material);

        assert!(sphere.ray_intersect(&mut ray));
    }
}
