use crate::geometry::vec3::Vec3;

pub mod material_factory;
pub mod rgb;

pub fn move_from_surface(direction: &Vec3, normal: &Vec3, point: &Vec3) -> Vec3 {
    if direction.dot_product(normal) < 0. {
        point.minus(&normal.scale(1e-3))
    } else {
        point.plus(&normal.scale(1e-3))
    }
}