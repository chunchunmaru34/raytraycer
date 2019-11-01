use crate::geometry::{Sphere, Light};
use crate::camera::{Camera};

pub struct Scene {
    pub objects: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}