use super::vec3::Vec3;

pub struct Material {
    pub color: Vec3,
    pub albedo: (f32, f32),
    pub specular_exponent: f32,
}
