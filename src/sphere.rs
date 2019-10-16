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

pub struct Light {
    pub position: Point,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Point, intensity: f32) -> Self {
        Self { position, intensity }
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

    pub fn minus(&self, vec: &Vec3) -> Self {
        Self {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.y,
        }
    }

    pub fn plus(&self, vec: &Vec3) -> Self {
        Self {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.y,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let del = 1. / self.length();
        Self {
            x: self.x / del,
            y: self.y / del,
            z: self.z / del,
        }
    }
}