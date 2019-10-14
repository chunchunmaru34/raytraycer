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
    pub center: Point,
    pub radius: f32,
    pub color: (u8, u8, u8),
}

impl Sphere {
    pub fn new(center: Point, radius: f32, color: (u8, u8, u8)) -> Self {
        Self { center, radius, color }
    }

    pub fn ray_intersect(&self, orig: &Point, dir: &Point) -> bool {
        // orig - A
        // dir - B
        // center - C
        let ab = [(orig.x - dir.x).abs(), (orig.y - dir.y).abs(), (orig.z - dir.z).abs()];
        let ac = [(orig.x - self.center.x).abs(), (orig.y - self.center.y).abs(), (orig.z - self.center.z).abs()];
        let cb = [(dir.x - self.center.x).abs(), (dir.y - self.center.y).abs(), (dir.z - self.center.z).abs()];

        let ab_module = (ab[0].powi(2) + ab[1].powi(2) + ab[2].powi(2)).sqrt();
        let ac_module = (ac[0].powi(2) + ac[1].powi(2) + ac[2].powi(2)).sqrt();
        let cb_module = (cb[0].powi(2) + cb[1].powi(2) + cb[2].powi(2)).sqrt();
       
        let cos_alpha = (ac_module.powi(2) + ab_module.powi(2) - cb_module.powi(2)) / (2.0 * ab_module * ac_module);
        if cos_alpha < 0.0 {
            return false;
        }
        let projection = ac_module * cos_alpha;

        let distance = (ac_module.powi(2) - projection.powi(2)).abs().sqrt();
        
        return distance <= self.radius;
    }
}