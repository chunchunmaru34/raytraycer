use crate::geometry::vec3::Vec3;

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_vector(vec: &Vec3) -> Self {
        let r = if vec.x > 255. { 255 as u8 } else { vec.x as u8 };
        let g = if vec.y > 255. { 255 as u8 } else { vec.y as u8 };
        let b = if vec.z > 255. { 255 as u8 } else { vec.z as u8 };

        Self { r, g, b }
    }

    pub fn as_vector(&self) -> Vec3 {
        Vec3::new(self.r as f32, self.g as f32, self.b as f32)
    }

    pub fn as_array(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    pub fn clone(&self) -> Self {
        Self::new(self.r, self.g, self.b)
    }
}
