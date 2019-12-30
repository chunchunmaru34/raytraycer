use crate::geometry::material::{Material};
use crate::geometry::vec3::{Vec3};

pub fn get_mirror() -> Material {
  Material { 
    color: Vec3::new(255., 255., 255.),
    albedo: (0., 10., 0.8),
    specular_exponent: 1425.
  }
}

pub fn get_red_rubber() -> Material {
  Material {
    color: Vec3::new(75., 25., 24.),
    albedo: (0.9, 0.1, 0.),
    specular_exponent: 10.,
  }
}

pub fn get_dark_green_plastic() -> Material {
  Material {
    color: Vec3::new(12., 55., 44.),
    albedo: (0.6, 0.3, 0.1),
    specular_exponent: 50.,
  }
}