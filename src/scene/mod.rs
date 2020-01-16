use self::camera::Camera;
use self::light::Light;
use crate::geometry::sphere::Sphere;
use crate::utils::rgb::RGB;

pub mod camera;
pub mod light;

pub struct Scene {
    pub objects: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub camera: Camera,
    pub canvas: Canvas,
    pub options: SceneOptions,
}

pub struct SceneOptions {
    pub reflections_limit: usize,
    pub background_color: RGB,
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub fov: f32,
}
