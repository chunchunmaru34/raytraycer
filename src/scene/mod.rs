use crate::geometry::sphere::{Sphere};
use crate::scene::light::{Light};
use crate::scene::camera::{Camera};

pub mod camera;
pub mod light;

pub struct Scene {
    pub objects: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}