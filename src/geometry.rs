pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

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

        let projection_sqr = c.dot_product(&c) - tca * tca;

        if projection_sqr > self.radius * self.radius {
            return false;
        }

        let thc = (self.radius * self.radius - projection_sqr).sqrt();
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

pub struct Light {
    pub position: Vec3,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vec3, intensity: f32) -> Self {
        Self { position, intensity }
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }
}

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

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot_product(&self, vec: &Vec3) -> f32 {
        self.x * vec.x + self.y * vec.y  + self.z * vec.z
    }

    pub fn scale(&self, factor: f32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    pub fn divide(&self, factor: f32) -> Self {
        Self {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        }
    }

    pub fn minus(&self, vec: &Vec3) -> Self {
        Self {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z,
        }
    }

    pub fn plus(&self, vec: &Vec3) -> Self {
        Self {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).abs().sqrt()
    }

    pub fn normalize(&self) -> Self {
        self.divide(self.length())
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Vec3, Ray, Sphere};

    #[test]
    fn length() {
        let vec = Vec3::new(0., 3., 4.);
        let length = vec.length();

        assert_eq!(length, 5.);
    }

    #[test]
    fn normalize() {
        let vec = Vec3::new(0., 3., 4.);
        let normal = vec.normalize();

        let expected_x = 0.;
        let expected_y = 3. / 5.;
        let expected_z = 4. / 5.;

        assert_eq!(normal.x, expected_x);
        assert_eq!(normal.y, expected_y);
        assert_eq!(normal.z, expected_z);
    }

    #[test]
    fn plus() {
        let vec1 = Vec3::new(1., 1., 1.);
        let vec2 = Vec3::new(4., 4., 4.);
        let result = vec1.plus(&vec2);
        
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 5.0);
        assert_eq!(result.z, 5.0);
    }

    #[test]
    fn minus() {
        let vec1 = Vec3::new(1., 1., 1.);
        let vec2 = Vec3::new(4., 4., 4.);
        let result = vec1.minus(&vec2);
        
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, -3.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn dot_product() {
        let vec1 = Vec3::new(1., 2., 3.);
        let vec2 = Vec3::new(2., 3., 4.);
        let result = vec1.dot_product(&vec2);

        assert_eq!(result, 20.0);
    }

    #[test]
    fn scale() {
        let vec = Vec3::new(3., 2., 4.);
        let new_vec = vec.scale(2.);

        assert_eq!(new_vec.x, 6.);
        assert_eq!(new_vec.y, 4.);
        assert_eq!(new_vec.z, 8.);
    }

     #[test]
    fn divide() {
        let vec = Vec3::new(3., 2., 4.);
        let new_vec = vec.divide(2.);

        assert_eq!(new_vec.x, 1.5);
        assert_eq!(new_vec.y, 1.);
        assert_eq!(new_vec.z, 2.);
    }

    #[test]
    fn ray_intersect() {
        let orig = Vec3::new(0., 0., 0.);
        let dir = Vec3::new(4., 0., 0.).normalize();
        let mut ray = Ray::new(orig, dir, std::f32::MAX);

        let sphere = Sphere::new(Vec3::new(4., 1., 0.), 2.5, (24, 24, 24));

        assert!(sphere.ray_intersect(&mut ray));
    }
}