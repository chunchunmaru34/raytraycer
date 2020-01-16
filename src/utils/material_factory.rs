use crate::geometry::material::{Material};
use crate::utils::rgb::RGB;

pub fn get_mirror() -> Material {
  Material { 
    color: RGB::new(255, 255, 255),
    albedo: (0., 10., 0.8, 0.),
    specular_exponent: 1425.,
    refractive_index: 1.
  }
}

pub fn get_red_rubber() -> Material {
  Material {
    color: RGB::new(75, 25, 24),
    albedo: (0.9, 0.1, 0., 0.),
    specular_exponent: 10.,
    refractive_index: 1.
  }
}

pub fn get_dark_green_plastic() -> Material {
  Material {
    color: RGB::new(12, 55, 44),
    albedo: (0.6, 0.3, 0.1, 0.),
    specular_exponent: 50.,
    refractive_index: 1.
  }
}

pub fn get_glass() -> Material {
  Material {
    color: RGB::new(255, 255, 255),
    albedo: (0.0, 0.5, 0.1, 0.8),
    specular_exponent: 125.,
    refractive_index: 1.5
  }
}