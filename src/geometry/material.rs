use crate::utils::rgb::RGB;

pub struct Material {
    pub color: RGB,
    pub albedo: (f32, f32, f32, f32),
    pub specular_exponent: f32,
    pub refractive_index: f32,
}