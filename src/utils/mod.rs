use crate::geometry::vec3::{Vec3};

pub mod material_factory;

pub fn limit_color(color: Vec3) -> (u8, u8, u8) {
    let r = if color.x > 255. { 255 as u8 } else { color.x as u8 };
    let g = if color.y > 255. { 255 as u8 } else { color.y as u8 };
    let b = if color.z > 255. { 255 as u8 } else { color.z as u8 };

    (r, g ,b)
}